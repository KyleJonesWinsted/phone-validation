use std::{
    cmp,
    error::Error,
    thread::sleep,
    time::{Duration, Instant},
};

use phone_validation::{write_output_file, CsvRow};
use serde::{Deserialize, Serialize};

pub async fn run_online(csv_rows: Vec<CsvRow>, output_path: String) -> Result<(), Box<dyn Error>> {
    let results = make_requests(&csv_rows).await?;
    write_output_file(output_path, results);
    Ok(())
}

// Only 10 requests per second allowed
async fn make_requests(csv_rows: &Vec<CsvRow>) -> Result<Vec<CsvRow>, Box<dyn Error>> {
    let mut start_index = 0;
    let mut results = Vec::new();
    loop {
        let start_time = Instant::now();
        let end_index = cmp::min(start_index + 10, csv_rows.len() - 1);
        let mut requests = Vec::new();
        for index in start_index..end_index {
            let row = csv_rows.get(index).unwrap();
            requests.push(tokio::spawn(send_request(row.clone())));
        }
        for request in requests {
            results.push(request.await??);
        }
        start_index += 10;
        if start_index >= csv_rows.len() {
            break;
        }
        let elapsed = start_time.elapsed();
        sleep(Duration::from_millis(
            (1000 - elapsed.as_millis()).try_into().unwrap(),
        ));
    }
    Ok(results)
}

async fn send_request(csv_row: CsvRow) -> Result<CsvRow, reqwest::Error> {
    let data = reqwest::get("https://www.google.com").await?.text().await?;
    // let result: ResponseData = serde_json::from_str(&data).expect("Unable to parse result");
    // TODO: parse result and update csv row
    Ok(csv_row)
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseData {}
