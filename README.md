<!--
SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>

SPDX-License-Identifier: GPL-3.0-or-later
-->

# td0: A parsing and editing library for the .TD0 e-kit/sample pad backup file format.

This library provides the ability to extract configuration (but not audio) data from .TD0 format backup files (used by 
some Roland Sample Pads and e-kits.) It can also edit some data in those files and rearrange
items (such as Kits) within a backup file without the device being connected to the computer.

## Features

- Extract data (kit settings, setlist order, system settings) from backup files.
- Experimental: Edit these same settings.
- Experimental: Reorder items such as kits, or setlist entries.
- **Available offline**: All you need is a copy of the backup .TD0 file. The device does not need to be attached.
- Rust-compatible .dylib, may be dynamically linked to your rust program.
- Python bindings
- Reference CLI for editing via the command-line: `td0file`

**WARNING** Editing a TD0 backup file is experimental and may lead to invalid configuration which, when loaded into
a device could unexpected behavior up to and including hard-locking (unrecoverably "bricking") the device. Editing is
currently **NOT** recommended by the author(s) of this library.

Should you choose to do so,
- Create a whole system backup first, and save it somewhere where you won't accidentally edit it.
- Download the Factory Reset firmware from Roland ahead of time, in case you need to reset your configuration.
- After loading an experimental backup onto your device, be sure to test all features you want to use well ahead
  of any kind of performance. The author(s) are NOT RESPONSBLE for any damage caused to your device, career, 
  or reputation.

## Installation

### Building from source
This project can built with Rust "2024" edition or up. 

**Prerequisites**
- Rust installed locally: https://rust-lang.org/learn/get-started/

**Build Steps**
- Either clone the source code or download a .zip file from Github and extract into a directory.
    - e.g. `git clone https://github.com/natepugh/libtd0.git`
- Navigate to that directory and use cargo to build:
    - `cargo build --bins --release`

If you want to build the Python bindings too:
- Install [Maturin](https://github.com/pyo3/maturin)
- Navigate to the `libtd0/td0-python` directory
- Activate your virtualenv or use uv to invoke maturin
- Build:
    - `maturin build --generate-stubs`

The compiled binaries are saved in the `libtd0/target/release` directory by default. 

### Running
After installing the binary:

- View its usage by running the `--help` command:
    - `td0file --help`
- Display the file manifest information:
    ```bash
    td0file -f BCKUP-ALL.TD0
    ```
    ```
    Backup Name: BACKUP-APP
    Backup Type: System
    Checksum from File   : a57bd129d71a9fc33ee1e438391eb5a4
    Checksum (calculated): a57bd129d71a9fc33ee1e438391eb5a4
    Device Model: SPDSXPro
    Device Firmware: 1.10
    Device Firmware Build: 0083
    Device Serial: XXXXXXX 
    Actual Size: 3983408
    Calculated Size: 3983408
    Chunks:
        HDRa:  pos: 144  size: 80  num_items: 1  item_size: 64
        KITa:  pos: 224  size: 691216  num_items: 200  item_size: 3456
        STLa:  pos: 691440  size: 8720  num_items: 32  item_size: 272
        WVPa:  pos: 700160  size: 3280180  num_items: 20001  item_size: 164
        TGLa:  pos: 3980340  size: 2064  num_items: 128  item_size: 16
        STPa:  pos: 3982404  size: 648  num_items: 1  item_size: 632
        TRGa:  pos: 3983052  size: 316  num_items: 1  item_size: 300
        CURa:  pos: 3983368  size: 24  num_items: 1  item_size: 8
    ```

    JSON output is also supported for most operations:

    ```bash
    tdofile -f BCKUP-ALL.TD0 --output-format=json
    ```
    ```
    {"backup_name":"BACKUP-APP","backup_type":"System", ...etc.}
    ```
- Dump specific fields as .csv (for import into a spreadsheet):
    Dump the "name" and "tempo" fields for all kits.

    ```bash
    td0file -f BCKUP-ALL.TD0 chunk-values KITa --fields=name,tempo --output-format=csv
    ```
    ```
    index,name,tempo
    1,Dance,120.0
    2,Lofi HipHop,85.0
    3,Slow Beats,92.0
    4,Grime,137.0
    5,House,120.0
    ...
    198,USER KIT,120.0
    199,USER KIT,120.0
    200,USER KIT,120.0
    ```

## Support by model
### SPD-SX Pro
Supported firmware versions: 1.10, 2.00

Read/Write 
- Per Kit: 1800+ settings (not all have been discovered- ~400 settings remain unaccounted for.)
- Setlist: Name + kit order
- LED Colors: R, G, B values for all 16 color presets.
- Tag names
- Wave: start/end/loop points, wav name, wav filename.

# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - Initial release
- `td0file` tool works in most cases.
- `pytd0` Python bindings work in a proof-of-concept.
- **Known Bugs**
    - Roland Factory restore backup files aren't considered valid.
    - `td0file` binary emits the wrong error in the above case-
      "unknown chunk" when a chunk access is attempted, should be
      "invalid firmware".
