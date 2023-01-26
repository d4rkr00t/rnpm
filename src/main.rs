use std::path::PathBuf;

use directories::ProjectDirs;
use rayon::prelude::*;

mod artifacts_manager;
mod package;
mod parsers;

fn main() {
    let contents = std::fs::read_to_string("example/package-lock.json").unwrap();
    let dg = parsers::package_lock::parse(&contents);
    let proj_dir = ProjectDirs::from("com", "rnpm", "rnpm").unwrap();
    let am = artifacts_manager::ArtifactsManager::new(proj_dir.data_dir());
    am.init().unwrap();
    println!("Artifacts: \"{}\"", am.artifacts_path.display());
    println!("");
    let dir_path = PathBuf::from("/Users/ssysoev/Development/rnpm/example/artifacts/");

    dg.par_iter().for_each(|pkg| {
        am.fetch(&pkg.get_id(), &pkg.resolved).unwrap();
        let dest_path = dir_path.join(&pkg.dest);
        am.unpack_to(&pkg.get_id(), &dest_path).unwrap();
    });
}

// https://pnpm.io/symlinked-node-modules-structure
// https://github.com/oven-sh/bun/blob/main/src/install/npm.zig
// https://github.com/npm/npm-registry-fetch/blob/main/lib/index.js#L108

// Artifacts storage:
//  com.rnpm.rnpm
//  └── artifacts
//      ├── tars
//      │   ├── @babel__core__7.12.3.tgz
//      │   ├── @babel__generator__7.12.5.tgz
//      └── tmp <- temp dir to copy tars and unpack
