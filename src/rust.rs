use crate::common::{FigureData, OUTPUT_DIR};

pub fn emit(all_figure_data: &Vec<FigureData>, version: &str) {
    let crate_dir = format!("{OUTPUT_DIR}/rust");
    let src_dir = format!("{crate_dir}/src");

    println!("Writing Rust crate artifact: '{crate_dir}'");

    // Create directory structure
    std::fs::create_dir_all(&src_dir).expect("Failed to create rust crate directories");

    // Config and utils
    let exported_vec_name = "PORTAL_FIGURES";
    let figure_data_to_export_name = |figure_data: &FigureData| figure_data.name_id.to_uppercase();

    // Cargo.toml
    let cargo_toml_content = format!(
        r#"[package]
name = "portal_figure"
description = "A library of data relating to figures from the Skylanders series of video games."
version = "{version}"
edition = "2024"
license = "CC0-1.0"

[dependencies]
"#
    );
    std::fs::write(format!("{crate_dir}/Cargo.toml"), cargo_toml_content)
        .expect("Failed to write Cargo.toml");

    // Rust code for individual named exports
    let specific_figure_rs = all_figure_data
        .iter()
        .map(|figure_data| {
            let export_name = figure_data_to_export_name(figure_data);
            let FigureData {
                figure_id,
                variant_id,
                name,
                ..
            } = figure_data;
            format!(
                r#"
pub const {export_name}: PortalFigure = PortalFigure {{
    figure_id: 0x{figure_id:x},
    variant_id: 0x{variant_id:x},
    name: "{name}",
}};
"#
            )
        })
        .collect::<Vec<String>>()
        .join("");

    // Rust code for slice containing all figures
    let all_export_names = all_figure_data
        .iter()
        .map(|figure_data| figure_data_to_export_name(figure_data))
        .collect::<Vec<String>>();
    let portal_figures_rs = format!(
        r#"pub const {exported_vec_name}: &[PortalFigure] = &[
    {},
];"#,
        all_export_names.join(",\n    ")
    );

    // Source code for lib.rs inside emitted rust crate artifact
    let lib_rs_content = format!(
        r#"pub struct PortalFigure {{
    pub figure_id: u16,
    pub variant_id: u16,
    pub name: &'static str,
}}

{specific_figure_rs}

{portal_figures_rs}

pub fn find_figure(figure_id: u16, variant_id: u16) -> Option<&'static PortalFigure> {{
    for figure in {exported_vec_name} {{
        if figure.figure_id == figure_id && figure.variant_id == variant_id {{
            return Some(figure);
        }}
    }}
    None
}}
"#
    );
    std::fs::write(format!("{}/lib.rs", src_dir), lib_rs_content).expect("Failed to write lib.rs");

    // README.md
    let readme_content = r#"# portal_figure

A library of data relating to figures from the Skylanders series of video games.

This crate is built from source: https://github.com/peabnuts123/libportalfigure

## Installation

```sh
cargo add portal_figure
```

## Usage

```rust
use portal_figure::{PORTAL_FIGURES, PortalFigure, SPYRO, find_figure};

fn main() {
    // Use `find_figure()` to look up figure data by Figure ID + Variant ID
    let figure: &PortalFigure = find_figure(0x1ce, 0x3000).expect("Couldn't find figure");
    println!(
        "{} (figure_id='0x{:x}') (variant_id='0x{:x}')",
        figure.name, figure.figure_id, figure.variant_id
    );

    // Individually exported figure data
    println!(
        "Spyro (figure_id='0x{:x}') (variant_id='0x{:x}')",
        SPYRO.figure_id, SPYRO.variant_id
    );

    // `PORTAL_FIGURES` is a &[PortalFigure] of all figures
    let all_spyros = PORTAL_FIGURES
        .iter()
        .filter(|figure| figure.name.to_lowercase().contains("spyro"))
        .collect::<Vec<&PortalFigure>>();

    println!("All Spyro figures:");
    for figure in all_spyros {
        println!(
            "{} (figure_id='0x{:x}') (variant_id='0x{:x}')",
            figure.name, figure.figure_id, figure.variant_id
        );
    }
}
```
"#;
    std::fs::write(format!("{}/README.md", crate_dir), readme_content)
        .expect("Failed to write README.md");
}
