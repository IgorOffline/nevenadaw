// [package]
// name = "rustenronshinydecember"
// version = "0.1.0"
// edition = "2024"
// 
// [dependencies]
// addr-spec = "0.8.4"
// csv = "1.4.0"
// num-format = "0.4.4"
// serde = { version = "1.0.228", features = ["derive"] }
// serde_json = "1.0.145"

use addr_spec::AddrSpec;
use csv::StringRecord;
use num_format::{Locale, ToFormattedString};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
struct EnronRecord {
    header: String,
    body_from: String,
    body_to: String,
}

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
struct EnronRecordWithEmails {
    header: String,
    body_from: String,
    body_to: String,
    from_emails: Vec<String>,
    to_emails: Vec<String>,
}

fn main() {
    let intro_one = "Loaded 517.401 records from emails.txt";
    let intro_two = "496.931/517.401 (20.470)";
    println!("{}", intro_one);
    println!("{}", intro_two);
    let file = File::open("enron_record_with_emails.txt")
        .expect("Failed to open enron_record_with_emails.txt");
    let reader = BufReader::new(file);
    let enron_records: HashSet<EnronRecordWithEmails> = serde_json::from_reader(reader)
        .expect("Failed to deserialize enron_record_with_emails.txt");

    println!(
        "Loaded {} records from enron_record_with_emails.txt",
        enron_records.len().to_formatted_string(&Locale::hr)
    );

    assert_eq!(enron_records.len(), 496_931);
    process(&enron_records);
}

fn process(enron_records: &HashSet<EnronRecordWithEmails>) {
    let mut from_emails_with_len_not_equals_one = 0;
    let mut to_emails_with_len_not_equals_one = 0;
    enron_records.iter().for_each(|record| {
        if record.from_emails.len() != 1 {
            from_emails_with_len_not_equals_one += 1;
        }
        if record.to_emails.len() != 1 {
            to_emails_with_len_not_equals_one += 1;
        }
    });
    println!(
        "from_emails_with_len_not_equals_one={}",
        from_emails_with_len_not_equals_one.to_formatted_string(&Locale::hr)
    );
    println!(
        "to_emails_with_len_not_equals_one={}",
        to_emails_with_len_not_equals_one.to_formatted_string(&Locale::hr)
    );
}

#[allow(dead_code)]
fn prepare_enron_records_cleaner() {
    let file = File::open("emails.txt").expect("Failed to open emails.txt");
    let reader = BufReader::new(file);
    let enron_records: HashSet<EnronRecord> =
        serde_json::from_reader(reader).expect("Failed to deserialize emails.txt");

    println!(
        "Loaded {} records from emails.txt",
        enron_records.len().to_formatted_string(&Locale::hr)
    );
    let mut enron_records_cleaner: HashSet<EnronRecord> = HashSet::new();
    enron_records.iter().for_each(|record| {
        if record.body_from.starts_with("From: ") && record.body_to.starts_with("To: ") {
            let new_record = EnronRecord {
                header: record.header.clone(),
                body_from: record.body_from.clone(),
                body_to: record.body_to.clone(),
            };
            enron_records_cleaner.insert(new_record);
        }
    });
    let count_difference = enron_records.len() - enron_records_cleaner.len();
    println!(
        "{}/{} ({})",
        count_difference.to_formatted_string(&Locale::hr),
        enron_records.len().to_formatted_string(&Locale::hr),
        enron_records_cleaner.len().to_formatted_string(&Locale::hr)
    );
    process_enron_records_cleaner(&enron_records_cleaner);
}

#[allow(dead_code)]
fn process_enron_records_cleaner(enron_records_cleaner: &HashSet<EnronRecord>) {
    //email_processing_example();
    let mut enron_record_with_emails: HashSet<EnronRecordWithEmails> = HashSet::new();
    enron_records_cleaner.iter().for_each(|record| {
        let from_emails = find_emails(&record.body_from)
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let to_emails = find_emails(&record.body_to)
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let new_record = EnronRecordWithEmails {
            header: record.header.clone(),
            body_from: record.body_from.clone(),
            body_to: record.body_to.clone(),
            from_emails,
            to_emails,
        };
        enron_record_with_emails.insert(new_record);
    });
    serde_json::to_writer(
        BufWriter::new(File::create("enron_record_with_emails123456.txt").unwrap()),
        &enron_record_with_emails,
    )
    .expect("Failed to serialize enron_record_with_emails.txt");
}

#[allow(dead_code)]
fn email_processing_example() {
    let input = "Hey [user1@example.com] check this out, also contact \"support@test\"@domain.co or admin@internal.net.";
    let found = find_emails(input);
    println!("Found {} valid emails:", found.len());
    for email in found {
        println!(" - {}", email);
    }
}

fn find_emails(text: &str) -> Vec<&str> {
    text.split(|c: char| c.is_whitespace() || ['[', ']', '(', ')', ',', ';'].contains(&c))
        .filter(|part| part.contains('@'))
        .filter(|part| part.parse::<AddrSpec>().is_ok())
        .collect()
}

fn _enron_initial_from_to() {
    let mut enron_records: HashSet<EnronRecord> = HashSet::new();
    let mut header_count: HashMap<String, i32> = HashMap::new();
    let file_path = r"C:\datasets\enron\emails.csv";
    if let Ok(file) = File::open(file_path) {
        let mut reader = csv::Reader::from_reader(file);
        let mut parsing_success = 0;
        let mut parsing_failure = 0;
        let mut record_error_counter = 0;
        for result in reader.records().take(999_999) {
            if let Ok(record) = result {
                let parsed_enron_record = parse_record(&record);
                if parsed_enron_record.is_some() {
                    let parsed_enron_record_unwrapped = parsed_enron_record.unwrap();
                    let enron_record_one = EnronRecord {
                        header: parsed_enron_record_unwrapped.header.to_string(),
                        body_from: parsed_enron_record_unwrapped.body_from.to_string(),
                        body_to: parsed_enron_record_unwrapped.body_to.to_string(),
                    };
                    let enron_record_two = EnronRecord {
                        header: parsed_enron_record_unwrapped.header.to_string(),
                        body_from: parsed_enron_record_unwrapped.body_from.to_string(),
                        body_to: parsed_enron_record_unwrapped.body_to.to_string(),
                    };
                    header_count.insert(enron_record_one.header.to_string(), -1);
                    enron_records.insert(enron_record_two);
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
        if *count > 9_999_999 {
            println!("{}: {}", person, count);
        }
    }
    //let _to_fields: HashSet<String> = enron_records
    //    .iter()
    //    .map(|r| r.body_to.to_string())
    //    .collect();
    //println!("to_fields.len={}", to_fields.len());
    //persist(&to_fields);
    // --- --- ---
    serde_json::to_writer(
        BufWriter::new(File::create("emails123456.txt").unwrap()),
        &enron_records,
    )
    .unwrap();
}

fn _persist_one(to_fields: &HashSet<String>) {
    if let Ok(file) = File::create("emails.txt") {
        let mut writer = BufWriter::new(file);
        for from in to_fields {
            let _ = writeln!(writer, "{}", from);
        }
    }
}

//println!("{}: {}", i, k);

#[allow(dead_code)]
fn parse_record(record: &StringRecord) -> Option<EnronRecord> {
    match (record.get(0), record.get(1)) {
        (Some(header), Some(body)) => {
            //println!("---");
            //println!("{}", header);
            //println!("---");
            //println!("{}", body);
            //println!("---");
            let body_from_raw = body.lines().find(|line| line.starts_with("From:"));
            let body_from = body_from_raw.unwrap_or("");
            let body_to_raw = body.lines().find(|line| line.starts_with("To:"));
            let body_to = body_to_raw.unwrap_or("");
            Some(EnronRecord {
                header: header.to_string(),
                body_from: body_from.to_string(),
                body_to: body_to.to_string(),
            })
        }
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_find_emails() {
//         let text = "From: user@example.com, To: [other@example.org]; (third@example.net)";
//         let emails = find_emails(text);
//         assert_eq!(
//             emails,
//             vec!["user@example.com", "other@example.org", "third@example.net"]
//         );
//     }
// }
