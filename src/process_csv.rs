
use std::thread::sleep;
use std::time::Duration;
use std::fs;
use std::path::Path;
use std::error::Error;
use crate::validate_yaml::validate_yaml;

// Function to process all CSV files in a given directory (same as before)
pub fn process_csv(input_directory: &str, output_directory: &str) -> Result<(), Box<dyn Error>> {
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

