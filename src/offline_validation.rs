use phone_validation::write_output_file;
use phonenumber::country::Id;

use crate::CsvRow;

pub fn run_offline(csv_rows: Vec<CsvRow>, output_path: String) {
    let invalid_rows = get_invalid_contacts(csv_rows);
    write_output_file(output_path, &invalid_rows);
}

fn get_invalid_contacts(csv_rows: Vec<CsvRow>) -> Vec<CsvRow> {
    csv_rows
        .into_iter()
        .filter_map(|mut contact| {
            if contact.phone.is_empty() || is_valid_number(&mut contact.phone) {
                None
            } else {
                contact.phone_type = Some("Invalid".to_string());
                Some(contact)
            }
        })
        .collect()
}

fn is_valid_number(phone: &String) -> bool {
    match phonenumber::parse(Some(Id::US), phone) {
        Ok(number) => return number.is_valid(),
        Err(_) => return false,
    }
}
