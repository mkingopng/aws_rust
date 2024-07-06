// import requirements (crates and modules)
use lambda_runtime::{handler_fn, Context, Error};  // AWS Lambda runtime for handling Lambda functions
use serde_json::json;  // Serde for JSON serialization/deserialization
use aws_sdk_s3::Client as S3Client;  // AWS SDK client for S3
use aws_sdk_dynamodb::Client as DynamoDbClient;  // AWS SDK client for DynamoDB
use aws_sdk_dynamodb::types::AttributeValue; // AWS SDK types for DynamoDB attribute values
use std::env;  // for accessing environment variables
use uuid::Uuid;  // to generate UUIDs


#[tokio::main]  // the main function is the entry point of the Lambda function
async fn main() -> Result<(), Error> {  // main function returns a Result or an Error
    let func = handler_fn(my_handler);  // create handler function
    lambda_runtime::run(func).await?;  // run Lambda runtime with handler function
    Ok(())  // return Ok if successful, else return Error
}

// the handler function that is executed when the Lambda function is invoked
async fn my_handler(_: serde_json::Value, _: Context) -> Result<serde_json::Value, Error> {
    // read env vars for S3 bucket name and DynamoDB table name
    let s3_bucket_name = env::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAME not set");  // expect() panics if the variable is not set
    let dynamo_table_name = env::var("DYNAMODB_TABLE").expect("DYNAMODB_TABLE not set");  // expect() panics if the variable is not set
    let config = aws_config::from_env().load().await;  // load AWS config from env
    let s3_client = S3Client::new(&config);  // create S3 and DynamoDB clients using the loaded config
    let dynamo_client = DynamoDbClient::new(&config);
    let guid = Uuid::new_v4().to_string();  // generate new UUID
    let file_name = format!("{}.txt", guid); // create file name using UUID
    let encoded_string = guid.clone().into_bytes();  // encode UUID as bytes

    // put the encoded UUID into the S3 bucket
    s3_client.put_object().bucket(&s3_bucket_name).key(&file_name).body(encoded_string.into()).send().await?;  // send the request to S3 and wait for the response

    // put the UUID into the DynamoDB table
    dynamo_client.put_item()
        .table_name(&dynamo_table_name)  // specify the table name
        .item("id", AttributeValue::S(guid.clone()))  // specify the item to be inserted
        .send()  // send the request
        .await?;  // wait for the response

    Ok(json!({"id": guid}))  // return a JSON response with the UUID
}