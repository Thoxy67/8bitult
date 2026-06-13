# 8bitult

Desktop software for configuring 8BitDo controllers over Bluetooth Low Energy, written in Rust.

It does what the official 8BitDo Ultimate Software does, but from the terminal and on Linux. Right now it only supports the **8BitDo Micro**, since that is the only device I have to reverse engineer. Support for other devices can be added once the protocol for them is known.

> Status: work in progress. The BLE protocol was reverse engineered by sniffing the official app, so expect rough edges.

## What it does

- Read the key bindings currently stored on the controller
- Apply a binding profile from a TOML file
- Save the controller's current bindings back to a profile
- List the profiles it can find
- Export and import all your profiles as a single compressed archive

## Layout

This is a Cargo workspace with two crates:

| Crate | Path | Role |
|-------|------|------|
| `heigtbitult` | `./src` | Library. BLE connection, the read/write protocol, the keycode table. |
| `heigtbitult-cli` | `./cli` | The `8bitult-cli` binary. Profiles, backups, and the terminal UI. |

If you want the full command reference and the profile format, see the [CLI README](cli/README.md). For the list of keys you can bind, see the [keycode reference](cli/doc/keycodes.md).

## Building

You need a recent Rust toolchain and a working Bluetooth stack.

```bash
git clone https://git.thoxy.xyz/thoxy/8bitult
cd 8bitult
cargo build --release
```

The binary lands at `target/release/8bitult-cli`.

On Linux the tool shells out to `bluetoothctl` to make sure the adapter is powered on and scanning, so make sure BlueZ is installed.

## Quick start

Turn the controller on, put it in its keyboard mode, then:

```bash
# See what is bound right now
8bitult-cli read

# Apply the bundled default layout
8bitult-cli attach -p Default

# Save whatever is on the device into a new profile
8bitult-cli save -n "My Layout" -d "for emulators"
```

Add `-v`, `-vv`, or `-vvv` to any command to turn up the logging when something does not connect.

## How it finds the device

The controller advertises under the name `80EL`. On startup the tool starts a scan on every Bluetooth adapter it can see and polls them until that name shows up or the scan times out (15 seconds by default). Both the device name and the timeout live in `src/config/mod.rs` if you need to change them.

## License

MIT. See [LICENSE](LICENSE).

## Contributing

Pull requests are welcome.

1. Fork the repository
2. Create a branch (`git checkout -b feature/my-change`)
3. Commit your changes
4. Push the branch
5. Open a pull request

If you have an 8BitDo device other than the Micro and want to help add support, captures of the official app talking to it are the most useful thing you can share.
