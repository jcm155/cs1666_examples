extern crate sdl_rust;

use std::time::Instant;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;

use sdl_rust::SDLCore;
use sdl_rust::Demo;

const TITLE: &str = "SDL16 Event Handling";
const CAM_W: u32 = 640;
const CAM_H: u32 = 480;
// No timeout needed!

pub struct SDL16 {
	core: SDLCore,
}

impl Demo for SDL16 {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(SDL16{ core })
	}

	fn run(&mut self) -> Result<(), String> {
		let texture_creator = self.core.wincan.texture_creator();
		let bg = texture_creator.load_texture("images/nightmare_fuel.png")?;

		// We'll use the Instant::elapsed method to get an std::time::Durtion
		let start = Instant::now();

		'gameloop: loop {
			for event in self.core.event_pump.poll_iter() {
				match event {
					Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
					_ => {},
				}
			}

			self.core.wincan.set_draw_color(Color::BLACK);
			self.core.wincan.clear();
			self.core.wincan.copy(&bg, None, None)?;
			self.core.wincan.present();
		}

		// Out of game loop, print time and return Ok
		let t = start.elapsed().as_millis();
		println!("\n\nYou managed to stare at the image for {}ms!", t);

		Ok(())
	}
}

fn main() {
	sdl_rust::runner(TITLE, SDL16::init);
}
