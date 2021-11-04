use std::{
    cmp, env,
    error::Error,
    io::{stdout, Write},
    thread::sleep,
    time::{Duration, Instant},
};

use phone_validation::{write_output_file, CsvRow};
use serde::{Deserialize, Serialize};
use urlencoding::encode;

pub async fn run_online(csv_rows: Vec<CsvRow>, output_path: String) -> Result<(), Box<dyn Error>> {
    let api_key = env::var("PHONE_VALIDATOR_API_KEY").unwrap();
    let results = fetch_phone_types(&csv_rows, api_key).await?;
    write_output_file(output_path, results);
    Ok(())
}

// Only 10 requests per second allowed
async fn fetch_phone_types(
    csv_rows: &Vec<CsvRow>,
    api_key: String,
) -> Result<Vec<CsvRow>, Box<dyn Error>> {
    let mut start_index = 0;
    let mut results = Vec::with_capacity(csv_rows.len());
    let start = Instant::now();
    loop {
        print_progress(start_index, &results, start);
        let start_time = Instant::now();
        let end_index = cmp::min(start_index + 10, csv_rows.len() - 1);
        let mut requests = Vec::new();
        for index in start_index..end_index {
            let row = csv_rows.get(index).unwrap();
            requests.push(tokio::spawn(send_request(row.clone(), api_key.clone())));
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
            (1000 - elapsed.as_millis()).try_into().unwrap_or(0),
        ));
    }
    Ok(results)
}

async fn send_request(mut csv_row: CsvRow, api_key: String) -> Result<CsvRow, reqwest::Error> {
    let url = format!(
        "https://www.phonevalidator.com/api/v2/phonesearch?apikey={}&phone={}&type=basic",
        api_key,
        encode(&csv_row.phone.split('x').nth(0).unwrap_or_default())
    );
    let data = reqwest::get(url).await?.text().await?;
    let result: ResponseData =
        serde_json::from_str(&data).expect(&format!("Unable to parse data {}", data));
    csv_row.phone_type = result.phone_basic.unwrap_or_default().line_type;
    Ok(csv_row)
}

fn print_progress(index: usize, csv_rows: &Vec<CsvRow>, start: Instant) {
    let last_row = if let Some(row) = csv_rows.get(index - 1) {
        format!("{} {:?}", row.phone, row.phone_type)
    } else {
        "".to_string()
    };
    print!(
        "\rFinished {:5} of {:5}  Elapsed: {:?}    Last Number: {:40}",
        index,
        csv_rows.capacity(),
        start.elapsed(),
        last_row,
    );
    stdout().flush().unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseData {
    #[serde(rename = "Phone")]
    phone: Option<String>,
    #[serde(rename = "Cost")]
    cost: Option<f64>,
    #[serde(rename = "SearchDate")]
    search_date: Option<String>,
    #[serde(rename = "StatusCode")]
    status_code: Option<String>,
    #[serde(rename = "StatusMessage")]
    status_message: Option<String>,
    #[serde(rename = "PhoneBasic")]
    phone_basic: Option<ResponsePhoneBasic>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponsePhoneBasic {
    #[serde(rename = "PhoneNumber")]
    phone_number: Option<String>,
    #[serde(rename = "ReportDate")]
    report_date: Option<String>,
    #[serde(rename = "LineType")]
    line_type: Option<String>,
    #[serde(rename = "PhoneCompany")]
    phone_company: Option<String>,
    #[serde(rename = "PhoneLocation")]
    phone_location: Option<String>,
    #[serde(rename = "ErrorCode")]
    error_code: Option<String>,
    #[serde(rename = "ErrorDescription")]
    error_description: Option<String>,
}

impl Default for ResponsePhoneBasic {
    fn default() -> Self {
        Self {
            phone_number: Default::default(),
            report_date: Default::default(),
            line_type: Default::default(),
            phone_company: Default::default(),
            phone_location: Default::default(),
            error_code: Default::default(),
            error_description: Default::default(),
        }
    }
}
