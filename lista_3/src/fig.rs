use std::f64::consts::PI;
use std::fmt;
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Fig {
    Kolo { r: f64 },
    Prost { a: f64, b: f64 },
    Kwadr { a: f64 },
    Romb { a: f64, alfa: f64 },
}

pub trait Figura {
    fn pole(&self) -> f64;
    fn obwod(&self) -> f64;
    //fn paint(&self) -> Shape;
}

impl Figura for Fig {
    fn pole(&self) -> f64 {
        match self {
            Fig::Kolo { r } => PI * r * r,
            Fig::Prost { a, b } => a * b,
            Fig::Kwadr { a } => a * a,
            Fig::Romb { a, alfa } => a * a * alfa.sin(),
        }
    }

    fn obwod(&self) -> f64 {
        match self {
            Fig::Kolo { r } => 2.0 * PI * r,
            Fig::Prost { a, b } => 2.0 * (a + b),
            Fig::Kwadr { a } => 4.0 * a,
            Fig::Romb { a, alfa } => a * a * alfa.sin(),
        }
    }

    /*
    fn paint(&self) -> Shape {
        match self {
            Fig::Kolo { r } => {
                Shape::circle(*r)
            },
            Fig::Prost { a, b } => {
                Shape::rect(*a, *b)
            },
            Fig::Kwadr { a } => {
                Shape::rect(*a, *a)
            },
            Fig::Romb { a, alfa } => {
                let height = a * alfa.sin();
                Shape::rect(*a, height)
            },
        }
    }*/
}


impl fmt::Display for Fig{
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Fig::Kolo { r } => write!(f, "Kolo (r = {})", r),
            Fig::Prost { a, b } => write!(f, "Prostokat (a = {}, b = {})", a, b),
            Fig::Kwadr { a } => write!(f, "Kwadrat (a = {})", a),
            Fig::Romb { a, alfa } => write!(f, "Romb (a = {}, alfa = {:.2})", a, alfa),
        }
    } 
}