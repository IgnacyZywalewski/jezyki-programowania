use std::f64::consts::PI;
use std::fmt;
use serde::{Serialize, Deserialize};
use flo_draw::canvas::{GraphicsContext, Color};
use flo_draw::canvas::*;


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
    fn paint(&self, gc: &mut dyn GraphicsContext, offset_x: f32, offset_y: f32);
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

    fn paint(&self, gc: &mut dyn GraphicsContext, offset_x: f32, offset_y: f32) {
        let scale = 30.0;
        gc.new_path();
        match self {
            Fig::Kolo { r } => {
                gc.circle(offset_x, offset_y, (*r * scale) as f32);
            }
            Fig::Prost { a, b } => {
                let w = (*a * scale / 2.0) as f32;
                let h = (*b * scale / 2.0) as f32;

                gc.move_to(offset_x - w, offset_y - h);
                gc.line_to(offset_x + w, offset_y - h);
                gc.line_to(offset_x + w, offset_y + h);
                gc.line_to(offset_x - w, offset_y + h);
                gc.close_path();
            }
            Fig::Kwadr { a } => {
                let w = (*a * scale / 2.0) as f32;

                gc.move_to(offset_x - w, offset_y - w);
                gc.line_to(offset_x + w, offset_y - w);
                gc.line_to(offset_x + w, offset_y + w);
                gc.line_to(offset_x - w, offset_y + w);
                gc.close_path();
            }
            Fig::Romb { a, alfa } => {
                let dx = (*a * scale / 2.0) as f32;
                let dy = dx * (alfa.to_radians().cos()) as f32;

                gc.move_to(offset_x, offset_y - dy);
                gc.line_to(offset_x + dx, offset_y);
                gc.line_to(offset_x, offset_y + dy);
                gc.line_to(offset_x - dx, offset_y);
                gc.close_path();
            }
        }

        gc.fill_color(Color::Rgba(0.3, 0.6, 0.8, 1.0));
        gc.fill();
        gc.stroke_color(Color::Rgba(0.0, 0.0, 0.0, 1.0));
        gc.line_width(2.0);
        gc.stroke();
    }
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