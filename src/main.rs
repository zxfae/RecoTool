mod args;
mod ip_info;
mod handlers;

use async_std::task;
use crate::args::parse_args;
use crate::handlers::{handle_full_name, handle_help, handle_ip, handle_username};

fn main() {
    let args = parse_args();

    if let Some(full_name) = args.full_name {
        handle_full_name(&full_name);
    } else if let Some(ip) = args.ip {
        task::block_on(handle_ip(&ip));
    } else if let Some(username) = args.username {
        handle_username(&username);
    } else {
        handle_help();
    }
}
