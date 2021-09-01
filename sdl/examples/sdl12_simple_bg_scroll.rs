extern crate sdl_rust;

use std::collections::HashSet;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::render::Texture;

use sdl_rust::SDLCore;
use sdl_rust::Demo;

const TITLE: &str = "SDL12 Simple BG scrolling";

const CAM_W: u32 = 640;
const CAM_H: u32 = 480;

const TILE_SIZE: u32 = 100;

const SPEED_LIMIT: i32 = 5;
const ACCEL_RATE: i32 = 1;

enum PlayerType {
	Bird,
	Plane,
	UFO,
	Chopper,
}

struct Player<'a> {
	pos: Rect,
	src: Rect,
	texture: Texture<'a>,
}

impl<'a> Player<'a> {
	fn new(t: PlayerType, pos: Rect, texture: Texture<'a>) -> Player {
		let (x, y) = match t {
			PlayerType::Bird => (0, 0),
			PlayerType::Plane => (TILE_SIZE, 0),
			PlayerType::UFO => (0, TILE_SIZE),
			PlayerType::Chopper => (TILE_SIZE, TILE_SIZE),
		};

		let src = Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE);
		Player {
			pos,
			src,
			texture,
		}
	}

	fn pos(&self) -> Rect {
		self.pos
	}

	fn update_pos(&mut self, vel: (i32, i32), x_bounds: (i32, i32), y_bounds: (i32, i32)) {
		self.pos.set_x((self.pos.x() + vel.0).clamp(x_bounds.0, x_bounds.1));
		self.pos.set_y((self.pos.y() + vel.1).clamp(y_bounds.0, y_bounds.1));
	}

	fn src(&self) -> Rect {
		self.src
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

pub struct SDL12 {
	core: SDLCore,
}

impl Demo for SDL12 {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(SDL12{ core })
	}

	fn run(&mut self) -> Result<(), String> {
		let texture_creator = self.core.wincan.texture_creator();

		// BG is the same size and window, but will scroll to simulate
		// consistent horizontal movement
		let bg = texture_creator.load_texture("images/small_bg.png")?;

		// Will indicate the offset to use in rendering bg image
		let mut bg_offset = 0;

		let mut p = Player::new(
			PlayerType::Plane,
			Rect::new(
				(CAM_W/2 - TILE_SIZE/2) as i32,
				(CAM_H/2 - TILE_SIZE/2) as i32,
				TILE_SIZE,
				TILE_SIZE,
			),
			texture_creator.load_texture("images/birds.png")?,
		);

		let mut x_vel = 0;
		let mut y_vel = 0;

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
			let mut y_deltav = 0;
			if keystate.contains(&Keycode::W) {
				y_deltav -= ACCEL_RATE;
			}
			if keystate.contains(&Keycode::A) {
				x_deltav -= ACCEL_RATE;
			}
			if keystate.contains(&Keycode::S) {
				y_deltav += ACCEL_RATE;
			}
			if keystate.contains(&Keycode::D) {
				x_deltav += ACCEL_RATE;
			}
			x_deltav = resist(x_vel, x_deltav);
			y_deltav = resist(y_vel, y_deltav);
			x_vel = (x_vel + x_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);
			y_vel = (y_vel + y_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);

			// Back to moving the player only within the camera window
			p.update_pos((x_vel, y_vel), (0, (CAM_W - TILE_SIZE) as i32), (0, (CAM_H - TILE_SIZE) as i32));

			bg_offset -= 1;
			if bg_offset < -(CAM_W as i32) {
				bg_offset = -1;
			}

			self.core.wincan.set_draw_color(Color::BLACK);
			self.core.wincan.clear();

			// Draw background
			self.core.wincan.copy(&bg, None, Rect::new(bg_offset, 0, CAM_W, CAM_H))?;
			self.core.wincan.copy(&bg, None, Rect::new(bg_offset + (CAM_W as i32), 0, CAM_W, CAM_H))?;

			// Draw player
			self.core.wincan.copy(p.texture(), p.src(), p.pos())?;

			self.core.wincan.present();
		}

		// Out of game loop, return Ok
		Ok(())
	}
}

fn main() {
	sdl_rust::runner(TITLE, SDL12::init);
}
