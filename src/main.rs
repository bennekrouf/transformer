mod generate_yml_from_csv; 
mod models;
mod validate_yaml;

use crate::validate_yaml::validate_yaml;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = "orders.csv";
    let output_folder = "generated";

    validate_yaml(file_path);
    process_csv(input_file, output_folder)
}
