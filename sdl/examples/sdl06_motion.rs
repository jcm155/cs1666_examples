extern crate sdl_rust;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl_rust::SDLCore;
use sdl_rust::Demo;

const TITLE: &str = "SDL06 Key Events";
const CAM_W: u32 = 640;
const CAM_H: u32 = 480;

pub struct SDL06 {
	core: SDLCore,
}

impl Demo for SDL06 {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(SDL06{ core })
	}

	fn run(&mut self) -> Result<(), String> {
		let w = 25;
		let mut x_pos = (CAM_W/2 - w/2) as i32;
		let mut y_pos = (CAM_H/2 - w/2) as i32;

		let mut x_vel = 0;
		let mut y_vel = 0;

		'gameloop: loop {
			for event in self.core.event_pump.poll_iter() {
				match event {
					Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
					Event::KeyDown{keycode: Some(k), ..} => {
						match k {
							Keycode::W => y_vel -= 1,
							Keycode::A => x_vel -= 1,
							Keycode::S => y_vel += 1, 
							Keycode::D => x_vel += 1,
							_ => {},
						}
					}
					_ => {},
				}
			}

			x_pos += x_vel;
			y_pos += y_vel;

			self.core.wincan.set_draw_color(Color::BLACK);
			self.core.wincan.clear();

			self.core.wincan.set_draw_color(Color::CYAN);
			self.core.wincan.fill_rect(Rect::new(x_pos, y_pos, w, w))?;

			self.core.wincan.present();
		}

		// Out of game loop, return Ok
		Ok(())
	}
}

fn main() {
	sdl_rust::runner(TITLE, SDL06::init);
}
