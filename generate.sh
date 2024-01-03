#!/bin/bash
wget https://thetvdb.github.io/v4-api/swagger.yml
openapi-generator generate -g rust -i swagger.yml --skip-validate-spec
rm swagger.yml
sed -i 's/version = "\([0-9]\+\)\.\([0-9]\+\)"/version = "\1.\2.0"/' Cargo.toml
cargo fmt
