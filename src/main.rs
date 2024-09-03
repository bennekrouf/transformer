
mod models;
mod validate_yaml;
mod generate_yml_endpoints;
mod generate_yml_from_csv;
mod process_csv;
mod transformer_service;

use std::env;
use std::path::Path;
use tokio::sync::Mutex;
use std::sync::Arc;
use tonic::transport::Server;
use crate::transformer_service::generated::transformer_service_server::TransformerServiceServer;
use dotenvy::from_path;
use messengerc::{connect_to_messenger_service, MessagingService};
use tonic_reflection::server::Builder;
use crate::transformer_service::MyTransformerService;

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

