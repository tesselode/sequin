use std::error::Error;

use sequin::{
	easing::{Out, Powi},
	sequence::Sequence,
};
use tetra::{
	graphics::{
		self,
		mesh::{Mesh, ShapeStyle},
		Color,
	},
	math::Vec2,
	time::get_delta_time,
	Context, ContextBuilder, Event, State,
};

struct MainState {
	sequence: Sequence<Vec2<f32>>,
}

impl MainState {
	pub fn new() -> Self {
		Self {
			sequence: Sequence::new(Vec2::new(200.0, 200.0))
				.tween(1.0, Vec2::new(600.0, 400.0), Powi(2))
				.wait(0.5)
				.tween(2.0, Vec2::new(200.0, 400.0), Out(Powi(2))),
		}
	}
}

impl State<Box<dyn Error>> for MainState {
	fn event(&mut self, _ctx: &mut Context, _event: Event) -> Result<(), Box<dyn Error>> {
		Ok(())
	}

	fn update(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		self.sequence.update(get_delta_time(ctx).as_secs_f32());
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
		graphics::clear(ctx, Color::BLACK);
		Mesh::circle(ctx, ShapeStyle::Fill, Vec2::zero(), 64.0)?.draw(ctx, self.sequence.current());
		println!(
			"{:?}, {}",
			self.sequence.current(),
			self.sequence.finished()
		);
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	ContextBuilder::new("sequin-test", 800, 600)
		.build()?
		.run(|_| Ok(MainState::new()))
}
