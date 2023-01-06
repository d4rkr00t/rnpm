use serde::{Deserialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct PackageLock {
    version: String,
    lockfileVersion: u32,
    packages: HashMap<String, PackageLockPackage>,
}

#[derive(Debug, Deserialize)]
struct PackageLockPackage {
    version: String,
    resolved: Option<String>,
    integrity: Option<String>,
}

pub fn parse(file_content: &str) {
    let packageLock: PackageLock = serde_json::from_str(file_content).unwrap();
    println!("{:#?}", packageLock);
}
