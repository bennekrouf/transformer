use crate::models::Output;
// use serde::Deserialize;
use serde_yaml;
use std::fs;

// Function to read, validate YAML file, and display result
pub fn validate_yaml(file_path: &str) {
    // Read the YAML file
    let yaml_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Unable to read the file: {}", e);
            return;
        }
    };

    // Validate YAML
    match serde_yaml::from_str::<Output>(&yaml_content) {
        Ok(data) => {
            // println!("YAML is valid. Parsed data: {:?}", data);
            println!("{} is valid", &file_path);
        }
        Err(e) => {
            eprintln!("YAML is invalid. Error: {}", e);
        }
    }
}
