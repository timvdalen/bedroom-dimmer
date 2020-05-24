use dotenv;
use std::env;
use philipshue::Bridge;

fn main() {
	dotenv::dotenv().ok();
	let ip = env::var("HUE_BRIDGE_IP").expect("No IP address given with HUE_BRIDGE_IP");
	let username = env::var("HUE_USERNAME").expect("No username given with HUE_USERNAME");
	let first_lamp = env::var("FIRST_LAMP").expect("No first lamp ID given with FIRST_LAMP").parse::<usize>().expect("FIRST_LAMP is not numeric");

	let bridge = Bridge::new(&ip, &username);
	match bridge.get_light(first_lamp) {
		Ok(light) => {
			println!("{:?}", light);
		},
		Err(_) => {},
	}
}
