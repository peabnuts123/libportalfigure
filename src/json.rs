use crate::common::{FigureData, OUTPUT_DIR};

pub fn emit(all_figure_data: &Vec<FigureData>, version: &str) {
    let out_file_name = format!("{}/portal-figure_v{version}.json", OUTPUT_DIR);

    println!("Writing JSON artifact: '{out_file_name}'");
    let mut file = std::fs::File::create(&out_file_name)
        .unwrap_or_else(|_| panic!("Failed to create output file: '{}'", out_file_name));

    serde_json::to_writer(&mut file, &all_figure_data).expect("Failed to write JSON data");
}
