use std::ops::{Add, Mul, Sub};

pub trait Tweenable: Sized + Copy {
	fn lerp(self, other: Self, amount: f32) -> Self;
}

impl<T> Tweenable for T
where
	T: Copy + Add<T, Output = T> + Sub<T, Output = T> + Mul<f32, Output = T>,
{
	fn lerp(self, other: Self, amount: f32) -> Self {
		self + (other - self) * amount
	}
}
