use std::error::Error as StdError;
use config::{ScanConfig, init_client};
use dynamo_utils::scan_and_dump_table;

mod config;
mod dynamo_utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    // Initialize the scan configuration
    let mut scan_config = ScanConfig::new_from_env();

    // Set the output file for the scan
    scan_config.output_file = "out/dynamodb_items.csv".to_string();

    // Initialize the DynamoDB client
    let client = init_client().await?;

    // Scan the DynamoDB table and write the primary keys to a CSV file
    scan_and_dump_table(&client, scan_config)
        .await
        .expect("Scan failed");

    println!("Primary keys have been written to CSV file");
    Ok(())
}
