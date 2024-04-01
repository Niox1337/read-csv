use csv;
use std::error::Error;
use std::env::args;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>>{
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record)
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a file path");
        std::process::exit(1);
    }

    let file_path = &args[1];
    if let Err(e) = read_from_file(file_path) {
        eprintln!("Error reading file: {}", e);
    }
}
