
use csv::ReaderBuilder;
use serde_yaml::to_writer;
use std::collections::HashSet;
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;

use crate::models::{Field, Entity, Property};
use crate::generate_yml_endpoints::generate_endpoints;

pub fn process_csv(input_file: &str, output_folder: &str) -> Result<(), Box<dyn Error>> {
    // Extract the file name without extension
    let input_path = Path::new(input_file);
    let file_stem = input_path.file_stem().unwrap_or_default().to_str().unwrap_or_default();
    
    // Determine the domain word from the file name
    let domain_word = if file_stem.ends_with('s') {
        &file_stem[..file_stem.len() - 1] // Handle plural by removing the trailing 's'
    } else {
        file_stem
    };

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

    // Generate a list of endpoints using the extracted domain word
    let endpoints = generate_endpoints(domain_word);

    // Create the output structure
    let output = Entity { endpoints, fields };

    // Ensure the output folder exists
    fs::create_dir_all(output_folder)?;

    // Generate the output file path using the file name
    let output_file_name = format!("{}.yml", file_stem);
    let output_path = Path::new(output_folder).join(output_file_name);
    let mut output_file = File::create(output_path)?;

    // Write to the YAML file
    to_writer(&mut output_file, &output)?;

    println!(
        "YAML file generated successfully in the folder: {}",
        output_folder
    );
    Ok(())
}

