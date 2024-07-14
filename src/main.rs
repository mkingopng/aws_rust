use lambda_runtime::{handler_fn, Context, Error};
use serde_json::json;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_dynamodb::Client as DynamoDbClient;
use aws_sdk_dynamodb::types::AttributeValue;
use std::env;
use uuid::Uuid;
use log::{info, error};
use simple_logger::SimpleLogger;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct LambdaEvent {
    #[serde(rename = "resource")]
    resource: Option<String>,
}

#[derive(Serialize)]
struct Response {
    statusCode: i32,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().init().unwrap();
    let func = handler_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn my_handler(event: LambdaEvent, _: Context) -> Result<Response, Error> {
    match event.resource.as_deref() {
        Some("/") => process_request().await,
        Some("/health") => Ok(health_check().await),
        _ => process_request().await,  // Default to process_request for testing purposes
    }
}

async fn process_request() -> Result<Response, Error> {
    let s3_bucket_name = env::var("S3_BUCKET_NAME").unwrap_or_else(|_| "".to_string());
    let dynamo_table_name = env::var("DYNAMODB_TABLE").unwrap_or_else(|_| "".to_string());

    if s3_bucket_name.is_empty() || dynamo_table_name.is_empty() {
        error!("Environment variables S3_BUCKET_NAME and DYNAMODB_TABLE must be set");
        return Ok(Response {
            statusCode: 500,
            body: json!({"error": "Missing environment variables"}).to_string(),
        });
    }

    let config = aws_config::load_from_env().await;
    let s3_client = S3Client::new(&config);
    let dynamo_client = DynamoDbClient::new(&config);

    let guid = Uuid::new_v4().to_string();
    let file_name = format!("{}.txt", guid);
    let encoded_string = guid.clone().into_bytes();

    match s3_client.put_object().bucket(&s3_bucket_name).key(&file_name).body(encoded_string.into()).send().await {
        Ok(_) => info!("Successfully put object in S3"),
        Err(e) => {
            error!("Failed to put object in S3: {}", e);
            return Ok(Response {
                statusCode: 500,
                body: json!({"error": "Failed to put object in S3"}).to_string(),
            });
        }
    };

    match dynamo_client.put_item().table_name(&dynamo_table_name).item("id", AttributeValue::S(guid.clone())).send().await {
        Ok(_) => info!("Successfully put item in DynamoDB"),
        Err(e) => {
            error!("Failed to put item in DynamoDB: {}", e);
            return Ok(Response {
                statusCode: 500,
                body: json!({"error": "Failed to put item in DynamoDB"}).to_string(),
            });
        }
    };

    Ok(Response {
        statusCode: 200,
        body: json!({"id": guid}).to_string(),
    })
}

async fn health_check() -> Response {
    Response {
        statusCode: 200,
        body: json!({"status": "healthy"}).to_string(),
    }
}
