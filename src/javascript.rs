use crate::common::{FigureData, OUTPUT_DIR};

pub fn emit(all_figure_data: &Vec<FigureData>, version: &str) {
    let package_dir = format!("{OUTPUT_DIR}/javascript");
    let src_dir = format!("{package_dir}/src");

    println!("Writing JavaScript package artifact: '{package_dir}'");

    // Create directory structure
    std::fs::create_dir_all(&src_dir).expect("Failed to create JavaScript package directories");

    // Config and utils
    let exported_array_name = "PortalFigures";
    let figure_data_to_export_name = |figure_data: &FigureData| {
        // Convert name_id to NameId (i.e. PascalCase)
        figure_data
            .name_id
            .split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<String>()
    };

    // Cargo.toml
    let package_json_content = format!(
        r#"{{
  "name": "portal-figure",
  "description": "A library of data relating to figures from the Skylanders series of video games.",
  "version": "{version}",
  "main": "src/index.js",
  "types": "src/index.d.ts",
  "keywords": [],
  "author": "peabnuts123",
  "license": "CC0-1.0",
  "type": "commonjs"
}}"#
    );
    std::fs::write(format!("{package_dir}/package.json"), package_json_content)
        .expect("Failed to write package.json");

    // JavaScript code for individual named exports
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
export const {export_name} = {{
  figureId: 0x{figure_id:x},
  variantId: 0x{variant_id:x},
  name: "{name}",
}};
"#
            )
        })
        .collect::<Vec<String>>()
        .join("");

    // JavaScript code for array containing all figures
    let all_export_names = all_figure_data
        .iter()
        .map(|figure_data| figure_data_to_export_name(figure_data))
        .collect::<Vec<String>>();
    let portal_figures_rs = format!(
        r#"export const {exported_array_name} = [
  {},
];"#,
        all_export_names.join(",\n  ")
    );

    // Source code for index.js inside emitted JavaScript package artifact
    let index_js_content = format!(
        r#"{specific_figure_rs}

{portal_figures_rs}

export function findFigure(figureId, variantId) {{
  for (const figure of {exported_array_name}) {{
    if (figure.figureId == figureId && figure.variantId == variantId) {{
      return figure;
    }}
  }}
  return undefined;
}}
"#
    );
    std::fs::write(format!("{}/index.js", src_dir), index_js_content)
        .expect("Failed to write index.js");

    let figure_type_name: &str = "PortalFigure";
    // TypeScript type definitions for individual named exports
    let specific_figure_dts = all_figure_data
        .iter()
        .map(|figure_data| {
            let export_name = figure_data_to_export_name(figure_data);
            format!("export const {export_name}: {figure_type_name};")
        })
        .collect::<Vec<String>>()
        .join("\n");

    // Source code for index.d.ts inside emitted JavaScript package artifact
    let index_dts_content = format!(
        r#"export interface {figure_type_name} {{
  figureId: number;
  variantId: number;
  name: string;
}}

{specific_figure_dts}

export const {exported_array_name}: readonly {figure_type_name}[];

export function findFigure(figureId: number, variantId: number): {figure_type_name} | undefined;
"#
    );
    std::fs::write(format!("{}/index.d.ts", src_dir), index_dts_content)
        .expect("Failed to write index.d.ts");

    // README.md
    let readme_content = r#"# portal-figure

A library of data relating to figures from the Skylanders series of video games.

This package is built from source: https://github.com/peabnuts123/libportalfigure

## Installation

```sh
npm install portal-figure
```

## Usage

```typescript
import { Spyro, findFigure, type PortalFigure, PortalFigures } from 'portal-figure';

// Use `findFigure()` to look up figure data by Figure ID + Variant ID
const figure: PortalFigure = findFigure(0x1ce, 0x3000)!;
console.log(`${figure.name} (figureId='0x${figure.figureId.toString(16)}') (variantId='0x${figure.variantId.toString(16)}')`);

// Individually exported figure data
console.log(`Spyro (figureId='0x${Spyro.figureId.toString(16)}') (variantId='0x${Spyro.variantId.toString(16)}')`);

// `PortalFigures` is an array of all figures
const allSpyros = PortalFigures.filter((figure) => figure.name.toLowerCase().includes("spyro"));

console.log("All Spyro figures:");
for (const figure of allSpyros) {
  console.log(`${figure.name} (figureId='0x${figure.figureId.toString(16)}') (variantId='0x${figure.variantId.toString(16)}')`);
}
```
"#;
    std::fs::write(format!("{}/README.md", package_dir), readme_content)
        .expect("Failed to write README.md");
}
