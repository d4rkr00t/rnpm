use std::env;

#[macro_use]
extern crate commandspec;

use directories::ProjectDirs;

mod artifacts_manager;
mod package;
mod parsers;
mod strategies;

fn main() {
    let cwd = env::current_dir().unwrap();
    let package_lock_path = cwd.join("package-lock.json");
    let contents = std::fs::read_to_string(package_lock_path).unwrap();
    let packages = parsers::package_lock::parse(&contents);
    let proj_dir = ProjectDirs::from("com", "rnpm", "rnpm").unwrap();

    let am = artifacts_manager::ArtifactsManager::new(proj_dir.data_dir());
    am.init().unwrap();

    let npm_strat = strategies::npm::NpmStrategy::new(cwd);

    println!("Artifacts: \"{}\"", am.artifacts_path.display());
    println!("Installing {} packages...", packages.len());

    npm_strat.install(&packages, &am, false).unwrap();

    println!("Done! Installed {} packages...", packages.len());
}

// https://pnpm.io/symlinked-node-modules-structure
// https://github.com/oven-sh/bun/blob/main/src/install/npm.zig
// https://github.com/npm/npm-registry-fetch/blob/main/lib/index.js#L108
// https://github.com/orogene/orogene/issues/194
//
// TODO: proper cli interface
// TODO: proper error handling
// TODO: proper logging - default, verbose, etc...
// TODO: test outputs
// TODO: deduplicate packages by id
