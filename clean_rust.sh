#!/bin/bash
# Helper script to clean the Rust code and the deploy_bookstore directory
pushd code
cargo clean
popd
pushd no_workspace/deploy_bookstore
cargo clean
popd
