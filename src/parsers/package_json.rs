use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PackageJson {
    pub name: String,
    pub version: String,

    pub scripts: Option<HashMap<String, String>>,
}

pub fn parse(file_content: &str) -> PackageJson {
    let package_json: PackageJson = serde_json::from_str(file_content).unwrap();
    return package_json;
}

pub fn parse_from_path(path: &PathBuf) -> PackageJson {
    let file_content = std::fs::read_to_string(path).unwrap();
    return parse(&file_content);
}
