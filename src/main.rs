#![warn(clippy::all, clippy::pedantic)]

use csv::{Reader, Writer};
use phonenumber::country::Id;
use serde::{Deserialize, Serialize};
use std::{env::args, time::Instant};

fn main() {
    let start = Instant::now();
    let input_path = args().nth(1).expect("Missing CSV file path");
    let output_path = args().nth(2).expect("Missing output path");
    let rdr = Reader::from_path(input_path).expect("Did not find file. Try absolute path");
    let csv_rows: Vec<CsvRow> = get_csv_rows(rdr);
    let invalid_rows = get_invalid_contacts(csv_rows);
    write_output_file(output_path, invalid_rows);
    println!("Done! That took {:?}", start.elapsed());
}

fn write_output_file(path: String, contacts: Vec<CsvRow>) {
    let mut wtr = Writer::from_path(path).expect("Unable to create file with given path");
    for contact in contacts {
        wtr.serialize(contact).expect("Unable to serialize contact");
    }
    wtr.flush().expect("Unable to flush writer");
}

fn get_csv_rows(mut rdr: Reader<std::fs::File>) -> Vec<CsvRow> {
    rdr.deserialize()
        .map(|r| r.expect("Unable to read csv row"))
        .collect()
}

fn get_invalid_contacts(csv_rows: Vec<CsvRow>) -> Vec<CsvRow> {
    csv_rows
        .into_iter()
        .filter_map(|contact| {
            if is_valid_number(&contact.phone) {
                None
            } else {
                Some(contact)
            }
        })
        .collect()
}

fn is_valid_number(phone: &Option<String>) -> bool {
    if let Some(phone) = phone {
        match phonenumber::parse(Some(Id::US), phone) {
            Ok(number) => return number.is_valid(),
            Err(_) => return false,
        }
    }
    true
}

#[derive(Debug, Deserialize, Serialize)]
struct CsvRow {
    internal_id: String,
    phone: Option<String>,
}
