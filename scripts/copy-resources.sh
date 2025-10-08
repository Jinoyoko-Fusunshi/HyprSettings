# create share directory and copy style resource for the HyprSettings applications
shared_directory="/usr/share/hyprsettings"
dark_style="dark-style.css"
light_style="light-style.css"

sudo mkdir -p "$shared_directory"
sudo cp "res/$dark_style" "$shared_directory/$dark_style"
sudo cp "res/$light_style" "$shared_directory/$light_style"