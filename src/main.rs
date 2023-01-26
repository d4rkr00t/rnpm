use std::path::PathBuf;

use directories::ProjectDirs;

mod artifacts_manager;
mod package;
mod parsers;
mod strategies;

fn main() {
    let contents = std::fs::read_to_string("example/package-lock.json").unwrap();
    let packages = parsers::package_lock::parse(&contents);
    let proj_dir = ProjectDirs::from("com", "rnpm", "rnpm").unwrap();
    let dir_path = PathBuf::from("/Users/ssysoev/Development/rnpm/example/");

    let am = artifacts_manager::ArtifactsManager::new(proj_dir.data_dir());
    am.init().unwrap();

    let npm_strat = strategies::npm::NpmStrategy::new(dir_path);

    println!("Artifacts: \"{}\"", am.artifacts_path.display());
    println!("");

    npm_strat.install(&packages, &am).unwrap();
}

// https://pnpm.io/symlinked-node-modules-structure
// https://github.com/oven-sh/bun/blob/main/src/install/npm.zig
// https://github.com/npm/npm-registry-fetch/blob/main/lib/index.js#L108
