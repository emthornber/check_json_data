extern crate jsonschema;
extern crate serde_json;

use jsonschema::{CompilationError, Draft, JSONSchema};
use serde_json::Value;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut schema_file =
        File::open("/home/thornbem/Work/Elm/CanWebElm/CanWebElmSchema.json").unwrap();
    let mut json_schema_string = String::new();
    schema_file.read_to_string(&mut json_schema_string).unwrap();

    let mut data_file = File::open("/home/thornbem/Work/Elm/CanWebElm/holywelltown.json").unwrap();
    let mut json_data_string = String::new();
    data_file.read_to_string(&mut json_data_string).unwrap();

    validate_json(json_schema_string, json_data_string).unwrap();
}

fn validate_json(json_schema: String, json_data: String) -> Result<(), CompilationError> {
    let schema: Value = serde_json::from_str(&json_schema).unwrap();
    //    eprintln!("{:?}", schema);
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
