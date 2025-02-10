# 8BitDo Micro Configurator

A command-line interface tool for configuring your 8BitDo Micro controller key bindings.

## Features

- Read current key bindings from your device
- Load and apply key binding profiles
- Manage multiple binding profiles
- Verify applied configurations
- Support for local and system-wide profiles

## Installation

### Prerequisites

- Rust and Cargo installed on your system
- A compatible 8BitDo Micro controller
- Linux/macOS with Bluetooth support

### Building from source

```bash
git clone https://git.thoxy.xyz/thoxy/8bitult
cd 8bitult
cargo build --release
```

The binary will be available at `target/release/8bitult-cli`

## Usage

### Commands Overview

```bash
8bitult-cli [COMMAND] [OPTIONS]
```

Available commands:
- `list`: Display all available binding profiles with their descriptions
- `read`: Read and display current device bindings
- `attach`: Apply a binding profile to the device
- `save`: Save current device configuration as a new profile
- `backup`: Import or export profiles

### List Available Profiles

List all available binding profiles with their descriptions:

```bash
8bitult-cli list
```

Profiles can be stored in:
- `./profiles/` (local to your current directory)
- `~/.config/8bitult/profiles/` (system-wide)

### Read Current Bindings

Display the current key bindings from your device:

```bash
8bitult-cli read
```

### Attach a Profile

Apply a profile to your device using either the profile name or a direct path:

```bash
# Using profile name
8bitult-cli attach -p "Default"

# Using config file path
8bitult-cli attach -c /path/to/profile.toml
```

### Save Current Configuration

Save the current device configuration as a new profile:

```bash
# Basic save with name
8bitult-cli save -n "My Profile"

# Save with description
8bitult-cli save -n "My Profile" -d "Custom profile for gaming"

# Save to specific location
8bitult-cli save -n "My Profile" -o /path/to/save/profile.toml
```

### Backup Management

Export all profiles to an archive:
```bash
# Export to default backup.tar.zst
8bitult-cli backup --export

# Export to specific file
8bitult-cli backup --export --save my_backup.tar.zst
```

Import profiles from archive(s):
```bash
# Import profiles from .tar.zst files in current directory
8bitult-cli backup --import
```

## Profile Configuration

Profiles are defined in TOML format and must include:
- A `name` field identifying the profile
- An optional `description` field providing additional information
- A `bindings` section mapping buttons to their key assignments
- Each button can have 0 to 4 key bindings
- Keys can be specified by name or using `keycode(XX)` format where XX is a hexadecimal value

Here's an example:

```toml
name = "Default"
description = "Default 8BitDo Micro keyboard layout"

[bindings]
A = ["G"]
B = ["J"]
X = ["H"]
Y = ["I"]
L1 = ["K"]
R1 = ["M"]
L2 = ["L"]
R2 = ["R"]
SELECT = ["N"]
START = ["O"]
HOME = []
LOGO = ["S"]
UP = ["C"]
DOWN = ["D"]
LEFT = ["E"]
RIGHT = ["F"]
```

### Available Buttons

- Main buttons: `A`, `B`, `X`, `Y`
- Shoulder buttons: `L1`, `R1`, `L2`, `R2`
- Special buttons: `SELECT`, `START`, `HOME`, `LOGO`
- D-Pad: `UP`, `DOWN`, `LEFT`, `RIGHT`

### Key Binding Rules

1. Each button can have up to 4 keys assigned:
```toml
# Example of maximum key bindings per button
A = ["G", "B", "C", "D"]  # 4 keys assigned to button A
START = ["ENTER"]         # Single key assigned
HOME = []                 # No keys assigned
```

2. Custom key codes can be used with the `keycode()` syntax:
```toml
# Using custom key codes (in hexadecimal)
L1 = ["keycode(04)", "keycode(05)"]  # Using raw key codes
R1 = ["A", "keycode(06)"]            # Mixing predefined keys and key codes
```

For detailed information about HID keycodes, please refer to the [Keycodes documentation](doc/keycodes.md)

### Available Keys

The tool supports a wide range of keys including:
- Alphabet keys: `A` to `Z`
- Numbers: `0` to `9`
- Function keys: `F1` to `F12`
- Special keys: `ESC`, `TAB`, `ENTER`, `SPACE`, etc.
- Modifiers: `LEFT_SHIFT`, `RIGHT_SHIFT`, `LEFT_CTRL`, `RIGHT_CTRL`, etc.
- Navigation: `UP`, `DOWN`, `LEFT`, `RIGHT`, `HOME`, `END`, etc.
- And many more

For a complete list of supported keys, use the built-in help:
```bash
8bitult-cli help
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request