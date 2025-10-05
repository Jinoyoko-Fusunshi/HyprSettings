#!/bin/bash

# load application information
source "scripts/.env"

# check if build package exists and start build process otherwise
package_name="out/$APPLICATION_NAME-$APPLICATION_VERSION-$PACKAGE_VERSION-$APPLICATION_ARCH.pkg.tar.zst"
if [ ! -d "out" ] || [ ! -f "$package_name" ]; then
  . scripts/build.sh
fi

# install built application package
sudo pacman -U "$package_name"