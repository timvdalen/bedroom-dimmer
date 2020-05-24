use std::env;
use std::thread;
use std::time::Duration;
use dotenv;
use philipshue::bridge;
use philipshue::errors::{HueError, HueErrorKind, BridgeError};

fn main() {
	dotenv::dotenv().ok();
	let device_type = env::var("HUE_DEVICETYPE").unwrap_or("host#user".to_string());
	let ip = env::var("HUE_BRIDGE_IP").expect("No IP address given with HUE_BRIDGE_IP");

	loop {
		match bridge::register_user(&ip, &device_type) {
			Ok(bridge) => {
				println!("User registered: {}, on IP: {}", bridge, ip);
				break;
			}
			Err(HueError(HueErrorKind::BridgeError { error: BridgeError::LinkButtonNotPressed, .. }, _)) => {
				println!("Please, press the link on the bridge. Retrying in 5 seconds");
				thread::sleep(Duration::from_secs(5));
			}
			Err(e) => {
				println!("Unexpected error occured: {}", e);
				return;
			}
		}
	}
}
