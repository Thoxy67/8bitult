# 8BitDo Micro Configurator

A command-line tool for reading and changing the key bindings on an 8BitDo Micro controller.

## Features

- Read the current key bindings from the device
- Apply a binding profile from a TOML file
- Save the device's current bindings into a new profile
- List the profiles it can find
- Export and import profiles as a compressed archive
- Verify that bindings were applied by reading them back

## Installation

### Prerequisites

- Rust and Cargo
- An 8BitDo Micro controller
- Linux or macOS with Bluetooth. On Linux you also need BlueZ (`bluetoothctl`).

### Building from source

```bash
git clone https://git.thoxy.xyz/thoxy/8bitult
cd 8bitult
cargo build --release
```

The binary ends up at `target/release/8bitult-cli`.

## Usage

```bash
8bitult-cli [COMMAND] [OPTIONS]
```

Commands:

- `list` shows the profiles it can find
- `read` reads and prints the current device bindings
- `attach` writes a profile to the device
- `save` reads the device and saves the result as a profile
- `backup` exports or imports profiles

Add `-v` for info logs, `-vv` for debug, `-vvv` for trace. This is the first thing to reach for when a command cannot find or talk to the controller.

### List profiles

```bash
8bitult-cli list
```

Profiles are looked up in two places:

- `./profiles/` next to where you run the command
- `~/.config/8bitult/profiles/` for your user

### Read current bindings

```bash
8bitult-cli read
```

If the bindings on the device match one of your saved profiles, the tool tells you which one.

### Attach a profile

Apply a profile by name, or point straight at a file:

```bash
# By profile name (the `name` field inside the TOML, not the file name)
8bitult-cli attach -p "Default"

# By file path
8bitult-cli attach -c /path/to/profile.toml
```

After writing, the tool reads the bindings back and warns you if anything did not stick.

### Save the current configuration

```bash
# Just a name
8bitult-cli save -n "My Profile"

# With a description
8bitult-cli save -n "My Profile" -d "Custom profile for gaming"

# To a specific file
8bitult-cli save -n "My Profile" -o /path/to/profile.toml
```

Without `-o`, the profile is written to `~/.config/8bitult/profiles/`, with the file name derived from the profile name (lowercased, spaces turned into underscores).

### Backup

Both flags take the archive path as their argument.

```bash
# Export every profile into one archive
8bitult-cli backup --export backup.tar.zst

# Import the profiles from an archive into your user profiles directory
8bitult-cli backup --import backup.tar.zst
```

Imported profiles are unpacked into `~/.config/8bitult/profiles/`.

## Profile format

A profile is a TOML file with three parts:

- `name`: how the profile is identified by `attach -p` and shown by `list`
- `description`: optional free text
- `[bindings]`: a button to keys mapping, where each button gets 0 to 4 keys

Example:

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

### Buttons

The button names are fixed. These are the 16 you can use:

- Face buttons: `A`, `B`, `X`, `Y`
- Shoulders: `L1`, `R1`, `L2`, `R2`
- System: `SELECT`, `START`, `HOME`, `LOGO`
- D-Pad: `UP`, `DOWN`, `LEFT`, `RIGHT`

Leave a button out, or give it an empty list, to bind nothing to it.

### Keys

Each button takes up to 4 keys. Anything past the fourth is ignored. A key can be written two ways:

1. By name, for example `"A"`, `"Enter"`, `"Space"`, `"LeftShift"`.
2. As a raw HID code in hex, for example `"keycode(04)"` for `A` or `"keycode(28)"` for `Enter`.

```toml
[bindings]
A = ["G", "B", "C", "D"]              # four keys on one button
START = ["Enter"]                     # one key
HOME = []                             # nothing
L1 = ["keycode(04)", "keycode(05)"]   # raw codes for A and B
R1 = ["A", "keycode(06)"]             # names and raw codes can mix
```

**Key names are case sensitive and must match the reference table exactly.** They follow the spelling in the keycode table, so:

- Letters are single uppercase, `A` to `Z`
- Number row keys are `Key1` to `Key9` and `Key0`, not `1` to `0`
- Common keys are spelled out: `Enter`, `Escape`, `Backspace`, `Tab`, `Space`, `Delete`
- Modifiers are `LeftCtrl`, `LeftShift`, `LeftAlt`, `LeftMeta`, and the `Right*` versions
- Arrows are `Up`, `Down`, `Left`, `Right`

If a name does not match, the command fails and tells you which key it could not resolve. When in doubt, use `keycode(XX)`.

The full list of names and their codes is in [doc/keycodes.md](doc/keycodes.md). That file also explains an important detail: codes map to physical key positions on a US QWERTY keyboard, so on other layouts you bind by where the key sits, not by the letter printed on it.

## License

MIT. See [LICENSE](LICENSE).

## Contributing

Pull requests are welcome.

1. Fork the repository
2. Create a branch (`git checkout -b feature/my-change`)
3. Commit your changes
4. Push the branch
5. Open a pull request
