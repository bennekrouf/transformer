mod models;
mod validate_yaml;
mod generate_yml_endpoints;
mod generate_yml_from_csv;
use std::thread::sleep;
use std::time::Duration;
use crate::generate_yml_from_csv::process_csv;
use crate::validate_yaml::validate_yaml;
use std::error::Error;
use std::fs;
use std::path::Path;

// Function to process all CSV files in a given directory
fn process_csv_files_in_directory(input_directory: &str, output_directory: &str) -> Result<(), Box<dyn Error>> {
    // Read all entries in the input directory
    let entries = fs::read_dir(input_directory)?;

    // Process each CSV file
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a file with a `.csv` extension
        if path.is_file() && path.extension().map_or(false, |ext| ext == "csv") {
            let file_name = path.file_stem().unwrap_or_default().to_str().unwrap_or_default();
            let output_file_path = Path::new(output_directory).join(format!("{}.yml", file_name));

            // Process the CSV file and generate YAML
            println!("Processing file: {:?}", path);
            process_csv(path.to_str().unwrap(), output_directory)?;

            // Add a loop to wait until the file exists
            let mut retries = 0;
            while !fs::metadata(&output_file_path).is_ok() && retries < 10 {
                sleep(Duration::from_secs(1));
                retries += 1;
            }

            // Validate the generated YAML file
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

fn main() -> Result<(), Box<dyn Error>> {
    let input_directory = "csv";  // Directory containing CSV files
    let output_directory = "generated";  // Directory to save generated YAML files

    // Ensure the output directory exists
    fs::create_dir_all(output_directory)?;

    // Process and validate all CSV files in the input directory
    process_csv_files_in_directory(input_directory, output_directory)?;

    Ok(())
}

