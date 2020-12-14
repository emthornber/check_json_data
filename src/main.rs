extern crate clap;
extern crate jsonschema;
extern crate serde_json;

use clap::{App, Arg};
use jsonschema::{CompilationError, Draft, JSONSchema};
use serde_json::Value;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::path::Path;
use std::process;

const DEFAULT_SCHEMA: &str = "data/CanWebElmSchema.json";
const SCHEMA_HELP: &str =
    "Defines JSON schema to use.  This is optional and defaults to 'daata/CanWebElmSchema";

struct Params {
    schema_file: String,
    layout_file: String,
}

fn main() {
    let params: Params = validate_opts().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {:?}", err);
        process::exit(1);
    });
    let json_schema_string = read_json_file(params.schema_file).unwrap_or_else(|err| {
        eprintln!("Cannot read JSON schema file {:?}", err);
        process::exit(1);
    });
    let json_data_string = read_json_file(params.layout_file).unwrap_or_else(|err| {
        eprintln!("Cannot read JSON data file {:?}", err);
        process::exit(1);
    });

    validate_json(json_schema_string, json_data_string).unwrap_or_else(|_| {
        eprintln!("JSON data validation failed");
        process::exit(1);
    });
}

fn read_json_file(json_file: String) -> io::Result<String> {
    let mut json_src = File::open(json_file)?;
    let mut json_str = String::new();
    json_src.read_to_string(&mut json_str)?;
    Ok(json_str)
}

fn validate_json(
    json_schema: String,
    json_data: String,
) -> std::result::Result<(), CompilationError> {
    let schema: Value = serde_json::from_str(&json_schema).unwrap();
    let instance: Value = serde_json::from_str(&json_data).unwrap();
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
    let mut app = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("schema")
                .short("s")
                .long("schema")
                .value_name("FILE")
                .help(SCHEMA_HELP)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("LAYOUT")
                .help("Sets the layout data file to validate")
                .required(true)
                .index(1),
        );
    let matches = app.clone().get_matches();

    // Gets a value for schema file, if supplied by user, or defaults to "data/CanWebElmSchema.json"
    let schema = matches.value_of("schema").unwrap_or(DEFAULT_SCHEMA);
    let schema_metadata = Path::new(&schema).metadata()?;
    //    let schema_metadata = match schema_metadata {
    //        Ok(file) => file,
    //        Err(e) => {
    //            if e.kind() == ErrorKind::NotFound {
    //                eprintln!("Schema file {} not found: {:?}", schema, e);
    //                return clap::Error(e);
    //            } else {
    //                panic!("Problem opening schema file {}: {:?}", schema, e);
    //            }
    //        }
    //    };
    if !schema_metadata.is_file() {
        eprintln!("{} is not a file", schema);
        app.print_long_help().unwrap();
        process::exit(1);
    };
    let layout = matches.value_of("LAYOUT").unwrap();
    let layout_metadata = Path::new(&layout).metadata();
    let layout_metadata = match layout_metadata {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Problem reading Layout file {}: {:?}", layout, e);
            return Err(e);
        }
    };
    if !layout_metadata.is_file() {
        eprintln!("{} is not a file", layout);
        app.print_long_help().unwrap();
        process::exit(1);
    };
    Ok(Params {
        schema_file: String::from(schema),
        layout_file: String::from(layout),
    })
}
