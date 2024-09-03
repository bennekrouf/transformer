pub mod generated {
    tonic::include_proto!("transformer");
}

use std::fs;
use tonic::{Request, Response, Status};
use generated::transformer_service_server::TransformerService;
use generated::{ProcessCsvFilesRequest, ProcessCsvFilesResponse};
use crate::process_csv::process_csv;

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
        match process_csv(&input_directory, &output_directory) {
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

