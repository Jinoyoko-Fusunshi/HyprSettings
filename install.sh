#!/bin/bash

# build and copy executable into binary directory
cargo build --release
sudo cp target/release/HyprSettings /usr/bin

sudo bash copy-resources.sh