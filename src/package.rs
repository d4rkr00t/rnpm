use std::collections::HashMap;

pub struct Package {
    pub is_top_level: bool,
    pub name: String,
    pub version: String,
    pub resolved: String,
    pub integrity: String,
    pub dest: String,
    pub bin: Option<HashMap<String, String>>,
    pub has_install_scripts: bool,
    pub is_bundled: bool,
}

impl Package {
    pub fn get_clean_name(&self) -> String {
        self.name.replace("@", "__").replace("/", "__")
    }

    pub fn get_id(&self) -> String {
        format!("{}__{}", self.get_clean_name(), self.version)
    }
}

pub type PackagesVec = Vec<Package>;
