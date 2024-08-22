mod generate_yml_from_csv;
mod models;
mod validate_yaml;
mod generate_yml_endpoints;

use crate::generate_yml_from_csv::process_csv;
use crate::validate_yaml::validate_yaml;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = "orders.csv";
    let output_folder = "generated";

    let file_path = "generated/orders.yml";
    let _ = process_csv(input_file, output_folder);
    validate_yaml(file_path);

    Ok(())
}
