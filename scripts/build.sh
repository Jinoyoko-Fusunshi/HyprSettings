#!/bin/bash

# meta information for the linux arch package build process
export APPLICATION_NAME="hyprsettings"
export APPLICATION_VERSION="0.1.1"
export PACKAGE_VERSION="1"
export APPLICATION_ARCH="x86_64"
export BUILDDIR="../out"
export PKGDEST="../out"
export PACKAGER="Jinoyoko <jinoyoko@outlook.com>"

# build hyprsettings application
mkdir -p out
( cd scripts && makepkg -fs )

# sign binary packages
mkdir -p signing
sha256sum out/*.pkg.tar.zst > signing/SHA256SUMS
gpg --detach-sign --armor --output signing/SHA256SUMS.asc signing/SHA256SUMS