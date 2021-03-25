use std::error::Error;

use sequin::{
	easing::{Out, Powi},
	tween,
};
use tetra::{
	graphics::{
		self,
		mesh::{Mesh, ShapeStyle},
		Color,
	},
	input::Key,
	math::Vec2,
	time::get_delta_time,
	Context, ContextBuilder, Event, State,
};

struct MainState {
	animation_time: f32,
}

impl MainState {
	pub fn new() -> Self {
		Self {
			animation_time: 0.0,
		}
	}
}

impl State<Box<dyn Error>> for MainState {
	fn event(&mut self, _ctx: &mut Context, event: Event) -> Result<(), Box<dyn Error>> {
		if let Event::KeyPressed { key: Key::Space } = event {
			self.animation_time = 0.0;
		}
		Ok(())
	}

	fn update(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.animation_time += get_delta_time(ctx).as_secs_f32();
		self.animation_time = self.animation_time.min(1.0);
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		graphics::clear(ctx, Color::BLACK);
		Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), 64.0)?.draw(
			ctx,
			tween(
				Vec2::new(200.0, 200.0)..Vec2::new(600.0, 400.0),
				self.animation_time,
				Out(Powi(2)),
			),
		);
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	ContextBuilder::new("sequin-test", 800, 600)
		.build()?
		.run(|_| Ok(MainState::new()))
}
