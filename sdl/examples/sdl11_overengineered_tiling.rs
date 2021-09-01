extern crate sdl_rust;
extern crate rand;

use rand::thread_rng;
use rand::Rng;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::render::Texture;

use sdl_rust::SDLCore;
use sdl_rust::Demo;

const TITLE: &str = "SDL11 Tiling";

const CAM_W: u32 = 640;
const CAM_H: u32 = 480;

const TILE_SIZE: u32 = 100;

struct Bird<'a> {
	num: i32,
	pos: Rect,
	texture: &'a Texture<'a>,
}

impl<'a> Bird<'a> {
	fn new(num: i32, pos: Rect, texture: &'a Texture) -> Bird<'a> {
		Bird {
			num,
			pos,
			texture,
		}
	}
}

struct Brick<'a> {
	num: i32,
	pos: Rect,
	texture: &'a Texture<'a>,
}

impl<'a> Brick<'a> {
	fn new(num: i32, pos: Rect, texture: &'a Texture) -> Brick<'a> {
		Brick {
			num,
			pos,
			texture,
		}
	}
}

trait Drawable {
	fn src(&self) -> Rect;
	fn pos(&self) -> Rect;
	fn texture(&self) -> &Texture;
}

impl<'a> Drawable for Bird<'a> {
	fn src(&self) -> Rect {
		let x = (self.num % 2) * (TILE_SIZE as i32);
		let y = ((self.num % 4) / 2) * (TILE_SIZE as i32);
		Rect::new(x, y, TILE_SIZE, TILE_SIZE)
	}

	fn pos(&self) -> Rect {
		self.pos
	}

	fn texture(&self) -> &'a Texture {
		self.texture
	}
}

impl<'a> Drawable for Brick<'a> {
	fn src(&self) -> Rect {
		let x = (self.num % 4) * (TILE_SIZE as i32);
		Rect::new(x, 0, TILE_SIZE, TILE_SIZE)
	}

	fn pos(&self) -> Rect {
		self.pos
	}

	fn texture(&self) -> &'a Texture {
		self.texture
	}
}

pub struct SDL11 {
	core: SDLCore,
}

impl Demo for SDL11 {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(SDL11{ core })
	}

	fn run(&mut self) -> Result<(), String> {
		let texture_creator = self.core.wincan.texture_creator();

		let bird_sheet = texture_creator.load_texture("images/birds.png")?;
		let brick_sheet = texture_creator.load_texture("images/bricks.png")?;

		let mut rng = thread_rng();
		let birds: Vec<_> = (0..8)
			.map(|i| {
				Bird::new(
					i,
					Rect::new(
						rng.gen_range(0..((CAM_W-TILE_SIZE) as i32)),
						rng.gen_range(0..((CAM_H-(2*TILE_SIZE)) as i32)),
						TILE_SIZE,
						TILE_SIZE,
					),
					&bird_sheet,
				)
			})
			.collect();

		let bricks: Vec<_> = (0..=(CAM_W/TILE_SIZE))
			.map(|i| {
				Brick::new(
					i as i32,
					Rect::new(
						(i * TILE_SIZE) as i32,
						(CAM_H - TILE_SIZE) as i32,
						TILE_SIZE,
						TILE_SIZE,
					),
					&brick_sheet,
				)
			})
			.collect();

		'gameloop: loop {
			for event in self.core.event_pump.poll_iter() {
				match event {
					Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..} => break 'gameloop,
					_ => {},
				}
			}

			let drawable_iter = bricks.iter()
				// Create trait object via cast
				.map(|b| { b as &dyn Drawable })
				.chain(birds.iter()
					// Create trait object via coercion
					.map(|b| { let d: &dyn Drawable = b; d})
				);

			self.core.wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
			self.core.wincan.clear();

			for d in drawable_iter {
				self.core.wincan.copy(d.texture(), d.src(), d.pos())?;
			}

			self.core.wincan.present();
		}

		// Out of game loop, return Ok
		Ok(())
	}
}

fn main() {
	sdl_rust::runner(TITLE, SDL11::init);
}
