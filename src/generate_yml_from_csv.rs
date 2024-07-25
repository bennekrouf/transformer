use csv::ReaderBuilder;
use serde_yaml::to_writer;
use std::collections::HashSet;
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;

use crate::models::{Endpoint, Field, Output, Property};

pub fn process_csv(input_file: &str, output_folder: &str) -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let file = File::open(input_file)?;
    let mut rdr = ReaderBuilder::new().flexible(true).from_reader(file);

    // Read the headers
    let headers = rdr.headers()?.clone();

    // Initialize data structures to store column values and properties
    let mut column_values: Vec<HashSet<String>> = vec![HashSet::new(); headers.len()];
    let mut fully_populated: Vec<bool> = vec![true; headers.len()];
    let mut is_number: Vec<bool> = vec![true; headers.len()];

    // Process each record
    for result in rdr.records() {
        let record = result?;
        for (i, field) in record.iter().enumerate() {
            let trimmed_field = field.trim();
            if !trimmed_field.is_empty() {
                column_values[i].insert(trimmed_field.to_string());
                // Check if the field is a number
                if is_number[i] && trimmed_field.parse::<f64>().is_err() {
                    is_number[i] = false;
                }
            } else {
                fully_populated[i] = false;
            }
        }
    }

    // Collect relevant field data
    // Collect relevant field data
    let mut fields = vec![];
    for (i, header) in headers.iter().enumerate() {
        if column_values[i].len() > 1 {
            let mut properties = vec![];
            if fully_populated[i] {
                properties.push(Property::Mandatory);
            }
            if is_number[i] {
                properties.push(Property::Number);
            }
            fields.push(Field {
                name: header.to_string(),
                properties,
            });
        }
    }
    // Generate a list of endpoints
    let endpoints = vec![
        Endpoint {
            path: "get_a_list_of_last_orders".to_string(),
            description: "Retrieve a list of last orders".to_string(),
        },
        Endpoint {
            path: "create_new_order".to_string(),
            description: "Create a new order".to_string(),
        },
        Endpoint {
            path: "update_order".to_string(),
            description: "Update an existing order".to_string(),
        },
        Endpoint {
            path: "delete_order".to_string(),
            description: "Delete an order".to_string(),
        },
    ];

    // Create the output structure
    let output = Output { endpoints, fields };

    // Ensure the output folder exists
    fs::create_dir_all(output_folder)?;

    // Generate the output file path
    let output_path = Path::new(output_folder).join("orders.yml");
    let mut output_file = File::create(output_path)?;

    // Write to the YAML file
    to_writer(&mut output_file, &output)?;

    println!(
        "YAML file generated successfully in the folder: {}",
        output_folder
    );
    Ok(())
}
