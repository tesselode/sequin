use std::ops::{Add, Mul, Sub};

pub trait Tweenable:
	Sized + Copy + Add<Self, Output = Self> + Sub<Self, Output = Self> + Mul<f32, Output = Self>
{
}

impl<T> Tweenable for T where
	T: Copy + Add<T, Output = T> + Sub<T, Output = T> + Mul<f32, Output = T>
{
}
