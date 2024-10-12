use serde::Deserialize;
use std::net::Ipv4Addr;
use thiserror::Error;

#[derive(Deserialize, Debug)]
pub struct IpInfo {
    pub query: Option<String>,
    pub country: Option<String>,
    pub regionname: Option<String>,
    pub city: Option<String>,
    pub zip: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub isp: Option<String>,
    pub org: Option<String>,
    pub asn: Option<String>,
}

#[derive(Debug)]
pub struct PrivateIpInfo {
    pub ip: String,
    pub class: String,
    pub range: String,
    pub purpose: String,
}

pub enum IpInfoResult {
    Public(IpInfo),
    Private(PrivateIpInfo),
}

#[derive(Error, Debug)]
pub enum IpInfoError {
    #[error("HTTP error: {0}")]
    HttpError(String),
    #[error("Received non-200 response: {0}")]
    NonSuccessResponse(u16),
    #[error("Failed to parse JSON response: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Invalid IP address")]
    InvalidIpAddress,
}

impl From<surf::Error> for IpInfoError {
    fn from(error: surf::Error) -> Self {
        IpInfoError::HttpError(error.to_string())
    }
}

fn get_private_ip_info(ip: &str) -> Result<PrivateIpInfo, IpInfoError> {
    let ipv4: Ipv4Addr = ip.parse().map_err(|_| IpInfoError::InvalidIpAddress)?;

    let (class, range, purpose) = if ipv4.is_private() {
        if ipv4.octets()[0] == 10 {
            ("A", "10.0.0.0 to 10.255.255.255", "Private network")
        } else if ipv4.octets()[0] == 172 && (16..=31).contains(&ipv4.octets()[1]) {
            ("B", "172.16.0.0 to 172.31.255.255", "Private network")
        } else {
            ("C", "192.168.0.0 to 192.168.255.255", "Private network")
        }
    } else if ipv4.is_loopback() {
        ("A", "127.0.0.0 to 127.255.255.255", "Loopback")
    } else if ipv4.is_link_local() {
        ("B", "169.254.0.0 to 169.254.255.255", "Link-local")
    } else {
        ("Unknown", "Unknown", "Special purpose")
    };

    Ok(PrivateIpInfo {
        ip: ip.to_string(),
        class: class.to_string(),
        range: range.to_string(),
        purpose: purpose.to_string(),
    })
}
pub async fn fetch_ip_info(ip: &str) -> Result<IpInfoResult, IpInfoError> {
    if is_private_or_reserved_ip(ip) {
        get_private_ip_info(ip).map(IpInfoResult::Private)
    } else {
        let url = format!("http://ip-api.com/json/{}", ip);
        let mut response = surf::get(&url).await.map_err(IpInfoError::from)?;

        if response.status().is_success() {
            let info = response.body_json::<IpInfo>().await.map_err(IpInfoError::from)?;
            Ok(IpInfoResult::Public(info))
        } else {
            Err(IpInfoError::NonSuccessResponse(response.status().into()))
        }
    }
}

fn is_private_or_reserved_ip(ip: &str) -> bool {
    ip.parse::<Ipv4Addr>()
        .map(|ipv4| ipv4.is_private() || ipv4.is_loopback() || ipv4.is_link_local() || ipv4.is_broadcast())
        .unwrap_or(false)
}