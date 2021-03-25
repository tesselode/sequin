pub trait Easing: std::fmt::Debug + Send + Sync {
	fn ease(&self, x: f32) -> f32;
}

#[derive(Debug, Clone, Copy)]
pub struct Linear;

impl Easing for Linear {
	fn ease(&self, x: f32) -> f32 {
		x
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Powi(pub i32);

impl Easing for Powi {
	fn ease(&self, x: f32) -> f32 {
		x.powi(self.0)
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Powf(pub f32);

impl Easing for Powf {
	fn ease(&self, x: f32) -> f32 {
		x.powf(self.0)
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Out<T: Easing>(pub T);

impl<T: Easing> Easing for Out<T> {
	fn ease(&self, x: f32) -> f32 {
		1.0 - self.0.ease(1.0 - x)
	}
}
