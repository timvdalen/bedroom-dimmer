use dotenv;
use std::env;
use philipshue::Bridge;
use bedroom_dimmer::dimming::dim;
use philipshue::errors::HueError;

fn main() -> Result<(), HueError> {
	dotenv::dotenv().ok();
	let ip = env::var("HUE_BRIDGE_IP").expect("No IP address given with HUE_BRIDGE_IP");
	let username = env::var("HUE_USERNAME").expect("No username given with HUE_USERNAME");
	let first_lamp = env::var("FIRST_LAMP").expect("No first lamp ID given with FIRST_LAMP").parse::<usize>().expect("FIRST_LAMP is not numeric");
	let second_lamp = env::var("SECOND_LAMP").expect("No second lamp ID given with SECOND_LAMP").parse::<usize>().expect("SECOND_LAMP is not numeric");

	let bridge = Bridge::new(&ip, &username);
	dim(bridge, first_lamp, second_lamp)
}
