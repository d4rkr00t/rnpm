hyperfine -w 2 -p "rm -rf node_modules" --export-markdown warm-cache.md "../../target/release/rnpm" "bun install --ignore-scripts" "/Users/ssysoev/Development/temp/orogene/target/release/oro restore --cache oro-cache" "pnpm install --store-dir pnpm-cache --ignore-scripts"