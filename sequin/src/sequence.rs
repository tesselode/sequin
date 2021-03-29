use std::ops::Range;

use crate::{
	easing::{Easing, Linear},
	tweenable::Tweenable,
};

fn tween<T: Tweenable, E: Easing + ?Sized>(values: &Range<T>, progress: f32, easing: &Box<E>) -> T {
	values.start + (values.end - values.start) * easing.ease(progress)
}

#[derive(Debug)]
pub struct Stage<T: Tweenable> {
	pub duration: f32,
	pub values: Range<T>,
	pub easing: Box<dyn Easing>,
}

#[derive(Debug, Clone, Copy)]
enum State {
	Running { stage_index: usize, time: f32 },
	Finished,
}

pub struct Sequence<T: Tweenable> {
	start: T,
	stages: Vec<Stage<T>>,
	state: State,
	current_value: T,
}

impl<T: Tweenable> Sequence<T> {
	pub fn new(start: T) -> Self {
		Self {
			start,
			stages: vec![],
			state: State::Running {
				stage_index: 0,
				time: 0.0,
			},
			current_value: start,
		}
	}

	pub fn single(duration: f32, values: Range<T>, easing: impl Easing + 'static) -> Self {
		Self::new(values.start).tween(duration, values.end, easing)
	}

	pub fn tween(mut self, duration: f32, target: T, easing: impl Easing + 'static) -> Self {
		let start = self
			.stages
			.last()
			.map_or(self.start, |stage| stage.values.end);
		self.stages.push(Stage {
			duration,
			values: start..target,
			easing: Box::new(easing),
		});
		self
	}

	pub fn wait(mut self, duration: f32) -> Self {
		let start = self
			.stages
			.last()
			.map_or(self.start, |stage| stage.values.end);
		self.stages.push(Stage {
			duration,
			values: start..start,
			easing: Box::new(Linear),
		});
		self
	}

	pub fn update(&mut self, delta_time: f32) {
		if let State::Running { stage_index, time } = &mut self.state {
			if self.stages.len() == 0 {
				self.state = State::Finished;
				return;
			}
			let mut current_stage = &self.stages[*stage_index];
			// increment the current time
			*time += delta_time;
			// advance through the stages of the animation.
			// this is done in a loop in case we pass through
			// multiple stages in one frame.
			while *time >= current_stage.duration {
				*time -= current_stage.duration;
				*stage_index += 1;
				// if we reached the last stage, set the state
				// to finished and stop updating.
				if *stage_index >= self.stages.len() {
					self.state = State::Finished;
					// set the current value to the end value
					// of the last stage; otherwise, the final
					// value of the animation will be outdated
					self.current_value = current_stage.values.end;
					return;
				}
				current_stage = &self.stages[*stage_index];
			}
			self.current_value = tween(
				&current_stage.values,
				*time / current_stage.duration,
				&current_stage.easing,
			);
		}
	}

	pub fn current(&self) -> T {
		self.current_value
	}

	pub fn finished(&self) -> bool {
		if let State::Finished = self.state {
			true
		} else {
			false
		}
	}
}
