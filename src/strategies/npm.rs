use std::{fs::File, io, path::PathBuf};

use rayon::prelude::*;

use crate::{artifacts_manager::ArtifactsManager, package::PackagesVec};

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
}

// TODO: make sure .package-lock.json matches what npm produces
// TODO: support .bin linking
