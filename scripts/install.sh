#!/bin/bash

# meta information for the linux arch package build and installation
export APPLICATION_NAME="hyprsettings"
export APPLICATION_VERSION="0.1.0"
export PACKAGE_VERSION="1"
export APPLICATION_ARCH="x86_64"
# build and package hyprsettings application
sudo pacman -U "out/$APPLICATION_NAME-$APPLICATION_VERSION-$PACKAGE_VERSION-$APPLICATION_ARCH.pkg.tar.zst"