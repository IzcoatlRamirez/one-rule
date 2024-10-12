use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn zero_rule(filename: &str, target_column: usize) -> io::Result<String> {
    let mut class_counts: HashMap<String, i32> = HashMap::new();
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let columns: Vec<&str> = line.split(',').collect();

        if target_column < columns.len() {
            let target_value = columns[target_column];
            let counter = class_counts.entry(target_value.to_string()).or_insert(0);
            *counter += 1;
        }
    }

    let mut major_class = String::new();
    let mut max_count = 0;

    for (class, count) in &class_counts {
        if *count > max_count {
            major_class = class.clone();
            max_count = *count;
        }
    }

    Ok(major_class)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Uso: {} <archivo_csv> <columna_target>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let target_column: usize = args[2].parse().expect("El índice de columna debe ser un número");

    match zero_rule(filename, target_column) {
        Ok(class) => println!("{}", class),
        Err(e) => eprintln!("Error al leer el archivo: {}", e),
    }
}
