extern crate sdl_rust;

use std::collections::HashSet;
use std::time::{Instant, Duration};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::render::Texture;

use sdl_rust::SDLCore;
use sdl_rust::Demo;

const TITLE: &str = "SDL17 Calculate FPS";
// Repeat of SDL15 Animation with FPS calculation

const CAM_W: u32 = 640;
const CAM_H: u32 = 480;

const LEVEL_LEN: u32 = 2000;

const TILE_SIZE: u32 = 100;

const LTHIRD: i32 = ((CAM_W as i32) / 3) - (TILE_SIZE as i32)/2;
const RTHIRD: i32 = ((CAM_W as i32) * 2 / 3) - (TILE_SIZE as i32)/2;

const SPEED_LIMIT: i32 = 5;
const ACCEL_RATE: i32 = 1;

struct Player<'a> {
	pos: Rect,
	texture: Texture<'a>,
}

impl<'a> Player<'a> {
	fn new(pos: Rect, texture: Texture<'a>) -> Player {
		Player {
			pos,
			texture,
		}
	}

	fn x(&self) -> i32 {
		self.pos.x()
	}

	fn y(&self) -> i32 {
		self.pos.y()
	}

	fn update_pos(&mut self, vel: i32, x_bounds: (i32, i32)) {
		self.pos.set_x((self.pos.x() + vel).clamp(x_bounds.0, x_bounds.1));
	}

	fn texture(&self) -> &Texture {
		&self.texture
	}
}

fn resist(vel: i32, deltav: i32) -> i32 {
	if deltav == 0 {
		if vel > 0 {
			-1
		}
		else if vel < 0 {
			1
		}
		else {
			deltav
		}
	}
	else {
		deltav
	}
}

pub struct SDL17 {
	core: SDLCore,
}

impl Demo for SDL17 {
	fn init() -> Result<Self, String> {
		// With vsync:
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		// Without vsync:
		//let core = SDLCore::init(TITLE, false, CAM_W, CAM_H)?;
		Ok(SDL17{ core })
	}

	fn run(&mut self) -> Result<(), String> {
		let texture_creator = self.core.wincan.texture_creator();

		let bg = texture_creator.load_texture("images/bg.png")?;
		let mut scroll_offset = 0;

		let brick_sheet = texture_creator.load_texture("images/bricks.png")?;		

		let mut p = Player::new(
			Rect::new(
				TILE_SIZE as i32,
				(CAM_H - TILE_SIZE*2) as i32,
				TILE_SIZE,
				TILE_SIZE,
			),
			texture_creator.load_texture("images/walking.png")?,
		);

		let mut frames = 0;
		let mut src_x = 0;

		let mut flip = false;
		let mut x_vel = 0;

		// FPS tracking
		let mut all_frames = 0;
		let mut last_time = Instant::now();
		println!("\n\n");

		'gameloop: loop {
			for event in self.core.event_pump.poll_iter() {
				match event {
					Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
					_ => {},
				}
			}

			let keystate: HashSet<Keycode> = self.core.event_pump
				.keyboard_state()
				.pressed_scancodes()
				.filter_map(Keycode::from_scancode)
				.collect();

			let mut x_deltav = 0;
			if keystate.contains(&Keycode::A) {
				x_deltav -= ACCEL_RATE;
			}
			if keystate.contains(&Keycode::D) {
				x_deltav += ACCEL_RATE;
			}
			x_deltav = resist(x_vel, x_deltav);
			x_vel = (x_vel + x_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);

			p.update_pos(x_vel, (0, (LEVEL_LEN - TILE_SIZE) as i32));

			scroll_offset = if p.x() > scroll_offset + RTHIRD {
				(p.x() - RTHIRD).clamp(0, (LEVEL_LEN - CAM_W) as i32)
			}
			else if p.x() < scroll_offset + LTHIRD {
				(p.x() - LTHIRD).clamp(0, (LEVEL_LEN - CAM_W) as i32)
			}
			else {
				scroll_offset
			};
			
			let bg_offset = -(scroll_offset % (CAM_W as i32));
			let mut brick_offset = -(scroll_offset % (TILE_SIZE as i32));

			flip = if x_vel > 0 && flip {
				false
			}
			else if x_vel < 0 && !flip {
				true
			}
			else {
				flip
			};

			src_x = if x_vel != 0 {
				frames = if (frames + 1) / 6 > 3 {
					0
				}
				else {
					frames + 1
				};

				(frames / 6) * 100
			}
			else {
				src_x
			};
			
			self.core.wincan.set_draw_color(Color::BLACK);
			self.core.wincan.clear();

			// Draw background
			self.core.wincan.copy(&bg, None, Rect::new(bg_offset, 0, CAM_W, CAM_H))?;
			self.core.wincan.copy(&bg, None, Rect::new(bg_offset + (CAM_W as i32), 0, CAM_W, CAM_H))?;

			// Draw bricks
			let mut i = (scroll_offset % ((TILE_SIZE as i32) * 4)) / (TILE_SIZE as i32);
			while brick_offset < (CAM_W as i32) {
				let src = Rect::new((i % 4) * (TILE_SIZE as i32), 0, TILE_SIZE, TILE_SIZE);
				let pos = Rect::new(brick_offset, (CAM_H - TILE_SIZE) as i32, TILE_SIZE, TILE_SIZE);

				self.core.wincan.copy(&brick_sheet, src, pos)?;

				i += 1;
				brick_offset += TILE_SIZE as i32;
			}

			// Draw player
			self.core.wincan.copy_ex(
				p.texture(),
				Rect::new(src_x, 0, TILE_SIZE, TILE_SIZE),
				Rect::new(p.x() - scroll_offset, p.y(), TILE_SIZE, TILE_SIZE),
				0.0,
				None,
				flip,
				false,
			)?;

			self.core.wincan.present();

			// FPS Calculation
			all_frames += 1;
			let elapsed = last_time.elapsed();
			if elapsed > Duration::from_secs(5) {
				let fps_avg = (all_frames as f64) / elapsed.as_secs_f64();
				println!("Average FPS: {:.2}", fps_avg);

				all_frames = 0;
				last_time = Instant::now();
			}
		}

		// Out of game loop, return Ok
		Ok(())
	}
}

fn main() {
	sdl_rust::runner(TITLE, SDL17::init);
}
