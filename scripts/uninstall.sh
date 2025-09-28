#!/bin/bash

sudo pacman -R hyprsettings

hyprsettings_config="$HOME/.config/hypr/hyprsettings.yaml"
if [ -f "$hyprsettings_config" ]; then
  sudo rm "$hyprsettings_config"
fi