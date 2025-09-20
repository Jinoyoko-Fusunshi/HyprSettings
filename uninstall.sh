#!/bin/bash

hyprsettings_path="/usr/bin/hyprsettings"
if [ -f $hyprsettings_path ]; then
  sudo rm -R $hyprsettings_path
fi

hyprsettings_res_path="/usr/share/hyprsettings"
if [ -d $hyprsettings_res_path ]; then
  sudo rm -R $hyprsettings_res_path
fi

hyprsettings_desktop_path="/usr/share/applications/hyprsettings.desktop"
if [ -f $hyprsettings_desktop_path ]; then
  sudo rm $hyprsettings_desktop_path
fi