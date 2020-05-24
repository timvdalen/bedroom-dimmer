use dotenv;
use std::env;
use philipshue::Bridge;
use bedroom_dimmer::dimming::dim;
use tiny_http::Server;
use tiny_http::Response;

fn main() {
	dotenv::dotenv().ok();
	let ip = env::var("HUE_BRIDGE_IP").expect("No IP address given with HUE_BRIDGE_IP");
	let username = env::var("HUE_USERNAME").expect("No username given with HUE_USERNAME");
	let first_lamp = env::var("FIRST_LAMP").expect("No first lamp ID given with FIRST_LAMP").parse::<usize>().expect("FIRST_LAMP is not numeric");
	let second_lamp = env::var("SECOND_LAMP").expect("No second lamp ID given with SECOND_LAMP").parse::<usize>().expect("SECOND_LAMP is not numeric");

	let host = env::var("HTTP_HOST").expect("No HTTP_HOST given");
	let port = env::var("HTTP_PORT").expect("No HTTP_PORT given");

	let server = Server::http(format!("{}:{}", host, port)).expect("HTTP server creation failed");

	loop {
		let request = match server.recv() {
			Ok(rq) => rq,
			Err(e) => { println!("error: {}", e); break }
		};

		let bridge = Bridge::new(&ip, &username);
		match dim(bridge, first_lamp, second_lamp) {
			Ok(()) => {},
			Err(e) => { println!("Dimming error: {}", e) }
		}

		let _ = request.respond(Response::from_string("OK"));
	}
}
