#! /bin/bash

set -ev

DIR=`pwd`;

if [ -z "$NO_STD" ]; then
    cd "$DIR/alga" && cargo test;
    cd "$DIR/alga" && cargo run --example vectors;
    if [ "$TRAVIS_RUST_VERSION" == "nightly" ]; then
        cargo clean;
        cd "$DIR/alga_derive_test" && cargo test --verbose;
    fi
fi