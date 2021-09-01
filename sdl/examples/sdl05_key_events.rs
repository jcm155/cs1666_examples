extern crate sdl_rust;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl_rust::SDLCore;
use sdl_rust::Demo;

const TITLE: &str = "SDL05 Key Events";
const CAM_W: u32 = 640;
const CAM_H: u32 = 480;

pub struct SDL05 {
	core: SDLCore,
}

impl Demo for SDL05 {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(SDL05{ core })
	}

	fn run(&mut self) -> Result<(), String> {
		let bg_color = Color::RGBA(0, 128, 128, 255);
		let mut draw_color = Color::CYAN;
		
		let w = 100;
		let r = Rect::new((CAM_W/2 - w/2) as i32, (CAM_H /2 - w/2) as i32, w, w);

		'gameloop: loop {
			for event in self.core.event_pump.poll_iter() {
				match event {
					Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
					Event::KeyDown{keycode: Some(k), ..} => {
						match k {
							Keycode::W => draw_color = Color::RED,
							Keycode::A => draw_color = Color::GREEN,
							Keycode::S => draw_color = Color::BLUE, 
							Keycode::D => draw_color = Color::MAGENTA,
							_ => {},
						}
					}
					_ => {},
				}
			}

			self.core.wincan.set_draw_color(bg_color);
			self.core.wincan.clear();

			self.core.wincan.set_draw_color(draw_color);
			self.core.wincan.fill_rect(r)?;

			self.core.wincan.present();
		}

		// Out of game loop, return Ok
		Ok(())
	}
}

fn main() {
	sdl_rust::runner(TITLE, SDL05::init);
}
