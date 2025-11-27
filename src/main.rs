use glob::glob;

use crate::common::{FigureData, INPUT_GLOB, OUTPUT_DIR, RawFigureData, UNIMPLEMENTED_PREFIX};

mod common;
mod javascript;
mod json;
mod rust;
mod sqlite;

fn main() {
    // Clean output directory
    std::fs::remove_dir_all(OUTPUT_DIR).ok();

    // Read version
    let version = std::fs::read_to_string("VERSION").expect("Failed to read VERSION file");
    let version = version.trim();
    println!("Version: {}", version);

    // Parse every TOML file in src
    println!("Scanning TOML files...");
    let mut all_figure_data: Vec<FigureData> = Vec::new();
    for entry in glob(INPUT_GLOB).expect("Failed to read glob pattern") {
        match entry {
            Err(e) => panic!("Error reading file: {:?}", e),
            Ok(path) => {
                if path.starts_with(UNIMPLEMENTED_PREFIX) {
                    println!("Skipping unimplemented file: {path:?}");
                    continue;
                }

                println!("Processing file: {:?}", path);

                // Parse TOML
                let content = std::fs::read_to_string(&path).expect("Failed to read file content");
                let data: RawFigureData = toml::from_str(&content)
                    .unwrap_or_else(|err| panic!("Failed to parse TOML. (path='{path:?}') {err}"));

                let file_path = path.to_str().unwrap().to_string();

                // Validate data
                // - Ensure required fields are present
                // let figure_id = data.figure_id.expect("Missing figure_id");
                let figure_id = data
                    .figure_id
                    .unwrap_or_else(|| panic!("Missing required field 'figure_id': {file_path}"));
                let variant_id = data
                    .variant_id
                    .unwrap_or_else(|| panic!("Missing required field 'variant_id': {file_path}"));
                let name = data
                    .name
                    .unwrap_or_else(|| panic!("Missing required field 'name': {file_path}"));
                // - Ensure all figure/variant ID combinations are unique
                let duplicate_id_entry = all_figure_data
                    .iter()
                    .find(|fig| fig.figure_id == figure_id && fig.variant_id == variant_id);
                if let Some(duplicate) = duplicate_id_entry {
                    panic!(
                        "Duplicate `figure_id` and `variant_id` found: (figure_id='{figure_id}') (variant_id='{variant_id}') (file_a='{}') (file_b='{}')",
                        duplicate.file_path, file_path
                    );
                }

                // Unique ID derived from `name`
                let name_id = name
                    .replace(' ', "_")
                    .chars()
                    .filter(|c| c.is_alphanumeric() || *c == '_')
                    .collect::<String>()
                    .to_lowercase();
                // - Ensure all name IDs are unique
                let duplicate_name_entry =
                    all_figure_data.iter().find(|fig| fig.name_id == name_id);
                if let Some(duplicate) = duplicate_name_entry {
                    panic!(
                        "Name '{name}' from file '{file_path}' is not unique enough to generate unique name_id. Both files produced unique name_id '{name_id}'. Duplicate: (name='{}') (path='{}')",
                        duplicate.name, duplicate.file_path
                    );
                }

                // @TODO version
                all_figure_data.push(FigureData {
                    figure_id,
                    variant_id,
                    name,
                    name_id,
                    file_path,
                });
            }
        }
    }
    println!("Parsed {} files", all_figure_data.len());

    // Create output directory
    std::fs::create_dir_all(OUTPUT_DIR).expect("Failed to create output directory");

    // Write artifacts
    javascript::emit(&all_figure_data, &version);
    json::emit(&all_figure_data, &version);
    rust::emit(&all_figure_data, &version);
    sqlite::emit(&all_figure_data, &version);
}
