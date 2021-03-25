pub mod easing;
pub mod sequence;
mod tweenable;

use std::ops::Range;

use easing::Easing;
use tweenable::Tweenable;

pub fn tween<T: Tweenable, E: Easing>(values: Range<T>, progress: f32, easing: E) -> T {
	values.start + (values.end - values.start) * easing.ease(progress)
}
