use crate::common::{FigureData, OUTPUT_DIR};

pub fn emit(all_figure_data: &Vec<FigureData>) {
    let crate_dir = format!("{OUTPUT_DIR}/rust");
    let src_dir = format!("{crate_dir}/src");

    println!("Writing Rust crate artifact: '{crate_dir}'");

    // Create directory structure
    std::fs::create_dir_all(&src_dir).expect("Failed to create rust crate directories");

    // Config and utils
    let exported_vec_name = "PORTAL_FIGURES";
    let figure_data_to_export_name =
        |figure_data: &FigureData| figure_data.name_id.to_uppercase();

    // Cargo.toml
    let cargo_toml_content = r#"[package]
name = "portal-figure"
version = "0.1.0"
edition = "2024"

[dependencies]
"#;
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
}
