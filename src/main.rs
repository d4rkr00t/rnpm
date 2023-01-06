mod parsers;

fn main() {
    let contents = std::fs::read_to_string("example/package-lock.json").unwrap();
    parsers::package_lock::parse(&contents);
}
