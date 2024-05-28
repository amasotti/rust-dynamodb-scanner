use aws_sdk_dynamodb::{Client};
use aws_sdk_dynamodb::types::{AttributeValue, Select};
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fs::OpenOptions;
use csv::WriterBuilder;

use crate::config::ScanConfig;

/// Scans a table and dumps the results into a CSV file.
///
/// # Arguments
///
/// * `client` - An instance of the `Client` struct from the DynamoDB crate.
/// * `scan_config` - The configuration for the scan operation.
///
/// # Returns
///
/// Returns `Ok(())` if the scan and dump operation was successful, otherwise returns an error.
///
pub async fn scan_and_dump_table(client: &Client, scan_config: ScanConfig) -> Result<(), Box<dyn StdError>> {
    let items: Result<Vec<_>, _> = client
        .scan()
        .table_name(scan_config.table_name)
        .attributes_to_get(&scan_config.primary_key_name)
        .limit(1000)
        .select(Select::SpecificAttributes)
        .into_paginator()
        .items()
        .send()
        .collect()
        .await;

    match items {
        Ok(items) => {
            write_to_csv(items, &scan_config.primary_key_name, &scan_config.output_file)?;
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    Ok(())
}


/// Writes items to a CSV file.
///
/// # Arguments
///
/// * `items` - A vector of hash maps representing the items to write.
/// * `primary_key_name` - The name of the primary key attribute.
/// * `file_path` - The path to the CSV file to write to.
///
/// # Errors
///
/// Returns an error if there was a problem opening or writing to the file.
///
fn write_to_csv(items: Vec<HashMap<String, AttributeValue>>, primary_key_name: &str, file_path: &str) -> Result<(), Box<dyn StdError>> {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;

    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(file);

    for item in items {
        if let Some(AttributeValue::S(value)) = item.get(primary_key_name) {
            wtr.write_record([value])?;
        }
    }

    wtr.flush()?;
    Ok(())
}
