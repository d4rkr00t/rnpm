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
    pub bin: Option<HashMap<String, String>>,

    #[serde(default)]
    pub resolved: String,

    #[serde(default)]
    pub integrity: String,

    #[serde(rename(deserialize = "inBundle"), default)]
    pub in_bundle: bool,

    #[serde(rename(deserialize = "hasInstallScript"), default)]
    pub has_install_script: bool,
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
        let is_top_level = names.len() == 1;
        let name = names.pop().unwrap().to_string();

        packages_vec.push(Package {
            is_top_level,
            name,
            version: value.version,
            resolved: value.resolved,
            is_bundled: value.in_bundle,
            integrity: value.integrity,
            dest: key.to_string(),
            bin: value.bin.to_owned(),
            has_install_scripts: value.has_install_script,
        })
    }

    return packages_vec;
}
