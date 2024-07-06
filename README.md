# AWS Lambda in Rust

This project demonstrates how to build and deploy an AWS Lambda function using Rust. 
The Lambda function generates a UUID, stores it in an S3 bucket and a DynamoDB table, and returns the UUID in a JSON response.

## Prerequisites

- Install [Rust](https://www.rust-lang.org/) and [Cargo](https://doc.rust-lang.org/cargo/).
- Install [AWS CLI](https://aws.amazon.com/cli/).

## Steps

### set up IAM role & permissions

install musl

### 1. install Rust & Cargo
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. install cargo-lambda, a tool that simplifies building and deploying AWS Lambda functions written in Rust

```bash
cargo install cargo-lambda
```

### 3. the cargo.toml file: 
this lists the dependencies for the project. Its very similar to the pyproject.toml you may use if you choose Poetry as 
your python dependency manager. It probably feels a little different if you're used to using pip or conda

### 4. the main.rs file: this is our lambda function, it performs the following functions
The AWS Lambda function written in Rust performs the following tasks:
    - **Generate a UUID**: create a unique identifier (UUID).
    - **Store Data in S3**: The UUID is converted to bytes and stored in an S3 bucket as a file with the UUID as the filename.
    - **Store Data in DynamoDB**: The UUID is also stored in a DynamoDB table as an item with the key "id" set to the UUID value.
    - **Return a JSON Response**: Finally, it returns a JSON response containing the UUID.
serves as an example of integrating AWS services (S3 and DynamoDB) using Rust, demonstrating basic operations like 
storing and retrieving data. Its not exciting, but serves as a basis for comparison

### 5. compile the lambda function
- make sure you specify --release to reduce the size of the binary
- Don't use --debug, it will make the binary too large

If you've compiled previously, i try to always clean first. I don't think its strictly necessary
```bash
cargo clean
```

### compile to binary
```bash
cargo lambda build --release --target x86_64-unknown-linux-musl
```

### 6. Strip: 
we can further reduce the size of the binary by stripping it
```bash
strip target/lambda/aws_rust/bootstrap
```

### 7. package the compiled lambda function into a zip file
just the same as you would with a python lambda

```bash
cd target/lambda/aws_rust
zip lambda_function.zip bootstrap
```

### 7. deploy the lambda function using the AWS CLI
new
```bash
cd target/lambda/aws_rust
aws lambda create-function --function-name rust-lambda \
    --runtime provided.al2 --role arn:aws:iam::001499655372:role/Rust-vs-Python-project \
    --handler bootstrap --zip-file fileb://lambda_function.zip
```

update
```bash
cd target/lambda/aws_rust
aws lambda update-function-code --function-name rust-lambda --zip-file fileb://lambda_function.zip
```

### 9. invoke the lambda function
```bash
aws lambda invoke --function-name rust-lambda output.json
```

check the response
```bash
cat output.json
```