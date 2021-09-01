extern crate sdl_rust;

use std::collections::HashSet;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl_rust::SDLCore;
use sdl_rust::Demo;

const TITLE: &str = "SDL09 Circular Collisions";
const CAM_W: u32 = 640;
const CAM_H: u32 = 480;
const SPEED_LIMIT: i32 = 5;
const ACCEL_RATE: i32 = 1;

struct NOTSDL_Circ {
	x: i32,
	y: i32,
	r: u32,
}

impl NOTSDL_Circ {
	fn new(x: i32, y: i32, r: u32) -> NOTSDL_Circ{
		NOTSDL_Circ {
			x,
			y,
			r,
		}
	}

	fn x(&self) -> i32 {
		self.x
	}

	fn y(&self) -> i32 {
		self.y
	}

	fn r(&self) -> u32 {
		self.r
	}

	fn set_x(&mut self, x: i32) {
		self.x = x;
	}

	fn set_y(&mut self, y: i32) {
		self.y = y;
	}

	fn left(&self) -> i32 {
		self.x - (self.r as i32)
	}

	fn right(&self) -> i32 {
		self.x + (self.r as i32)
	}

	fn top(&self) -> i32 {
		self.y - (self.r as i32)
	}

	fn bottom(&self) -> i32 {
		self.y + (self.r as i32)
	}

	fn get_points(&self) -> Vec<Point> {
		let rad = self.r() as i32;

		// I really <3 Rust iterators...
		(-rad..rad)
			.flat_map(|i| {
				(-rad..rad).map(move |j| { (i, j) })
			})
			.filter_map(|t| {
				if (t.0.pow(2) + t.1.pow(2)) < rad.pow(2) {
					Some(Point::new(self.x + t.0, self.y + t.1))
				}
				else {
					None
				}
			})
			.collect()
	}
}

fn check_collision(a: &NOTSDL_Circ, b: &NOTSDL_Circ) -> bool {
	let radsum = (a.r() + b.r()) as i32;
	let distsq = (a.x() - b.x()).pow(2) + (a.y() - b.y()).pow(2);

	distsq < radsum.pow(2)
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

pub struct SDL09 {
	core: SDLCore,
}

impl Demo for SDL09 {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(SDL09{ core })
	}

	fn run(&mut self) -> Result<(), String> {
		let r = 12;

		let static_dot = NOTSDL_Circ::new((CAM_W/2 + 2*r) as i32, (CAM_H/2) as i32, r);
		let mut player_dot = NOTSDL_Circ::new((CAM_W/2) as i32, (CAM_H/2) as i32, r);

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

			// Slow down to 0 vel if no input and non-zero velocity
			x_deltav = resist(x_vel, x_deltav);
			y_deltav = resist(y_vel, y_deltav);

			// Don't exceed speed limit
			x_vel = (x_vel + x_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);
			y_vel = (y_vel + y_deltav).clamp(-SPEED_LIMIT, SPEED_LIMIT);

			// Try to move horizontally
			player_dot.set_x(player_dot.x() + x_vel);
			// Use the "go-back" approach to collision resolution
			if check_collision(&player_dot, &static_dot)
				|| player_dot.left() < 0
				|| player_dot.right() > CAM_W as i32
			{
				player_dot.set_x(player_dot.x() - x_vel);
			}

			// Try to move vertically
			player_dot.set_y(player_dot.y() + y_vel);
			if check_collision(&player_dot, &static_dot)
				|| player_dot.top() < 0
				|| player_dot.bottom() > CAM_H as i32
			{
				player_dot.set_y(player_dot.y() - y_vel);
			}	

			self.core.wincan.set_draw_color(Color::BLACK);
			self.core.wincan.clear();

			self.core.wincan.set_draw_color(Color::RED);
			self.core.wincan.draw_points(&(static_dot.get_points())[..])?;

			self.core.wincan.set_draw_color(Color::CYAN);
			self.core.wincan.draw_points(&(player_dot.get_points())[..])?;

			self.core.wincan.present();
		}

		// Out of game loop, return Ok
		Ok(())
	}
}

fn main() {
	sdl_rust::runner(TITLE, SDL09::init);
}
