mod parsers;

fn main() {
    let contents = std::fs::read_to_string("example/package-lock.json").unwrap();
    parsers::package_lock::parse(&contents);
}

// https://pnpm.io/symlinked-node-modules-structure
// https://github.com/oven-sh/bun/blob/main/src/install/npm.zig
// https://github.com/npm/npm-registry-fetch/blob/main/lib/index.js#L108
