# DynamoDB Table Scanner - Rust Demo

This project demonstrates how to use the AWS SDK for Rust to scan a DynamoDB table 
and extract one attribtue (the primary key in this example), which are then saved in a CSV file. 
This project is a simple example to show how to interact with AWS DynamoDB in Rust.


## Requirements

You just need to have Rust installed on your machine and these Env variables set:

```bash
AWS_PROFILE=default;
DYNAMODB_PRIMARY_KEY_NAME=someKey;
DYNAMODB_TABLE_NAME=someTable;
```