use dotenv;
use std::env;

fn main() {
	dotenv::dotenv().ok();
	let username = env::var("HUE_USERNAME").unwrap_or("host#user".to_string());

	println!("Registering: {}", username);
}
