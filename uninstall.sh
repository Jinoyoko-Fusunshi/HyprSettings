#!/bin/bash

hyprsettings_path="/usr/share/HyprSettings"
if [ -f $hyprsettings_path ]; then
  sudo rm -R $hyprsettings_path
fi

hyprsettings_res_path="/usr/share/HyprSettings"
if [ -d $hyprsettings_res_path ]; then
  sudo rm -R $hyprsettings_res_path
fi