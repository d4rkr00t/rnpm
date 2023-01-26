use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

use crate::package::{Package, PackagesVec};

// https://docs.npmjs.com/cli/v9/configuring-npm/package-lock-json#file-format
#[derive(Debug, Deserialize)]
struct PackageLock {
    // version: String,
    // #[serde(rename(deserialize = "lockfileVersion"))]
    // lockfile_version: u32,
    packages: BTreeMap<String, PackageLockPackage>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PackageLockPackage {
    pub version: String,
    pub resolved: Option<String>,
    pub integrity: Option<String>,
    pub bin: Option<HashMap<String, String>>,
    #[serde(rename(deserialize = "hasInstallScript"))]
    pub has_install_script: Option<bool>,
}

pub fn parse(file_content: &str) -> PackagesVec {
    let package_lock: PackageLock = serde_json::from_str(file_content).unwrap();
    let mut packages_vec: PackagesVec = Vec::new();

    for (key, value) in package_lock.packages {
        if key.len() == 0 {
            continue;
        }

        let mut names: Vec<&str> = key["node_modules/".len()..]
            .split("/node_modules/")
            .collect();
        let name = names.pop().unwrap().to_string();

        packages_vec.push(Package {
            name,
            version: value.version,
            resolved: value.resolved.unwrap(),
            integrity: value.integrity.unwrap(),
            dest: key.to_string(),
            bin: value.bin.to_owned(),
        })
    }

    return packages_vec;
}
