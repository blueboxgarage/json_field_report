use anyhow::{Context, Result};
use csv::{ReaderBuilder, StringRecord};
use serde_json::Value;
use std::collections::HashSet;
use std::fs::File;
use std::path::Path;

fn main() -> Result<()> {
    let csv_path = "json_to_compare.csv";
    process_csv(csv_path)?;
    Ok(())
}

fn process_csv(path: impl AsRef<Path>) -> Result<()> {
    let file = File::open(path).context("Failed to open CSV file")?;
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    
    println!("Request_ID,Field,Missing_From");
    
    for result in reader.records() {
        let record = result.context("Failed to read CSV record")?;
        process_record(&record)?;
    }
    
    Ok(())
}

fn process_record(record: &StringRecord) -> Result<()> {
    let request_id = &record[0];
    let json1_str = &record[1];
    let json2_str = &record[2];
    
    let json1: Value = serde_json::from_str(json1_str)
        .context("Failed to parse first JSON")?;
    let json2: Value = serde_json::from_str(json2_str)
        .context("Failed to parse second JSON")?;
    
    // Extract structure only (field paths) without considering values
    let fields1 = extract_field_paths(&json1);
    let fields2 = extract_field_paths(&json2);
    
    // Report fields in json1 but missing in json2
    for field in fields1.difference(&fields2) {
        println!("{},{},json2", request_id, field);
    }
    
    // Report fields in json2 but missing in json1
    for field in fields2.difference(&fields1) {
        println!("{},{},json1", request_id, field);
    }
    
    Ok(())
}

fn extract_field_paths(json: &Value) -> HashSet<String> {
    let mut paths = HashSet::new();
    extract_paths_recursive(json, "", &mut paths);
    paths
}

fn extract_paths_recursive(value: &Value, prefix: &str, paths: &mut HashSet<String>) {
    match value {
        Value::Object(map) => {
            // For empty objects, add the current path
            if map.is_empty() {
                paths.insert(prefix.to_string());
            }
            
            // Process each field in the object
            for (key, val) in map {
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };
                
                // Add the current field path
                paths.insert(new_prefix.clone());
                
                // Continue recursively for nested structures
                extract_paths_recursive(val, &new_prefix, paths);
            }
        }
        Value::Array(arr) => {
            // For empty arrays, add the current path
            if arr.is_empty() {
                paths.insert(prefix.to_string());
            } else {
                // We don't need to track array indices for structure comparison
                // Just check if there's a consistent structure in the first element
                if !arr.is_empty() {
                    let array_type_path = format!("{}[]", prefix);
                    paths.insert(array_type_path.clone());
                    extract_paths_recursive(&arr[0], &array_type_path, paths);
                }
            }
        }
        // For primitive values, we've already added their path when processing their parent
        _ => {}
    }
}
