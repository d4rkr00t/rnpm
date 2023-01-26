use std::{
    fs::{self, File},
    io::{self, Cursor},
    path::{Path, PathBuf},
};

use flate2::read::GzDecoder;
use tar::Archive;

#[derive(Debug)]
pub struct ArtifactsManager {
    pub artifacts_path: PathBuf,
    artifacts_tmp_path: PathBuf,
    artifacts_tars_path: PathBuf,
}

impl ArtifactsManager {
    pub fn new(proj_dir_path: &Path) -> Self {
        let artifacts_path = proj_dir_path.join("artifacts");
        let artifacts_tars_path = artifacts_path.join("tars");
        let artifacts_tmp_path = artifacts_path.join("tmp");

        Self {
            artifacts_path,
            artifacts_tmp_path,
            artifacts_tars_path,
        }
    }

    pub fn init(&self) -> Result<(), ()> {
        fs::create_dir_all(&self.artifacts_path).unwrap();
        fs::create_dir_all(&self.artifacts_tmp_path).unwrap();
        fs::create_dir_all(&self.artifacts_tars_path).unwrap();
        Ok(())
    }

    pub fn fetch(&self, name: &str, req_url: &str) -> Result<(), ()> {
        let artifact_storage_path = self.get_tar_path(name);
        if artifact_storage_path.exists() {
            println!("Cached {}", req_url);
            return Ok(());
        }

        println!("Downloading {}", req_url);
        let mut file = File::create(artifact_storage_path).unwrap();
        let body = reqwest::blocking::get(req_url).unwrap();
        let mut content = Cursor::new(body.bytes().unwrap());
        io::copy(&mut content, &mut file).unwrap();

        Ok(())
    }

    fn get_tar_path(&self, name: &str) -> PathBuf {
        return self.artifacts_tars_path.join(format!("{}.tgz", name));
    }

    fn get_tmp_path(&self, name: &str) -> PathBuf {
        return self.artifacts_tmp_path.join(format!("{}.tgz", name));
    }

    fn copy_to_tmp(&self, name: &str) -> Result<(), ()> {
        let src_path = self.get_tar_path(name);
        if !src_path.exists() {
            return Err(());
        }
        let dest_path = self.get_tmp_path(name);
        let mut src = File::open(src_path).unwrap();
        let mut dest = File::create(dest_path).unwrap();
        io::copy(&mut src, &mut dest).unwrap();

        return Ok(());
    }

    pub fn unpack_to(&self, name: &str, dest: &PathBuf) -> Result<(), ()> {
        self.copy_to_tmp(name)?;
        let artifact_path = self.get_tmp_path(name);
        let tgz_file = File::open(artifact_path).unwrap();
        let tar = GzDecoder::new(tgz_file);
        let mut arch = Archive::new(tar);

        fs::create_dir_all(dest).unwrap();

        arch.entries().unwrap().for_each(|maybe_entry| {
            let mut entry = maybe_entry.unwrap();
            let entry_path = entry.path().unwrap().to_path_buf();
            let mut path_components = entry_path.components();

            // remove parent from path_components
            path_components.next();

            let clean_entry_path = path_components.as_path().to_path_buf();
            let full_dest_path = dest.join(clean_entry_path);
            let mut dest_dir_path = full_dest_path.clone();
            dest_dir_path.pop();

            fs::create_dir_all(dest_dir_path).unwrap();

            entry.unpack(full_dest_path).unwrap();
        });

        return Ok(());
    }
}

// Artifacts storage:
//  com.rnpm.rnpm
//  └── artifacts
//      ├── tars
//      │   ├── @babel__core__7.12.3.tgz
//      │   ├── @babel__generator__7.12.5.tgz
//      └── tmp <- temp dir to copy tars and unpack
