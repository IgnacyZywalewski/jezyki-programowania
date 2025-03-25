use std::ops::{Add, Sub, Mul};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Poly {
	pub a: Vec<f32>
}

impl Poly{
	pub fn eval(&self, x: f32) -> f32 {
		let mut result = 0.0;
		let mut power = 1.0;
		
		for &coef in &self.a {
			result += coef * power;
			power *= x;
		}
		result
	}
}


//Dodawanie
impl Add for Poly {
	type Output = Poly;

	fn add(self, other: Poly) -> Poly {
		let max_len = self.a.len().max(other.a.len());
		let mut result = vec![0.0; max_len];

		for i in 0..self.a.len() {
			result[i] += self.a[i]; 
		}

		for i in 0..other.a.len() {
			result[i] += other.a[i];
		}

		Poly { a: result }
	}
}

impl Add<f32> for Poly {
	type Output = Poly;

	fn add(self, value: f32) -> Poly {
		let mut result = self.a.clone();
		
		if !result.is_empty() {
			result[0] += value;
		}
		else {
			result.push(value);
		}
		Poly { a: result } 
	}
}


impl Add<Poly> for f32 {
	type Output = Poly;

	fn add(self, poly: Poly) -> Poly {
		poly + self
	}
}


//Odejmowanie
impl Sub for Poly {
	type Output = Poly;

	fn sub(self, other: Poly) -> Poly {
		let max_len = self.a.len().max(other.a.len());
		let mut result = vec![0.0; max_len];

		for i in 0..self.a.len() {
			result[i] += self.a[i];
		}

		for i in 0..other.a.len() {
			result[i] -= other.a[i];
		}

		Poly { a: result }
	}
}

impl Sub<f32> for Poly {
	type Output = Poly;

	fn sub(self, value: f32) -> Poly {
		let mut result = self.a.clone();
		
		if !result.is_empty() {
			result[0] -= value;
		}
		else {
			result.push(value);
		}
		Poly { a: result } 
	}
}

impl Sub<Poly> for f32 {
	type Output = Poly;

	fn sub(self, poly: Poly) -> Poly {
		poly - self
	}
}


//Mnozenie
impl Mul for Poly {
	type Output = Poly;

	fn mul(self, other: Poly) -> Poly {
		let mut result = vec![0.0; self.a.len() + other.a.len() - 1];

		for (i, &x) in self.a.iter().enumerate() {
			for (j, &y) in other.a.iter().enumerate() {
				result[i + j] += x * y;
			}
		}

		Poly { a: result }
	}
}

impl Mul<f32> for Poly {
	type Output = Poly;

	fn mul(self, value: f32) -> Poly {
		let mut result = self.a.clone();

        for i in 0..self.a.len() {
			result[i] *= value;
		}

		Poly { a: result }
    }
}

impl Mul<Poly> for f32 {
    type Output = Poly;

    fn mul(self, poly: Poly) -> Poly {
        poly * self
    }
}



impl fmt::Display for Poly{
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

		for (i, &coef) in self.a.iter().enumerate() {
			if coef != 0.0 {
				if !result.is_empty() {
					result.push_str(" + ");
				}
				result.push_str(&format!("{:.2}", coef));
			}
			if i > 0 {
				result.push_str(&format!("x^{}", i));
			}
		}

		write!(f, "{}", result)
    } 
}