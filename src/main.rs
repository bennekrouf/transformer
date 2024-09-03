pub mod generated {
    tonic::include_proto!("transformer");
}
mod models;
mod validate_yaml;
mod generate_yml_endpoints;
mod generate_yml_from_csv;

use std::thread::sleep;
use std::time::Duration;
use std::fs;
use std::env;
use std::path::Path;
use std::error::Error;
use tokio::sync::Mutex;
use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};
use generated::transformer_service_server::{TransformerService, TransformerServiceServer};
use generated::{ProcessCsvFilesRequest, ProcessCsvFilesResponse};
use dotenvy::from_path;
use messengerc::{connect_to_messenger_service, MessagingService};
use tonic_reflection::server::Builder;
use crate::generate_yml_from_csv::process_csv;
use crate::validate_yaml::validate_yaml;
pub struct MyTransformerService;

#[tonic::async_trait]
impl TransformerService for MyTransformerService {
    async fn process_csv_files(
        &self,
        request: Request<ProcessCsvFilesRequest>,
    ) -> Result<Response<ProcessCsvFilesResponse>, Status> {
        let req = request.into_inner();

        let input_directory = req.input_directory;
        let output_directory = req.output_directory;

        // Ensure the output directory exists
        if let Err(e) = fs::create_dir_all(&output_directory) {
            return Ok(Response::new(ProcessCsvFilesResponse {
                message: format!("Failed to create output directory: {}", e),
                success: false,
            }));
        }

        // Process and validate all CSV files in the input directory
        match process_csv_files_in_directory(&input_directory, &output_directory) {
            Ok(_) => Ok(Response::new(ProcessCsvFilesResponse {
                message: "CSV files processed successfully.".to_string(),
                success: true,
            })),
            Err(e) => Ok(Response::new(ProcessCsvFilesResponse {
                message: format!("Error processing CSV files: {}", e),
                success: false,
            })),
        }
    }
}

// Function to process all CSV files in a given directory (same as before)
fn process_csv_files_in_directory(input_directory: &str, output_directory: &str) -> Result<(), Box<dyn Error>> {
    let entries = fs::read_dir(input_directory)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "csv") {
            let file_name = path.file_stem().unwrap_or_default().to_str().unwrap_or_default();
            let output_file_path = Path::new(output_directory).join(format!("{}.yml", file_name));

            println!("Processing file: {:?}", path);
            process_csv(path.to_str().unwrap(), output_directory)?;

            let mut retries = 0;
            while !fs::metadata(&output_file_path).is_ok() && retries < 10 {
                sleep(Duration::from_secs(1));
                retries += 1;
            }

            println!("Validating YAML file: {:?}", output_file_path);
            if fs::metadata(&output_file_path).is_ok() {
                validate_yaml(output_file_path.to_str().unwrap());
            } else {
                eprintln!("YAML file not found: {:?}", output_file_path);
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    let custom_env_path = Path::new("proto-definitions/.service");
    from_path(custom_env_path).expect("Failed to load environment variables from custom path");

    let ip = env::var("TRANSFORMER_DOMAIN").expect("Missing 'domain' environment variable");
    let port = env::var("TRANSFORMER_PORT").expect("Missing 'port' environment variable");
    let addr = format!("{}:{}", ip, port).parse().unwrap();

    let tag = env::var("TRANSFORMER_TAG").expect("Missing 'tag' environment variable");

    let messenger_client = connect_to_messenger_service().await
        .ok_or("Failed to connect to messenger service")?;

    let messaging_service = MessagingService::new(
        Arc::new(Mutex::new(messenger_client)),
        tag.clone(),
    );

    let mes = format!("Transformer listening on {:?}", &addr);
    let _ = messaging_service.publish_message(mes.to_string(), Some(vec![tag])).await;

    let transformer_service = MyTransformerService {};

    // Include the descriptor set for reflection
    let descriptor_set = include_bytes!(concat!(env!("OUT_DIR"), "/transformer_descriptor.bin"));
    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(descriptor_set)
        .build_v1()?;

    // Build and start the gRPC server
    Server::builder()
        .add_service(TransformerServiceServer::new(transformer_service))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}

