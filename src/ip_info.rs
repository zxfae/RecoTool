use serde::Deserialize;
use colored::*;
use surf;

#[derive(Deserialize)]
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

pub async fn fetch_ip_info(ip: &str) -> Option<IpInfo> {
    let url = format!("http://ip-api.com/json/{}", ip);
    match surf::get(&url).await {
        Ok(mut response) => {
            if response.status().is_success() {
                match response.body_json::<IpInfo>().await {
                    Ok(data) => Some(data),
                    Err(_) => {
                        println!("{}", "Error: Failed to parse JSON response".red());
                        None
                    }
                }
            } else {
                println!("{}", "Error: received non-200 response".red());
                None
            }
        }
        Err(_) => {
            println!("{}", "Error: Failed to fetch IP info".red());
            None
        }
    }
}
