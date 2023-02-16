use std::collections::HashMap;

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
