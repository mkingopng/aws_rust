# package the compiled lambda function into a zip file
# just the same as you would with a python lambda

cd target/lambda/aws_rust
zip lambda_function.zip bootstrap


# deploy the new lambda function using the AWS CLI
cd target/lambda/aws_rust
aws lambda create-function --function-name rust-lambda \
    --runtime provided.al2 --role arn:aws:iam::001499655372:role/Rust-vs-Python-project \
    --handler bootstrap --zip-file fileb://lambda_function.zip


# update
cd target/lambda/aws_rust
aws lambda update-function-code --function-name rust-lambda --zip-file fileb://lambda_function.zip


# invoke the lambda function
aws lambda invoke --function-name rust-lambda output.json
cat output.json
