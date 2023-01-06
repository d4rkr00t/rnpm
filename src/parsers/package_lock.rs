use serde::Deserialize;
use std::collections::HashMap;

// https://docs.npmjs.com/cli/v9/configuring-npm/package-lock-json#file-format
#[derive(Debug, Deserialize)]
struct PackageLock {
    version: String,
    #[serde(rename(deserialize = "lockfileVersion"))]
    lockfile_version: u32,
    packages: HashMap<String, PackageLockPackage>,
}

#[derive(Debug, Deserialize)]
struct PackageLockPackage {
    version: String,
    resolved: Option<String>,
    integrity: Option<String>,
    bin: Option<HashMap<String, String>>,
    #[serde(rename(deserialize = "hasInstallScript"))]
    has_install_script: Option<bool>,
}

pub fn parse(file_content: &str) {
    let package_lock: PackageLock = serde_json::from_str(file_content).unwrap();
    println!("{:#?}", package_lock);
}
