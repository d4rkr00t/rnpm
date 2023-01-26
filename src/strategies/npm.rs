use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};

use rayon::prelude::*;

use crate::{
    artifacts_manager::ArtifactsManager,
    package::{Package, PackagesVec},
};

pub struct NpmStrategy {
    working_directory: PathBuf,
}

impl NpmStrategy {
    pub fn new(working_directory: PathBuf) -> Self {
        Self { working_directory }
    }

    pub fn install(&self, packages: &PackagesVec, am: &ArtifactsManager) -> Result<(), ()> {
        packages.par_iter().for_each(|pkg| {
            am.fetch(&pkg.get_id(), &pkg.resolved).unwrap();

            let dest_path = self.working_directory.join(&pkg.dest);
            am.unpack_to(&pkg.get_id(), &dest_path).unwrap();

            self.link_bin(&pkg, &dest_path).unwrap();
        });

        self.copy_lock_file().unwrap();

        return Ok(());
    }

    fn get_lock_file_path(&self) -> PathBuf {
        return self.working_directory.join("package-lock.json");
    }

    fn get_node_modules_path(&self) -> PathBuf {
        return self.working_directory.join("node_modules");
    }

    fn get_node_modules_bin_path(&self) -> PathBuf {
        return self
            .working_directory
            .join(PathBuf::from("node_modules/.bin"));
    }

    fn copy_lock_file(&self) -> Result<(), ()> {
        let node_modules_path = self.get_node_modules_path();
        let lock_file_path = self.get_lock_file_path();
        let lock_file_dest_path = node_modules_path.join(".package-lock.json");

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
