extern crate sdl_rust;

use std::time::Duration;
use std::thread;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;

use sdl_rust::SDLCore;
use sdl_rust::Demo;

const TITLE: &str = "SDL03 Drawing";
const CAM_W: u32 = 640;
const CAM_H: u32 = 480;
const TIMEOUT: u64 = 10000;

pub struct SDL03 {
	core: SDLCore,
}

impl Demo for SDL03 {
	fn init() -> Result<Self, String> {
		let core = SDLCore::init(TITLE, true, CAM_W, CAM_H)?;
		Ok(SDL03{ core })
	}

	fn run(&mut self) -> Result<(), String> {
		let g = Color::RGBA(0, 255, 0, 255);
		let b = Color::RGBA(0, 0, 255, 255);

		self.core.wincan.set_draw_color(Color::RGBA(0, 128, 128, 255));
		self.core.wincan.clear();

		self.core.wincan.set_draw_color(g);
		self.core.wincan.draw_point(Point::new(100, 100))?;
		self.core.wincan.draw_points(&[
			Point::new(75, 75),
			Point::new(75, 125),
			Point::new(125, 75),
			Point::new(125, 125),
		][..])?;
		
		self.core.wincan.draw_line(
			Point::new(500, 300),
			Point::new(400, 400),
		)?;

		self.core.wincan.set_draw_color(b);

		self.core.wincan.draw_lines(&[
			Point::new(150, 150),
			Point::new(200, 150),
			Point::new(200, 200),
			Point::new(375, 375),
			Point::new(375, 300),
		][..])?;

		self.core.wincan.set_draw_color(g);
		self.core.wincan.draw_rect(Rect::new(400, 10, 100, 100))?;

		// Outline overwritten by blue fill_rect() call
		self.core.wincan.draw_rect(Rect::new(400, 110, 100, 100))?;

		self.core.wincan.set_draw_color(b);
		self.core.wincan.fill_rect(Rect::new(400, 110, 100, 100))?;

		// Uncomment for red outline
		//self.core.wincan.set_draw_color(g);
		//self.core.wincan.draw_rect(Rect::new(400, 110, 100, 100))?;

		// I <3 Rust iterators
		let rs: Vec<_> = (0..5)
			.map(|i| i * 25)
			.map(|i| Rect::new(225 + i, 225 + i, 25, 25))
			.collect();

		// Up until now, should have been BlendMode::None
		self.core.wincan.set_blend_mode(BlendMode::Blend);

		self.core.wincan.set_draw_color(Color::RGBA(0, 255, 0, 128));
		self.core.wincan.fill_rects(&rs[..])?;

		self.core.wincan.present();
		thread::sleep(Duration::from_millis(TIMEOUT));

		Ok(())
	}
}

fn main() {
	sdl_rust::runner(TITLE, SDL03::init);
}
