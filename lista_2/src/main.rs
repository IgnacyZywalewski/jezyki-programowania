//Lista 2
use std::fmt;
use std::f64::consts::PI;

mod zadanie_5;
use zadanie_5::Frac;

//Zadanie 1
#[derive(Debug)]
enum Fig {
    Kolo { r: f64 },
    Prost { a: f64, b: f64 },
    Kwadr { a: f64 },
    Romb { a: f64, alfa: f64 },
}

fn pole(f: &Fig) -> f64 {
    match f {
        Fig::Kolo { r } => PI * r * r,
        Fig::Prost { a, b } => a * b,
        Fig::Kwadr { a } => a * a,
        Fig::Romb { a, alfa } => a * a * alfa.sin(),
    }
}

fn obwod(f: &Fig) -> f64 {
    match f {
        Fig::Kolo { r } => 2.0 * PI * r,
        Fig::Prost { a, b } => 2.0 * (a + b),
        Fig::Kwadr { a } => 4.0 * a,
        Fig::Romb { a, alfa } => a * a * alfa.sin(),
    }
}
    
fn obrot90(f: &mut Fig) {
    match f {
        Fig::Prost { a, b } => std::mem::swap(a, b),
        Fig::Romb { alfa, .. } => *alfa = PI - *alfa, 
        _ => (),
    }
}

//Zadanie 2
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


fn main() {
    //Zadanie 1
    let mut figury = [
        Fig::Kolo { r: 1.5 },
        Fig::Prost { a: 1.0, b: 2.0 },
        Fig::Kwadr { a: 5.0 },
        Fig::Romb { a: 3.0, alfa: PI / 3.0 },
    ];

    for f in &mut figury {
        println!("{:?} ma pole = {:.2} obwod = {:.2}", f, pole(f), obwod(f));
        obrot90(f);
        println!("Po obrocie {:.2}\n", f);
    }

    //Zadanie 2
    for f in &figury {
        println!("{}", f);
    }

    println!();

    //Zadanie 5
    let a = Frac(2, 3); 
    let b = Frac(2, 4); 
    let c = Frac(2, 3); 
    let mut d = (a + b - c) * b / c;
    println!("Wynik: {:?}", d.uprosc());
    
}
