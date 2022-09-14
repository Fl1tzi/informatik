use std::env;

fn bmi_berechnen(alter: u32, groesse: u32, gewicht: u32) {
    let bmi: f32 = (gewicht as f32)/f32::powf(groesse as f32/100.0, 2.0);
    let normalgewicht: i32 = (groesse as i32)-100;
    let ideal_m: f32 = (normalgewicht as f32)*0.95;
    let ideal_w: f32 = (normalgewicht as f32)*0.90;

    if (0..=18).contains(&alter) {
        println!("Du bist zu jung!");
        return;
    }

    let optimaler_bmi = match alter {
            19..=24 => 19..=24,
            25..=34 => 20..=25,
            35..=44 => 21..=26,
            45..=54 => 22..=27,
            55..=64 => 23..=28,
            _ => 24..=29
    };

    println!("Dein BMI: {} ({})", &bmi, &(bmi as u32));
    if optimaler_bmi.contains(&(bmi as u32)) {
        println!("\tJUHU. Du befindest dich in dem optimalen Bmi.")
    } else {
        if (bmi as u32) < optimaler_bmi.min().unwrap() {
            println!("\tDu befindest dich unter dem optimalen Bmi.")
        } else {
            println!("\tDu befindest dich über dem optimalen Bmi.")
        }
    }
    println!("Idealgewicht (Mann): {}", ideal_m);
    println!("Idealgewicht (Frau): {}", ideal_w);

}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        4 => {
            let args_to_u32 = || -> Result<[u32; 3], std::num::ParseIntError> {
                let alter: u32 = args[1].parse()?;
                let groesse: u32 = args[2].parse()?;
                let gewicht: u32 = args[3].parse()?;
                Ok([alter, groesse, gewicht])
            };
            match args_to_u32() {
                Ok(v) => bmi_berechnen(v[0], v[1], v[2]),
                Err(_e) => println!("Du musst Zahlen angeben, welche nicht kleiner als null sind.")
            }
        }
        _ => {
            println!("Nutzung: [alter] [größe] [gewicht]");
            return;
        }
    }

}
