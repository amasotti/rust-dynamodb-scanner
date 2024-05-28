//! # Config
//!
//! This module contains the configuration for the application.


use aws_config::BehaviorVersion;
use aws_config::profile::ProfileFileCredentialsProvider;
use aws_sdk_dynamodb::Client;
use std::error::Error as StdError;

/// Configuration for the scan operation.
/// It contains the dynamoDB table name and the primary key name to be used in the scan operation.
#[derive(Debug)]
pub struct ScanConfig {
    pub table_name: String,
    pub primary_key_name: String,
    pub output_file: String,
}


impl ScanConfig {

    /// Creates a new `ScanConfig` instance with the specified `table_name` and `primary_key_name`.
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the table to scan.
    /// * `primary_key_name` - The name of the primary key for the table.
    ///
    /// # Returns
    ///
    /// A new `ScanConfig` instance with the specified `table_name` and `primary_key_name`.
    #[allow(dead_code)]
    pub fn new(table_name: &str, primary_key_name: &str) -> Self {
        ScanConfig {
            table_name: table_name.to_string(),
            primary_key_name: primary_key_name.to_string(),
            output_file: "".to_string(),
        }
    }

    /// Creates a new `ScanConfig` instance using environment variables.
    ///
    /// # Panics
    ///
    /// This function will panic if either `DYNAMODB_TABLE_NAME` or `DYNAMODB_PRIMARY_KEY_NAME` environment variables are not set.
    ///
    /// # Examples
    ///
    /// ```rust,should_panic
    /// use std::env;
    /// use crate::ScanConfig;
    ///
    /// env::set_var("DYNAMODB_TABLE_NAME", "my_table");
    /// env::set_var("DYNAMODB_PRIMARY_KEY_NAME", "id");
    ///
    /// let config = ScanConfig::new_from_env();
    /// assert_eq!(config.table_name, "my_table");
    /// assert_eq!(config.primary_key_name, "id");
    /// ```
    ///
    pub fn new_from_env() -> Self {
        let table_name = std::env::var("DYNAMODB_TABLE_NAME").unwrap();
        let primary_key_name = std::env::var("DYNAMODB_PRIMARY_KEY_NAME").unwrap();
        ScanConfig {
            table_name,
            primary_key_name,
            output_file: "".to_string(),
        }
    }
}

/// Configuration for connecting to DynamoDB using AWS profile.
///
/// # Fields
///
/// * `profile_name` - Name of the AWS profile to be used for authentication.
pub struct DynamoConfig {
    pub profile_name: String,
}


impl DynamoConfig {
    #[allow(dead_code)]
    pub fn new(profile_name: &str) -> Self {
        DynamoConfig {
            profile_name: profile_name.to_string(),
        }
    }

    /// Creates a new `DynamoConfig` instance from the environment.
    ///
    /// This function reads the value of the environment variable `AWS_PROFILE` and
    /// uses it as the `profile_name` for the `DynamoConfig` instance. If the
    /// environment variable is not set, this function will panic.
    ///
    /// # Panics
    ///
    /// This function will panic if the value of the `AWS_PROFILE` environment
    /// variable is not set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use my_crate::DynamoConfig;
    ///
    /// let config = DynamoConfig::new_from_env();
    /// ```
    ///
    pub fn new_from_env() -> Self {
        let profile_name = std::env::var("AWS_PROFILE").unwrap();
        DynamoConfig {
            profile_name,
        }
    }
}


pub async fn init_client() -> Result<Client, Box<dyn StdError>> {
    let dynamo_config = DynamoConfig::new_from_env();
    let profile_loader = ProfileFileCredentialsProvider::builder()
        .profile_name(&dynamo_config.profile_name)
        .build();
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let dynamo_config = aws_sdk_dynamodb::config::Builder::from(&config)
        .credentials_provider(profile_loader)
        .build();
    let client = Client::from_conf(dynamo_config);
    Ok(client)
}