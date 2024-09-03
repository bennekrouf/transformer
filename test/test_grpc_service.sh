#!/bin/bash

# Variables
GRPC_SERVER="localhost:50055"  # Replace with your server address
PROTO_FILE="proto-definitions/transformer.proto"  # Path to your .proto file
SERVICE_NAME="transformer.TransformerService"  # Full service name (package name + service name)
METHOD="ProcessCsvFiles"  # RPC method to call

# Input arguments for the gRPC method
INPUT_DIRECTORY="csv"
OUTPUT_DIRECTORY="generated"

# Send the gRPC request using grpcurl
grpcurl -plaintext -d "{
  \"input_directory\": \"$INPUT_DIRECTORY\",
  \"output_directory\": \"$OUTPUT_DIRECTORY\"
}" -import-path . -proto $PROTO_FILE $GRPC_SERVER $SERVICE_NAME/$METHOD
