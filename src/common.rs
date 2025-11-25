use serde::{Deserialize, Serialize};

/// Raw figure data read from a source file.
/// Everything is optional since we can't make any guarantees about the contents of the file.
/// This data must be validated before use.
#[derive(Deserialize)]
pub struct RawFigureData {
    pub figure_id: Option<u16>,
    pub variant_id: Option<u16>,
    pub name: Option<String>,
}

/// Parsed and validated figure data from a source file.
/// Generally created from a `RawFigureData`.
#[derive(Serialize)]
pub struct FigureData {
    /// Figure ID (e.g. Spyro, Snap Shot, Trigger Happy, etc.)
    pub figure_id: u16,
    /// Variant ID (e.g. lightcore, dark, etc.)
    pub variant_id: u16,
    /// Figure name.
    pub name: String,
    /// A unique slug derived from the figure name.
    #[serde(skip_serializing)]
    pub name_id: String,
    /// Path of the file from which this data was parsed, relative to the project root.
    #[serde(skip_serializing)]
    pub file_path: String,
}

/// Glob pattern for all input files.
pub const INPUT_GLOB: &str = "src/**/*.toml";
/// Destination for built artifacts.
pub const OUTPUT_DIR: &str = "dist";
/// Path prefix for source files that will be ignored.
pub const UNIMPLEMENTED_PREFIX: &str = "src/00-unimplemented/";
