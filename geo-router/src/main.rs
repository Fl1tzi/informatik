use std::{
    fs, io,
    io::{
        BufRead,
        stdin
    }
};

/// ähnelt einem unwrap, paniced aber nicht sondern skipped eine iteration
macro_rules! skip_if_err {
    ($res:expr) => {
        match $res {
            Ok(v) => v,
            Err(_) => continue
        }
    }
}

/// Daten in der CSV
#[derive(Debug)]
struct Standort {
    name: String,
    /// geografische Breite
    lat: f32,
    /// geografische Länge
    lon: f32
}

fn main() {
    let file = fs::File::open("coordinates.csv")
        .expect("Die Datei coordinates.csv konnte nicht geöffnet werden.");

    let reader = io::BufReader::new(file);

    let mut standorte: Vec<Standort> = Vec::new();

    // Zeilen einlesen
    for line in reader.lines() {
        match line {
            Ok(data) => {
                println!("{}", data);
                let v: [&str; 3] = data.split(';')
                    .collect::<Vec<&str>>()
                    .try_into()
                    .expect("Fehler in den Daten");

                standorte.push(Standort {
                    name: v[0].to_string(),
                    lat: skip_if_err!(v[1].parse::<f32>()),
                    lon: skip_if_err!(v[2].parse::<f32>())
                });
            },
            Err(e) => {
                println!("{}", e.kind())
            }
        }
    }

    let mut input = String::new();
    println!("Bitte gebe alle Städte in der richtigen Reihenfole mit einer Trennung durch ein \"/\" ein.\n");

    stdin().read_line(&mut input)
        .expect("Dieser String ist nicht erlaubt (kein UTF-8)");

    let mut letzter_standort: Option<&Standort> = None;

    for ort in input.split('/') {
        // whitespaces entfernen
        let ort = ort.trim();
        // überspringe Daten welche nicht gefunden werden
        let neuer_standort = match standorte.iter().find(|x| x.name == ort) {
            Some(v) => v,
            None => {
                println!("Der Standort \"{}\" wurde nicht gefunden", ort);
                continue
            }
        };

        // Berechne die Entfernung
        if let Some(letzter_standort) = letzter_standort {
            // Berechnung der Entfernung anhand vom Seitenkosinussatz
            let distance = || -> f32 {
                6378.388_f32
                *
                (
                    letzter_standort.lat.sin() * neuer_standort.lat.sin()
                    + 
                    letzter_standort.lat.cos() * neuer_standort.lat.cos()
                    *
                    (neuer_standort.lon - letzter_standort.lon).cos()  
                ).acos()
            };
            println!("{} -> {}: {} km", letzter_standort.name, neuer_standort.name, distance());
        }
        // den neuen standort für die nächste iteration merken
        letzter_standort = Some(neuer_standort);
    }
}
