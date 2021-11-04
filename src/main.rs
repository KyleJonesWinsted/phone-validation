use std::{env::args, time::Instant};
use csv::{Reader, Writer};
use phonenumber::country::Id;
use serde::{Deserialize, Serialize};

fn main() {
    let start = Instant::now();
    let input_path = args().nth(1).expect("Missing CSV file path");
    let output_path = args().nth(2).expect("Missing output path");
    let rdr = Reader::from_path(input_path).expect("Did not find file. Try absolute path");
    let invalid_contacts = get_invalid_contacts(rdr);
    write_output_file(output_path, invalid_contacts);
    println!("Done! That took {:?}", start.elapsed());
}

fn write_output_file(path: String, contacts: Vec<Contact>) {
    let mut wtr = Writer::from_path(path).expect("Unable to create file with given path");
    for contact in contacts {
        wtr.serialize(contact).expect("Unable to serialize contact");
    }
    wtr.flush().expect("Unable to flush writer");
}

fn get_invalid_contacts(mut rdr: Reader<std::fs::File>) -> Vec<Contact> {
    rdr.deserialize()
        .filter_map(|result| {
            let contact: Contact = result.expect("Unable to parse contact");
            if is_valid_contact(&contact) {
                None
            } else {
                Some(contact)
            }
        })
        .collect()
}

fn is_valid_contact(contact: &Contact) -> bool {
    return is_valid_number(&contact.main_phone) && is_valid_number(&contact.office_phone);
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
struct Contact {
    entity_id: String,
    internal_id: String,
    main_phone: Option<String>,
    office_phone: Option<String>,
}
