use flate2::read::GzDecoder;
use rayon::prelude::*;
use tar::Archive;

use std::fs::File;
use std::io::Cursor;
use std::path::{Path, PathBuf};

mod parsers;

fn main() {
    let contents = std::fs::read_to_string("example/package-lock.json").unwrap();
    let dg = parsers::package_lock::parse(&contents);

    dg.par_iter().for_each(|(dest, id, pkg)| {
        fetch_artifact(pkg.resolved.as_ref().unwrap(), &id, &dest);
    });
}

fn fetch_artifact(req_url: &str, name: &str, dest: &str) {
    println!("{}", name);

    let dir_path = "/Users/ssysoev/Development/rnpm/example/artifacts/".to_string();
    let tmp_path = "/Users/ssysoev/Development/rnpm/example/artifacts/tmp".to_string();
    std::fs::create_dir_all(&dir_path).unwrap();
    std::fs::create_dir_all(&tmp_path).unwrap();

    let clean_name = name.replace("/", "__");
    let artifact_path = format!("{}/{}.tgz", dir_path, clean_name);
    let path = Path::new(&artifact_path);

    let artifact_tmp_path = format!("{}/{}.tgz", tmp_path, clean_name);
    let tmp_path = Path::new(&artifact_tmp_path);

    if !path.exists() {
        let mut file = File::create(path).unwrap();
        println!("Downloading {}", req_url);
        let body = reqwest::blocking::get(req_url).unwrap();
        let mut content = Cursor::new(body.bytes().unwrap());
        std::io::copy(&mut content, &mut file).unwrap();
    }
    let mut src = File::open(path).unwrap();
    let mut tmp = File::create(tmp_path).unwrap();
    std::io::copy(&mut src, &mut tmp).unwrap();

    let tgz_file = File::open(tmp_path).unwrap();
    let tar = GzDecoder::new(tgz_file);
    let mut ar = Archive::new(tar);
    let dest_path = format!("{}{}", dir_path, dest);
    std::fs::create_dir_all(&dest_path).unwrap();
    ar.entries().unwrap().for_each(|entry| {
        let mut e = entry.unwrap();
        let raw_path = e.path().unwrap();
        let path_buf = raw_path.to_path_buf().to_owned();
        let mut components = path_buf.components();
        components.next();
        let path = components.as_path().to_path_buf();
        let dest = format!("{}/{}", dest_path, path.to_str().unwrap());
        let mut p = PathBuf::from(dest.clone());
        p.pop();
        std::fs::create_dir_all(&p.as_path()).unwrap();

        e.unpack(dest).unwrap();
    });
}

// https://pnpm.io/symlinked-node-modules-structure
// https://github.com/oven-sh/bun/blob/main/src/install/npm.zig
// https://github.com/npm/npm-registry-fetch/blob/main/lib/index.js#L108
