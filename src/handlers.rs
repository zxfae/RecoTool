use crate::ip_info::fetch_ip_info;
use colored::*;

pub fn handle_help() {
    println!("{}", "Displaying help message".green().bold());
}

pub fn handle_full_name(full_name: &str) {
    println!("Searching with full name: {}", full_name);
}

pub async fn handle_ip(ip: &str) {
    println!("Starting handle_ip with IP: {}\n", ip);
    if let Some(ip_info) = fetch_ip_info(ip).await {
        if let Some(query) = ip_info.query {
            println!("IP Address: {}", query);
        }
        if let Some(country) = ip_info.country {
            println!("Country: {}", country);
        }
        if let Some(region) = ip_info.regionname {
            println!("Region: {}", region);
        }
        if let Some(city) = ip_info.city {
            println!("City: {}", city);
        }
        if let Some(zip) = ip_info.zip {
            println!("Postal Code: {}", zip);
        }
        if let (Some(lat), Some(lon)) = (ip_info.lat, ip_info.lon) {
            println!("Location (lat, long): {}, {}", lat, lon);
        }
        if let Some(isp) = ip_info.isp {
            println!("ISP: {}", isp);
        }
        if let Some(org) = ip_info.org {
            println!("Organization: {}", org);
        }
        if let Some(asn) = ip_info.asn {
            println!("ASN: {}", asn);
        }
    }
}

pub fn handle_username(username: &str) {
    println!("Searching with username: {}", username);
}
