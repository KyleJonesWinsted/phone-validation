use std::env::args;

use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CsvRow {
    pub internal_id: String,
    pub phone: String,
    pub phone_type: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Online,
    Offline,
}

pub struct Parameters {
    pub input_path: String,
    pub output_path: String,
    pub mode: Mode,
}

impl Parameters {
    pub fn get() -> Parameters {
        let input_path = args().nth(1).expect("Missing CSV file path");
        let output_path = args().nth(2).expect("Missing output path");
        let mode = Self::get_mode();
        Parameters {
            input_path,
            output_path,
            mode,
        }
    }

    fn get_mode() -> Mode {
        match args().nth(3) {
            Some(s) if s == "--online" => Mode::Online,
            _ => Mode::Offline,
        }
    }
}

pub fn get_csv_rows(mut rdr: Reader<std::fs::File>) -> Vec<CsvRow> {
    rdr.deserialize()
        .map(|r| r.expect("Unable to read csv row"))
        .collect()
}

pub fn write_output_file(path: String, contacts: Vec<CsvRow>) {
    let mut wtr = Writer::from_path(path).expect("Unable to create file with given path");
    for contact in contacts {
        wtr.serialize(contact).expect("Unable to serialize contact");
    }
    wtr.flush().expect("Unable to flush writer");
}
