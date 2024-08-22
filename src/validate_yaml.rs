
use serde_yaml;
use std::fs;
use crate::models::Entity;

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
    match serde_yaml::from_str::<Entity>(&yaml_content) {
        Ok(_data) => {
            // Optionally, you can inspect the parsed data to verify correctness
            // println!("YAML is valid. Parsed data: {:?}", data);
            println!("The YAML file '{}' is valid.", file_path);
        }
        Err(e) => {
            eprintln!("YAML is invalid. Error: {}", e);
        }
    }
}

