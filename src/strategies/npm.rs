use rayon::prelude::*;
use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};

use crate::{
    artifacts_manager::ArtifactsManager,
    package::{Package, PackagesVec},
    parsers::package_json,
};

pub struct NpmStrategy {
    working_directory: PathBuf,
}

const PACKAGE_LOCK: &str = "package-lock.json";
const NODE_MODULES: &str = "node_modules";

impl NpmStrategy {
    pub fn new(working_directory: PathBuf) -> Self {
        Self { working_directory }
    }

    pub fn install(
        &self,
        packages: &PackagesVec,
        am: &ArtifactsManager,
        should_run_scripts: bool,
    ) -> Result<(), ()> {
        packages.par_iter().for_each(|pkg| {
            // packages.iter().for_each(|pkg| {
            if pkg.is_bundled {
                return;
            }

            am.fetch(&pkg.get_id(), &pkg.resolved).unwrap();

            let dest_path = self.working_directory.join(&pkg.dest);
            let mut should_clone = true;

            if dest_path.exists() {
                let package_json_path = dest_path.join("package.json");
                if package_json_path.exists() {
                    let pj = package_json::parse_from_path(&package_json_path);
                    if pj.version == pkg.version {
                        should_clone = false;
                    } else {
                        // println!("Removing: {}", dest_path.display());
                        // fs::remove_dir_all(&dest_path).unwrap();
                    }
                }
            }

            if should_clone {
                // println!("Cloning: {}", dest_path.display());
                am.clone_to(&pkg.get_id(), &dest_path).unwrap();
            }

            if pkg.is_top_level {
                self.link_bin(&pkg, &dest_path).unwrap();
            }
        });

        if should_run_scripts {
            self.run_scripts(packages)?;
        }

        self.copy_lock_file().unwrap();

        return Ok(());
    }

    fn run_scripts(&self, packages: &PackagesVec) -> Result<(), ()> {
        packages.par_iter().for_each(|pkg| {
            if pkg.has_install_scripts {
                return;
            }

            let dest_path = self.working_directory.join(&pkg.dest);
            let package_json_path = dest_path.join("package.json");
            let package_json_content = std::fs::read_to_string(package_json_path).unwrap();
            let pkg_json = package_json::parse(&package_json_content);
            if let Some(scripts) = pkg_json.scripts {
                if scripts.contains_key("install") {
                    let install_script = scripts.get("install").unwrap();
                    let dest_path_string = dest_path.display();
                    let output = execute!(
                        r"cd '{dest_path_string}'
                        {install_script}"
                    );
                    println!("{:?}", output);
                }
            }
        });
        return Ok(());
    }

    fn get_lock_file_path(&self) -> PathBuf {
        return self.working_directory.join(PACKAGE_LOCK);
    }

    fn get_node_modules_path(&self) -> PathBuf {
        return self.working_directory.join(NODE_MODULES);
    }

    fn get_node_modules_bin_path(&self) -> PathBuf {
        return self
            .working_directory
            .join(PathBuf::from(format!("{NODE_MODULES}/.bin")));
    }

    fn copy_lock_file(&self) -> Result<(), ()> {
        let node_modules_path = self.get_node_modules_path();
        let lock_file_path = self.get_lock_file_path();
        let lock_file_dest_path = node_modules_path.join(format!(".{PACKAGE_LOCK}"));

        if !lock_file_path.exists() {
            return Err(());
        }

        if !node_modules_path.exists() {
            return Err(());
        }

        let mut src = File::open(lock_file_path).unwrap();
        let mut dest = File::create(lock_file_dest_path).unwrap();
        io::copy(&mut src, &mut dest).unwrap();

        return Ok(());
    }

    fn link_bin(&self, package: &Package, package_src_path: &PathBuf) -> Result<(), ()> {
        if let None = package.bin {
            return Ok(());
        }
        let bins = package.bin.as_ref().unwrap();
        let bin_path = self.get_node_modules_bin_path();
        fs::create_dir_all(&bin_path).unwrap();

        for (key, value) in bins.iter() {
            let from_in_bin = bin_path.join(PathBuf::from(key));
            let to_in_package = package_src_path.join(PathBuf::from(value));
            if from_in_bin.exists() {
                fs::remove_file(&from_in_bin).unwrap();
            }

            std::os::unix::fs::symlink(&to_in_package, &from_in_bin).unwrap();
        }

        return Ok(());
    }
}

// TODO: make sure .package-lock.json matches what npm produces
// TODO: support postinstall scripts
// TODO: define package-lock.json supported version range
