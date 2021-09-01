extern crate sdl_rust;

use std::time::Duration;
use std::thread;

use sdl2::pixels::Color;

use sdl_rust::SDLCore;
use sdl_rust::Demo;

const TITLE: &str = "SDL02 Basic window";
const CAM_W: u32 = 1920;
const CAM_H: u32 = 1080;
const TIMEOUT: u64 = 5000;

pub struct SDL02 {
	core: SDLCore,
}

impl Demo for SDL02 {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(SDL02{ core })
	}

	fn run(&mut self) -> Result<(), String> {
		self.core.wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
		self.core.wincan.clear();
		self.core.wincan.present();
		thread::sleep(Duration::from_millis(TIMEOUT));

		Ok(())
	}
}

fn main() {
	sdl_rust::runner(TITLE, SDL02::init);
}
