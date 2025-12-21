/*
[package]
name = "rustenronshinydecember"
version = "0.1.0"
edition = "2024"

[dependencies]
csv = "1.4.0"
num-format = "0.4.4"
*/

use csv::StringRecord;
use num_format::{Locale, ToFormattedString};
use std::collections::HashMap;
use std::fs::File;

fn main() {
    let mut header_count: HashMap<String, i32> = HashMap::new();
    let file_path = r"C:\datasets\enron\emails.csv";
    if let Ok(file) = File::open(file_path) {
        let mut reader = csv::Reader::from_reader(file);
        let mut parsing_success = 0;
        let mut parsing_failure = 0;
        let mut record_error_counter = 0;
        for result in reader.records().take(999_999) {
            if let Ok(record) = result {
                let parsed_header = parse_record(&record);
                if parsed_header.is_some() {
                    header_count.insert(parsed_header.unwrap().to_string(), -1);
                    parsing_success += 1;
                } else {
                    parsing_failure += 1;
                }
            } else {
                record_error_counter += 1;
            }
        }
        println!(
            "parsing_success={}",
            parsing_success.to_formatted_string(&Locale::hr)
        );
        println!(
            "parsing_failure={}",
            parsing_failure.to_formatted_string(&Locale::hr)
        );
        println!(
            "record_error_counter={}",
            record_error_counter.to_formatted_string(&Locale::hr)
        );
    }
    println!(
        "header_count.len={}",
        header_count.len().to_formatted_string(&Locale::hr)
    );
    let mut person_count: HashMap<&str, i32> = HashMap::new();
    header_count.iter().take(999_999).for_each(|(k, _v)| {
        let person = k.split('/').next().unwrap_or("");
        *person_count.entry(person).or_insert(0) += 1;
    });
    println!("person_count.len()={}", person_count.len());
    //println!("person_count={:?}", person_count);
    let mut sorted_counts: Vec<(&&str, &i32)> = person_count.iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(a.1));
    for (person, count) in sorted_counts {
        println!("{}: {}", person, count);
    }
}
//println!("{}: {}", i, k);

fn parse_record(record: &StringRecord) -> Option<String> {
    match (record.get(0), record.get(1)) {
        (Some(header), Some(_body)) => Some(header.to_string()),
        _ => None,
    }
}
// use std::error::Error;
// println!("{}: {}", header, body)
// eprintln!("Error: Record is missing header or body")

// fn _read_csv() -> Result<(), Box<dyn Error>> {
//     let file_path = r"C:\datasets\enron\emails.csv";
//     let file = File::open(file_path)?;
//
//     let mut reader = csv::Reader::from_reader(file);
//
//     for result in reader.records().take(5) {
//         let record = result?;
//         println!("{:?}", record);
//     }
//
//     Ok(())
// }
