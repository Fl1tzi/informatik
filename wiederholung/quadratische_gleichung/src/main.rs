use std::env;

fn errechnen(a: f32, b: f32, c: f32) {
    if a == 0.0 {
        println!("a darf nicht 0 sein");
        return
    }
    let d = f32::powf(b, 2.0)-4.0*a*c;
    if d < 0.0 {
        println!("Es gibt keine Lösung, da d=0")
    } else if d == 0.0 {
        let loesung = -b+d.sqrt()/(2.0*a);
        println!("lösung: x={}", loesung);
    } else if d > 0.0 {
        let x1 = -b+d.sqrt()/(2.0*a);
        let x2 = -b-d.sqrt()/(2.0*a);
        println!("lösung: x1={} x2={}", x1, x2)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // first argument is program
    match args.len() {
    1 => {
        println!("no arguments")
    },
    2 => {
        match args[1].as_str() {
            "help" => println!("usage: {} [a] [b] [c]", args[0]),
            _ => println!("unknown command (use help)")
            }
        },
    3 => {
        println!("too few arguments")
    }
    4 => {
        let nums = || -> Result<Vec<f32>, std::num::ParseFloatError> {
            let a: f32 = args[1].parse()?;
            let b: f32 = args[2].parse()?;
            let c: f32 = args[3].parse()?;
            Ok(vec![a, b, c])
        };
        let values = nums();
        match values {
            Ok(v) => errechnen(v[0], v[1], v[2]),
            Err(_e) => println!("Fehler beim konvertieren der Variablen. Bitte benutze nur Zahlen.")
        }
    },
    _ => {
        println!("too many arguments")
    }
    }
}
