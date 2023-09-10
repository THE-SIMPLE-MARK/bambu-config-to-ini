use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::Path;

pub fn search_folder<'a>(start_dir: &Path, target_file_name: &str) -> Option<OsString> {
	if let Ok(entries) = fs::read_dir(start_dir) {
		for entry in entries.flatten() {
			let entry_path = entry.path();

			if entry_path.is_dir() {
				if let Some(result) = search_folder(&entry_path, target_file_name) {
					return Some(result);
				}
			} else if entry_path.file_name() == Option::from(OsStr::new(target_file_name)) {
				return Some(entry_path.into_os_string());
			}
		}
	}

	None
}
