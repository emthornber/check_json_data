extern crate clap;
extern crate jsonschema;
extern crate serde_json;

use clap::{App, Arg};
use jsonschema::{CompilationError, Draft, JSONSchema};
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::path::Path;

struct Params {
    schema_file: String,
    layout_file: String,
}

fn main() {
    let params: Params = validate_opts().unwrap();
    let mut schema_file = File::open(params.schema_file).unwrap();
    let mut json_schema_string = String::new();
    schema_file.read_to_string(&mut json_schema_string).unwrap();

    let mut data_file = File::open(params.layout_file).unwrap();
    let mut json_data_string = String::new();
    data_file.read_to_string(&mut json_data_string).unwrap();

    validate_json(json_schema_string, json_data_string).unwrap();
}

fn validate_json(
    json_schema: String,
    json_data: String,
) -> std::result::Result<(), CompilationError> {
    let schema: Value = serde_json::from_str(&json_schema).unwrap();
    //    eprintln!("{:?}", schema);
    let instance: Value = serde_json::from_str(&json_data).unwrap();
    //    eprintln!("{:?}", instance);
    let compiled = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)?;
    let result = compiled.validate(&instance);
    if let Err(errors) = result {
        for error in errors {
            println!("Validation error: {:?}", error)
        }
    }
    Ok(())
}

fn validate_opts() -> clap::Result<Params> {
    let matches = App::new("Check Layout JSON data")
        .version("0.1.0")
        .author("Mark Thornber <enchanted.systems@btinternet.com>")
        .about("Validate layout definitions encoded in JSON")
        .arg(
            Arg::with_name("schema")
                .short("s")
                .long("schema")
                .value_name("FILE")
                .help("Defines the JSON schema to use")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("LAYOUT")
                .help("Sets the layout data file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Gets a value for schema file, if supplied by user, or defaults to "data/CanWebElmSchema.json"
    let schema = matches
        .value_of("schema")
        .unwrap_or("data/CanWebElmSchema.json");
    let schema_metadata = Path::new(&schema)
        .metadata()
        .expect("Cannot read schema file");
    if !schema_metadata.is_file() {
        panic!("{} is not a file", schema);
    };
    let layout = matches.value_of("LAYOUT").unwrap();
    let layout_metadata = Path::new(&layout)
        .metadata()
        .expect("Cannot read layout file");
    if !layout_metadata.is_file() {
        panic!("{} is not a file", layout);
    };
    Ok(Params {
        schema_file: String::from(schema),
        layout_file: String::from(layout),
    })
}
