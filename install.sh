#!/bin/bash

# build and copy executable into binary directory
cargo build --release
sudo cp target/release/hyprsettings /usr/bin

# copy styles
sudo bash copy-resources.sh

# copy desktop assets
sudo cp res/hyprsettings.desktop /usr/share/applications