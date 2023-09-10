# bambu-config-to-ini
A CLI for converting Bambu Studio config files to `.ini` for use in Prusa Slicer and similar software. It supports inheritance, so the entire configuration tree is converted directly into one `.ini` file.

### Note: This is a quick side project I made for a bigger project, so it is not exactly developed for production use.
If anyone would like to use this CLI tool with actual panic messages and an overall better experience then feel free to contribute, otherwise open an issue about what you'd like to be added/changed and I will do my best to implement that. 

Also this is literally my first complex Rust project so please don't hate on the code quality / techniques used :)

## Usage:
```
bambu-config-to-ini.exe <input_json_file> <inheritance_files_folder>
```
| Argument                 | Expected Data | Description                                                                                                |
|--------------------------|---------------|------------------------------------------------------------------------------------------------------------|
| input_json_file          | File Path     | The JSON config file to convert.                                                                           |
| inheritance_files_folder | Folder Path   | The folder to use when searching for the file input_json_file or any of its parents may inherit data from. |
