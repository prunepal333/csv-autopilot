use std::fs;
use csv::{StringRecord};
use serde_json::Result;
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
struct CSVConfig{
    delimiter: String,
    has_header: bool,
    header_map: Vec<String>,
    path: String,
}
fn main() -> Result<()>{
    let contents = fs::read_to_string("autopilot.json").expect("Failed to read configuration parameters");
    // println!("{}", contents);
    
    let config: (CSVConfig, CSVConfig) = serde_json::from_str(&contents).expect("Failed to parse json config");

    let mut rdr = csv::ReaderBuilder::new()
                .delimiter(config.0.delimiter.as_bytes()[0])
                .has_headers(config.0.has_header)
                .from_path(&config.0.path).expect("expected valid input configuration parameter");
    
    let mut wtr = csv::WriterBuilder::new()
                .delimiter(config.1.delimiter.as_bytes()[0])
                .has_headers(config.1.has_header)
                .from_path(&config.1.path).expect("expected valid output configuration parameter");
    
    if config.1.has_header {
        wtr.write_record(config.1.header_map).unwrap_or_else(|err| {
            println!("Failed to write record: {:?}", err);
            std::process::exit(1);
        });
    }
    for result in rdr.records() {
        // println!("{:?}", result);
        let result: StringRecord = result.unwrap();
        wtr.write_record(&result).unwrap();
    }
    wtr.flush().unwrap_or_else(|err| {
        println!("Failed to flush stream to output: {:?}", err);
        std::process::exit(1);
    });
    println!("Your file has been created and saved to {:?}", &config.1.path);
    Ok(())
}
