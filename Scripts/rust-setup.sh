#!/usr/bin/env bash

echo "Current installed Rust build targets:"
rustup target list | grep apple
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
