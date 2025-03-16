use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy)]
pub struct Frac(pub i32, pub i32);

impl Frac{
	
	fn nwd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    pub fn uprosc(&mut self) -> Frac {
        let nwd = Self::nwd(self.0, self.1);
        self.0 /= nwd;
        self.1 /= nwd;

        if self.1 < 0 {
            self.0 = -self.0;
            self.1 = -self.1;
        }

        *self
    }
}


impl Add for Frac{
    type Output = Frac;

    fn add(self, other: Frac) -> Frac{
        Frac(
            self.0 * other.1 + self.1 * other.0,
            self.1 * other.1 
        ).uprosc()
    }
}

impl Sub for Frac{
    type Output = Frac;

    fn sub(self, other: Frac) -> Frac{
        Frac(
            self.0 * other.1 - self.1 * other.0,
            self.1 * other.1 
        ).uprosc()
    }
}

impl Mul for Frac{
    type Output = Frac;

    fn mul(self, other: Frac) -> Frac{
        Frac(
            self.0 * other.0,
            self.1 * other.1
        ).uprosc()
    }
}

impl Div for Frac{
    type Output = Frac;

    fn div(self, other: Frac) -> Frac{
        if other.1 == 0{
            println!("Nie mozna dzielic przez 0!!!");
        }

        Frac(
            self.0 * other.1,
            self.1 * other.0
        ).uprosc()
    }
}