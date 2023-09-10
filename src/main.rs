mod convert_section;

use convert_section::convert_section;
use std::collections::HashMap;

use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// check for arguments
	let args: Vec<String> = env::args().collect();
	if args.len() != 3 {
		eprintln!(
			"Usage: {} <input_json_file> <inheritance_files_folder>",
			args[0]
		);
		std::process::exit(1);
	}

	// extract the input JSON file path from the command-line arguments
	let input_json_path = &args[1];
	let inheritance_files_path = Path::new(&args[2]);

	// read JSON file
	let mut json_file = File::open(input_json_path)?;
	let mut json_content = String::new();
	json_file.read_to_string(&mut json_content)?;

	// parse JSON
	let json_value: Value = serde_json::from_str(&json_content)?;
	let Value::Object(json_obj) = json_value else { todo!() };
	let mut unused_data = HashMap::new();

	// convert JSON object to INI-like content
	let Ok(ini_content) = convert_section(json_obj, inheritance_files_path, false, &mut unused_data) else { todo!() };

	let mut file = File::create("output.ini")?;
	file
		.write_all(ini_content.as_bytes())
		.expect("TODO: panic message");

	println!("Conversion successful!");

	Ok(())
}
