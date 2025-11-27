use rusqlite::Connection;

use crate::common::{FigureData, OUTPUT_DIR};

pub fn emit(all_figure_data: &Vec<FigureData>, version: &str) {
    let out_file_name = format!("{}/portal-figure_v{version}.db", OUTPUT_DIR);

    println!("Writing SQLite artifact: '{out_file_name}'");

    let conn = Connection::open(&out_file_name)
        .unwrap_or_else(|_| panic!("Failed to create database file: '{}'", out_file_name));

    // Create table
    // @TODO Should we store version somewhere in DB?
    conn.execute(
        "CREATE TABLE PortalFigure (
            figure_id INTEGER NOT NULL,
            variant_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            PRIMARY KEY (figure_id, variant_id),
            UNIQUE(name)
        )",
        [],
    )
    .expect("Failed to create PortalFigure table");

    // Insert data
    for figure in all_figure_data {
        conn.execute(
            "INSERT INTO PortalFigure (figure_id, variant_id, name) VALUES (?1, ?2, ?3)",
            [&figure.figure_id.to_string(), &figure.variant_id.to_string(), &figure.name],
        )
        .unwrap_or_else(|err| panic!("Failed to insert figure '{}': {}", figure.name, err));
    }
}
