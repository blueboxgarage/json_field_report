# JSON Field Report

A tool to compare JSON structures and identify missing fields between them.

## Features

- Compare JSON structures from a CSV file
- Generate field reports showing which fields are missing from each JSON
- Web UI for interactive comparison and report generation
- Support for multiple JSON comparisons

## Usage

### CLI Usage

1. Update the JSON data in `json_to_compare.csv`
2. Run the program:
   ```
   cargo run
   ```
3. View the generated report in `field_report.txt`

### Web UI Usage

Run the simple web interface:
```
./serve.sh
```

This will start a web server on http://localhost:8080 where you can:
- View and edit JSON data
- Generate comparison reports
- See the differences in a table format

## Development Notes

### JSON Format
- JSON strings in CSV expect escaped double quotes (e.g., `{"name":"value"}` becomes `{""name"":""value""}`)

### Project Components
- CLI tool written in Rust for batch processing
- Simple web interface with JavaScript for interactive use
- Core comparison logic implemented in both Rust and JavaScript
