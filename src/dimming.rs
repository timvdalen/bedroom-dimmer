use philipshue::{Bridge, LightCommand};
use std::thread::sleep;
use std::time::Duration;
use philipshue::errors::{HueError};

fn dim_lamp(bridge: &Bridge, lamp: usize) -> Result<(), HueError> {
	let light = bridge.get_light(lamp)?;
	if !light.state.on {
		return Ok(())
	}

	let mut bri = light.state.bri;
	while bri > 1 {
		bri -= 1;
		let cmd = LightCommand::default().with_bri(bri);
		bridge.set_light_state(lamp, &cmd)?;
		sleep(Duration::from_millis(100));
	}
	let off_cmd = LightCommand::default().off();
	bridge.set_light_state(lamp, &off_cmd)?;

	Ok(())
}

pub fn dim(bridge: Bridge, first_lamp: usize, second_lamp: usize) -> Result<(), HueError> {
	dim_lamp(&bridge, first_lamp)?;
	dim_lamp(&bridge, second_lamp)?;

	Ok(())
}
