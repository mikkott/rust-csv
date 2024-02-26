use std::{error::Error, io, process};

#[derive(Debug, serde::Deserialize)]
struct POEItem {
    league: String,
    date: String,
    get: String,
    pay: String,
    value: f64,
    confidence: String,
}

fn read_csv_file() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .comment(Some(b'#'))
        .from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: POEItem = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = read_csv_file() {
        println!("erro reading csv to struct: {}", err);
        process::exit(1);
    }
}
