extern crate jsonschema;
extern crate serde_json;

use jsonschema::{CompilationError, Draft, JSONSchema};
use serde_json::json;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), CompilationError> {
    let mut schema_file =
        File::open("/home/thornbem/Work/Elm/CanWebElm/CanWebElmSchema.json").unwrap();
    let mut json_schema_string = String::new();
    schema_file.read_to_string(&mut json_schema_string).unwrap();

    let mut data_file = File::open("/home/thornbem/Work/Elm/CanWebElm/cagdemo.json").unwrap();
    let mut json_data_string = String::new();
    data_file.read_to_string(&mut json_data_string).unwrap();

    let schema = json!(json_schema_string);
    eprintln!("{:?}", schema);
    let instance = json!(json_data_string);
    let compiled = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .unwrap();
    let result = compiled.validate(&instance);
    if let Err(errors) = result {
        for error in errors {
            println!("Validation error: {:?}", error)
        }
    }
    Ok(())
}
