//Lista 2
use std::fmt;
use std::f64::consts::PI;

mod zadanie_5;
use zadanie_5::Frac;

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::error::Error;


//Zadanie 1
#[derive(Debug, PartialEq, Clone)]
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

//Zadanie 3
fn save(filename: &str, v: Vec<Fig>) -> Result<(), Box<dyn Error>> {
    let mut file = BufWriter::new(File::create(filename)?);

    for fig in &v{
        let line = match fig {
            Fig::Kolo { r } => format!("Kolo {:.2}\n", r),
            Fig::Prost { a, b } => format!("Prostokat {:.2} {:.2}\n", a, b),
            Fig::Kwadr { a } => format!("Kwadrat {:.2}\n", a),
            Fig::Romb { a, alfa } => format!("Romb {:.2} {:.2}\n", a, alfa),
        };
        file.write_all(line.as_bytes())?;
    }

    Ok(())
}


fn load(filename: &str) -> Result<Vec<Fig>, Box<dyn Error>>{
    let file = BufReader::new(File::open(filename)?);

    let mut figures = Vec::new();

    for line in file.lines(){
        let line = line?;
        println!("Debug: Odczytana linia -> {}", line);
        let parts: Vec<&str> = line.split_whitespace().collect();

        let fig = match parts[..] {
            ["Kolo", r] => Fig::Kolo { r: r.parse()? },
            ["Prost", a, b] => Fig::Prost { a: a.parse()?, b: b.parse()? },
            ["Kwadr", a] => Fig::Kwadr { a: a.parse()? },
            ["Romb", a, alfa] => Fig::Romb { a: a.parse()?, alfa: alfa.parse()? },
            _ => return Err("Niepoprawny format danych".into()),
        };

        figures.push(fig);
    }

    Ok(figures)
}

fn main() {
    //Zadanie 1
    let mut figury = vec![
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

    //Zadanie 3
    match save("Figury.txt", figury.clone()) {
        Ok(_) => println!("Dane zapisane poprawnie!"),
        Err(e) => eprintln!("Blad zapisu: {}", e),
    }
    /*
    let figury1 = match load("Figury.txt") {
        Ok(figury1) => figury1,
        Err(e) => {
            eprintln!("Blad podczas wczytywania pliku: {}", e);
            return;
        }
    };
    assert_eq!(figury, figury1);*/

    //Zadanie 5
    let a = Frac(2, 3); 
    let b = Frac(2, 4); 
    let c = Frac(2, 3); 
    let mut d = (a + b - c) * b / c;
    println!("Wynik: {:?}", d.uprosc());
    
}