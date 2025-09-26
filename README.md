# HyprSettings

A generical independent tool to configure the hyprland environment by a GTK GUI.
This tool gives the ability to easily configure all hyprland variables and writes the settings into their config files 
after clicking the save button.

<img src="docs/hyprsettings-overview.png" alt="HyprSettings Overview" height="511"/>

Since the Hyprland ecosystem is divided into separate program packages, each program has its own category panel to be configured. 
Each program is internally stored as a module and can only be configured when the program at start time actually exists
in the **$PATH** system environments. If not, a warning message appears in the specific clicked category and no settings can be adjusted.

As of right now, an own YAML config file will be used to store all the settings applied in the program. So at the beginning,
even when settings are made in the config file, they will be ignored and all settings must be reapplied in the GUI first. 
After the initial setup, the current settings will be loaded after the program restarts.
In the future a foolproof config parser will be implemented, so existing configs will be considered and be the state of proof.

## Modules
The current supported hyrpland modules to be configured are the following:
- **Hyprland** → the main desktop environment with its settings
- **Hyprpaper** → wallpaper settings
- **Hyprlock** → lockscreen settings

Each hyprland module in the ecosystem uses its own config file, so each module setting will be separately internally 
stored and written back to.

## Support
Since Hyprland is mainly focused for Linux Arch, this tool also is focused on this distribution only. 
This is very clear when searching for the hyprland modules at program start, because it uses the pacman package manager 
to search for the existing programs. For now support for other distributions is not planned.

## Build
Create the release build with cargo:
```shell
cargo build --release
```

Copy the styles css file to its required directory:
```shell
bash copy-resources.sh
```

Run the program via terminal:
```shell
./target/release/hyprsettings
```

## Dependencies

### programs
- **[cargo, rust](https://doc.rust-lang.org/cargo/)** = 1.89.0

### Rust packages
- **[gtk](https://github.com/gtk-rs/gtk4-rs)** = 0.10.1
- **[serde](https://github.com/serde-rs/serde)** = 1.0.225
- **[serde_yaml](https://github.com/dtolnay/serde-yaml)** = 0.9.34

## Installation
Install the program via the script (requires root)
```shell
bash install.sh
```
Uninstall the program via the script (requires root)
```shell
bash uninstall.sh
```

Run the program via terminal
```shell
hyprsettings
```

## Project
The development of this project originated with the main idea to actively learn the GTK framework and the Rust programming language.
Since it formed into a kind of usable tool, the idea was set to make it public for everyone.

> ⚠️ This tool will receive updates since not all features are fully implemented and only contains the basic features 
> for hyprland beginners. It may have some problems regarding the monitor settings 
> and other areas.
