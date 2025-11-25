use glob::glob;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct RawFigureData {
    figure_id: Option<u16>,
    variant_id: Option<u16>,
    name: Option<String>,
}

#[derive(Serialize)]
struct FigureData {
    figure_id: u16,
    variant_id: u16,
    name: String,
}

/// Glob pattern for all input files
const INPUT_GLOB: &str = "src/**/*.toml";
/// Destination for built artifacts
const OUTPUT_DIR: &str = "dist";

fn main() {
    // Clean output directory
    std::fs::remove_dir_all(OUTPUT_DIR).ok();

    // Parse every TOML file in src
    println!("Scanning TOML files...");
    let mut all_figure_data: Vec<FigureData> = Vec::new();
    for entry in glob(INPUT_GLOB).expect("Failed to read glob pattern") {
        match entry {
            Err(e) => panic!("Error reading file: {:?}", e),
            Ok(path) => {
                println!("Processing file: {:?}", path);

                // Parse TOML
                let content = std::fs::read_to_string(&path).expect("Failed to read file content");
                let data: RawFigureData = toml::from_str(&content).expect("Failed to parse TOML");

                // Validate data
                // - Ensure required fields are present
                let figure_id = data.figure_id.expect("Missing figure_id");
                let variant_id = data.variant_id.expect("Missing variant_id");
                let name = data.name.expect("Missing name");
                // - Ensure all figure/variant ID combinations are unique
                if all_figure_data
                    .iter()
                    .any(|fig| fig.figure_id == figure_id && fig.variant_id == variant_id)
                {
                    panic!(
                        "Duplicate `figure_id` and `variant_id` found: figure_id='{}', variant_id='{}'",
                        figure_id, variant_id
                    );
                }

                all_figure_data.push(FigureData {
                    figure_id,
                    variant_id,
                    name,
                });
            }
        }
    }
    println!("Parsed {} files", all_figure_data.len());

    // Create output directory
    std::fs::create_dir_all(OUTPUT_DIR).expect("Failed to create output directory");

    // Write artifacts
    write_json(&all_figure_data);
}

fn write_json(all_figure_data: &Vec<FigureData>) {
    let out_file_name = format!("{}/figure_data.json", OUTPUT_DIR);

    println!("Writing JSON artifact: '{out_file_name}'");
    let mut file = std::fs::File::create(&out_file_name)
        .unwrap_or_else(|_| panic!("Failed to create output file: {}", out_file_name));

    serde_json::to_writer(&mut file, &all_figure_data).expect("Failed to write JSON data");
}
