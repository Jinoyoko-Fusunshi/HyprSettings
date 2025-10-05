#!/bin/bash

# load application information
source "scripts/.env"

# build hyprsettings application
if [ -d out ]; then
  rm -r out
fi

mkdir -p out
( cd scripts && makepkg -fs )

# sign binary packages
mkdir -p signing
sha256sum out/*.pkg.tar.zst > signing/SHA256SUMS
gpg --detach-sign --armor --output signing/SHA256SUMS.asc signing/SHA256SUMS