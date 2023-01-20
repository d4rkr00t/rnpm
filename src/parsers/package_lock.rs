use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

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

// {
//   name_to_id: {
//      @babel/code-frame@7.18.6: 0,
//      @babel/code-generator@7.20.7: 1,
//      ...
//      @rollup/plugin-sucrase@2.0.2: 10
//      estree-walker@2.0.2: 11
//   },
//   packages: [
//      {@babel/code-frame@7.18.6, top_level},
//      {@babel/code-generator@7.18.6, top_level},
//      ...
//      {@rollup/plugin-surcase@2.0.2, top_level},
//      {estree-walker@2.0.2},
//   ],
//   dependencies:[
//      [],
//      []
//      ...
//      [11],
//      []
//   ]
// }
//
// [(dest_path, package)]

pub type PackagesVec = Vec<(String, String, PackageLockPackage)>;

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
        let name = names.pop().unwrap().replace("@", "__").replace("/", "__");
        let id = format!("{}__{}", name, value.version);
        packages_vec.push((key, id, value));
    }

    return packages_vec;
}
