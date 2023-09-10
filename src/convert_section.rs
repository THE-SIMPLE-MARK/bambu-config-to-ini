use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

mod search_folder;
use search_folder::search_folder;

#[derive(Debug)]
pub enum SectionDataValue {
	Single(Value),
	Array(Vec<Value>),
}

type SectionDataMap = HashMap<String, SectionDataValue>;

pub fn convert_section<'a>(
	obj: Map<String, Value>,
	start_dir: &Path,
	is_recursive: bool,
	upstream_section_data: &mut SectionDataMap,
) -> Result<String, io::Error> {
	let mut section_data: HashMap<String, SectionDataValue> = HashMap::new();
	let mut section_title = String::new();

	// check if the section inherits from any other files
	if obj.contains_key("inherits") {
		let inherits = &Value::to_string(obj.get("inherits").unwrap());

		// search for that file
		let inherits_file = search_folder(
			start_dir,
			format!("{:?}.json", inherits)
				.replace("\\", "")
				.replace("\"", "")
				.as_str(),
		);

		if inherits_file != None {
			println!("File inherited: {:?}", inherits_file);

			// add to section_data so it doesn't get overwritten
			section_data
				.entry("inherits".parse().unwrap())
				.or_insert_with(|| SectionDataValue::Single(inherits.parse().unwrap()));

			// read JSON file
			let mut json_file = File::open(inherits_file.unwrap())?;
			let mut json_content = String::new();
			json_file
				.read_to_string(&mut json_content)
				.expect("TODO: panic message");

			// parse JSON
			let json_value: Value = serde_json::from_str(&json_content)?;
			let Value::Object(json_obj) = json_value else { todo!() };

			// convert JSON object to INI-like content
			let Ok(_) = convert_section(json_obj, start_dir, true, if is_recursive { upstream_section_data } else { &mut section_data }) else { todo!() };

			println!("Successfully converted recursive data!");
		} else {
			eprintln!("ERROR: Could not find file to be inherited: {:?}", inherits);
		}
	}

	// iterate through the JSON object and add the section data to section_data
	for (key, value) in &obj {
		if key == "type" {
			section_title = value.to_string();
		}

		match value {
			// handle arrays
			Value::Array(child_array) => {
				// if we are in recursive mode we want to merge the data to the parent function
				if is_recursive {
					upstream_section_data
						.entry(key.clone())
						.or_insert_with(|| SectionDataValue::Array(child_array.clone()))
				} else {
					section_data
						.entry(key.clone())
						.or_insert_with(|| SectionDataValue::Array(child_array.clone()))
				}
			}

			// handle other types (numbers, strings, etc.)
			_ => {
				if is_recursive {
					upstream_section_data
						.entry(key.clone())
						.or_insert_with(|| SectionDataValue::Single(value.clone()))
				} else {
					section_data
						.entry(key.clone())
						.or_insert_with(|| SectionDataValue::Single(value.clone()))
				}
			}
		};
	}

	let mut ini = String::new();
	if !is_recursive {
		if !section_title.is_empty() {
			ini.push_str(format!("[{}]\n", section_title).replace("\"", "").as_str());
		}

		for (key, value) in &section_data {
			println!("Key: {:?} Value: {:?}", key, value)
		}

		for (key, value) in section_data {
			match value {
				// handle arrays
				SectionDataValue::Array(child_array) => {
					ini.push_str(&format!("{} = ", key)); // sdd space after =
					for (i, item) in child_array.iter().enumerate() {
						let item_str = match item {
							Value::Number(num) => num.to_string(),
							Value::String(str_val) => {
								if str_val.contains(";") {
									format!("\"{}\"", str_val)
								} else {
									str_val.clone()
								}
							}
							_ => item.to_string(),
						};
						ini.push_str(&item_str);
						if i < child_array.len() - 1 {
							ini.push(',');
						}
					}
					ini.push('\n');
				}

				// handle other types (numbers, strings, etc.)
				SectionDataValue::Single(value) => {
					let value_str = &Value::to_string(&value);
					let value_str = value_str.trim_matches('"'); // remove double quotes from numbers
					if value_str.contains(";") {
						ini.push_str(&format!("{} = \"{}\"\n", key, value_str));
					// add space after = and enclose in double quotes
					} else {
						ini.push_str(&format!("{} = {}\n", key, value_str)); // add space after =
					}
				}
			}
		}
	}

	Ok(ini)
}
