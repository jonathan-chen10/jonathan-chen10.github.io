#!/usr/bin/env zsh

SCRIPT_DIR="$(cd -- "$(dirname -- "${(%):-%x}")" && pwd)"
echo $SCRIPT_DIR
cd $SCRIPT_DIR
cargo run --manifest-path blog-generator/Cargo.toml -- ssg.toml
