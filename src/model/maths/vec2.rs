pub struct Vec2 {
    x: f64,
    y: f64
}

impl Vec2 {
	pub fn new(x: f64, y: f64) -> Self {
		Self { x, y }
	}

	pub fn x(&self) -> &f64 {
		&self.x
	}

	pub fn y(&self) -> &f64 {
		&self.y
	}

	pub fn dot(&self, other: &Self) -> f64 {
		self.x * other.x + self.y * other.y
	}

	pub fn get_norm(&self) -> f64 {
		(self.x * self.x + self.y * self.y).sqrt()
	}

	pub fn normalize(&self) -> Self {
		let norm: f64 = self.get_norm();
		Self {
			x: self.x / norm,
			y: self.y / norm
		}
	}
}