use std::ops::{Add, Sub, Mul, Div};
use rand::Rng;

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
	pub e0: f32,
	pub e1: f32,
	pub e2: f32,
}

impl Vec3 {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Vec3{
            e0: a,
            e1: b,
            e2: c,
        }
    }

	pub fn random_min_max(min: f32, max: f32) -> Vec3 {
		Vec3 {
			e0: rand::thread_rng().gen_range(min..max),
			e1: rand::thread_rng().gen_range(min..max),
			e2: rand::thread_rng().gen_range(min..max),
		}
	}

	pub fn random_in_unit_sphere() -> Vec3 {
		let mut p = Vec3::random_min_max(-1.0, 1.0);
		while p.dot(p) >= 1.0 {
			p = Vec3::random_min_max(-1.0, 1.0);
		}
		p
	}

	pub fn x(&self) -> f32 {
		self.e0
	}

	pub fn y(&self) -> f32 {
		self.e1
	}

	pub fn z(&self) -> f32 {
		self.e2
	}

	pub fn length(&self) -> f32 {
		self.dot(*self).sqrt()
	}

	pub fn dot(&self, other: Vec3) -> f32 {
		self.e0 * other.e0 + self.e1 * other.e1 + self.e2 * other.e2
	}

	pub fn cross(&self, other: Vec3) -> Vec3 {
		Vec3 {
			e0: self.e1 * other.e2 - self.e2 * other.e1,
			e1: -(self.e0 * other.e2 - self.e2 * other.e0),
			e2: self.e0 * other.e1 - self.e1 * other.e0,
		}
	}

	pub fn unit_vector(&self) -> Vec3 {
		*self / self.length()
	}
}

impl Add for Vec3 {
	type Output = Vec3;

	fn add(self, other: Vec3) -> Vec3 {
		Vec3 {
			e0: self.e0 + other.e0,
			e1: self.e1 + other.e1,
			e2: self.e2 + other.e2,
		}
	}
}

impl Sub for Vec3 {
	type Output = Vec3;

	fn sub(self, other: Vec3) -> Vec3 {
		self + other * -1.0
	}
}

impl Mul for Vec3 {
	type Output = Vec3;

	fn mul(self, other: Vec3) -> Vec3 {
		Vec3 {
			e0: self.e0 * other.e0,
			e1: self.e1 * other.e1,
			e2: self.e2 * other.e2,
		}
	}
}

impl Div for Vec3 {
	type Output = Vec3;

	fn div(self, other: Vec3) -> Vec3 {
		self * other.unit_vector()
	}
}

impl Mul<f32> for Vec3 {
	type Output = Vec3;

	fn mul(self, other: f32) -> Vec3 {
		Vec3 {
			e0: self.e0 * other,
			e1: self.e1 * other,
			e2: self.e2 * other,
		}
	}
}

impl Div<f32> for Vec3 {
	type Output = Vec3;

	fn div(self, other: f32) -> Vec3 {
		self * (1.0 / other)
	}
}
