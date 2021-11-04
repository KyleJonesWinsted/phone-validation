#![warn(clippy::all, clippy::pedantic)]
mod offline_validation;
mod online_validation;
use csv::Reader;
use phone_validation::{get_csv_rows, CsvRow, Mode, Parameters};

use std::{error::Error, time::Instant};

use crate::{offline_validation::run_offline, online_validation::run_online};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let Parameters {
        input_path,
        output_path,
        mode,
    } = Parameters::get();
    let rdr = Reader::from_path(input_path).expect("Did not find file. Try absolute path");
    let csv_rows: Vec<CsvRow> = get_csv_rows(rdr);
    match mode {
        Mode::Offline => run_offline(csv_rows, output_path),
        Mode::Online => run_online(csv_rows, output_path).await?,
    };
    println!("Done! That took {:?}", start.elapsed());
    Ok(())
}
