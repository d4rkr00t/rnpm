use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Cursor, Write},
    path::{Path, PathBuf},
};

use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use tar::Archive;

#[derive(Debug)]
pub struct ArtifactsManager {
    pub artifacts_path: PathBuf,
    artifacts_files_path: PathBuf,
}

impl ArtifactsManager {
    pub fn new(proj_dir_path: &Path) -> Self {
        let artifacts_path = proj_dir_path.join("artifacts");
        let artifacts_files_path = artifacts_path.join("files");

        Self {
            artifacts_path,
            artifacts_files_path,
        }
    }

    pub fn init(&self) -> Result<(), ()> {
        fs::create_dir_all(&self.artifacts_path).unwrap();
        fs::create_dir_all(&self.artifacts_files_path).unwrap();
        Ok(())
    }

    pub fn fetch(&self, pkg_id: &str, req_url: &str) -> Result<(), ()> {
        let pkg_manifest_path = self.get_pkg_manifest_path(pkg_id);
        if pkg_manifest_path.exists() {
            println!("Cached {}", req_url);
            return Ok(());
        }

        println!("Downloading {}", req_url);
        if let Ok(body) = reqwest::blocking::get(req_url) {
            let mut content = Cursor::new(body.bytes().unwrap());
            let mut tar = GzDecoder::new(&mut content);
            self.unpack_to_storage(&mut tar, pkg_id).unwrap();
        } else {
            return Err(());
        }

        Ok(())
    }

    pub fn clone_to(&self, pkg_id: &str, dest_path: &PathBuf) -> Result<(), ()> {
        let pkg_manifest_path = self.get_pkg_manifest_path(pkg_id);
        if !pkg_manifest_path.exists() {
            return Err(());
        }
        let pkg_manifest_content = std::fs::read_to_string(pkg_manifest_path).unwrap();
        let pkg_manifest: PackageManifest = serde_json::from_str(&pkg_manifest_content).unwrap();

        for (file_dest, file_src) in pkg_manifest.files.iter() {
            let file_dest_path = dest_path.join(file_dest);
            let file_src_path = self.artifacts_files_path.join(file_src);
            let mut dir_path = file_dest_path.clone();
            dir_path.pop();
            fs::create_dir_all(&dir_path).unwrap();

            let mut file_dest = File::create(&file_dest_path).unwrap();
            let mut file_src = File::open(&file_src_path).unwrap();

            io::copy(&mut file_src, &mut file_dest).unwrap();
        }

        return Ok(());
    }

    fn get_pkg_manifest_path(&self, pkg_id: &str) -> PathBuf {
        return self.artifacts_path.join(format!("{}.json", pkg_id));
    }

    fn unpack_to_storage(&self, readable: &mut dyn std::io::Read, pkg_id: &str) -> Result<(), ()> {
        let mut arch = Archive::new(readable);
        let mut path_map: HashMap<String, String> = HashMap::new();

        for file in arch.entries().unwrap() {
            let mut file = file.unwrap();
            let header = file.header();

            if header.entry_type().is_dir() {
                continue;
            }

            let file_path = file.path().unwrap().to_path_buf();
            let mut file_path_cmp = file_path.components();

            // remove parent from file_path_cmp
            file_path_cmp.next();

            let clean_file_path = file_path_cmp.as_path();
            let clean_file_path_str = clean_file_path.to_str().unwrap();

            let mut hasher = Sha1::new();
            hasher.update(pkg_id.as_bytes());
            hasher.update(clean_file_path_str);
            hasher.update(header.cksum().unwrap().to_string());
            let hash = format!("{:x}", hasher.finalize());

            let dest_path = self.artifacts_files_path.join(&hash);
            file.unpack(dest_path).unwrap();

            path_map.insert(clean_file_path_str.to_string(), hash);
        }

        let package_manifest = PackageManifest { files: path_map };
        let serialised_package_manifest = serde_json::to_string(&package_manifest).unwrap();
        let pkg_manifest_path = self.get_pkg_manifest_path(pkg_id);
        let package_manifest_file = File::create(pkg_manifest_path);
        package_manifest_file
            .unwrap()
            .write_all(serialised_package_manifest.as_bytes())
            .unwrap();
        return Ok(());
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageManifest {
    pub files: HashMap<String, String>,
}

//
// Artifacts storage:
//  com.rnpm.rnpm
//  └── artifacts
//      ├── files
//          ├── hash_of_a_file
//          ├── hash_of_a_file
//          ├── hash_of_a_file
//          └── package_integirty.json
