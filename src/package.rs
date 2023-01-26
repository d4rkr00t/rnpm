pub struct Package {
    pub name: String,
    pub version: String,
    pub resolved: String,
    pub integrity: String,
    pub dest: String,
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
