<!--
SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>

SPDX-License-Identifier: GPL-3.0-or-later
-->

# libtd0: A parsing and editing library for the .TD0 e-kit/sample pad backup file format

This library provides the ability to extract configuration (but not audio) data from .TD0 format backup files (used by
some Roland Sample Pads and e-kits.) It can also edit some data in those files and rearrange
items (such as Kits) within a backup file without the device being connected to the computer.

## Features

- Extract data (kit settings, setlist order, system settings) from backup files.
- Experimental: Edit these same settings.
- Experimental: Reorder items such as kits, or setlist entries.
- **Available offline**: All you need is a copy of the backup .TD0 file. The device does not need to be attached.

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

Install the latest stable version via pip:

```bash
pip install libtd0
```

## Usage

A .TD0 file is comprised of several chunks of data

### Parse a .TD0 file

Read the file in binary mode, then construct a TD0File object from the bytes:

```python
import libtd0

with open('my_backup.TD0', 'rb') as infile:
    raw_bytes = infile.read()

td0file = libtd0.TD0File(data)
# List the chunks in the file to verify it parsed correctly:
print(td0file.list_chunks())

# Outputs similar to (this output varies based on device model, firmware version, and file backup type):
# ['HDRa', 'PVRa', 'KITb', 'KITa', 'STLa', 'WVPa', 'TGLa', 'STPb', 'STPa', 'TRGa', 'CURa']
```

If the file is not the .TD0 format or is otherwise unparsable, a ValueError is raised:

```python
with open('invalid_file.bin', 'rb') as infile:
    invalid_bytes = infile.read()

try:
    td0file = libtd0.TD0File(invalid_bytes)
except ValueError as err:
    print(err)

# prints:
# Unable to parse input buffer: invalid or missing `TD0a` id chunk.
```

### Read specific fields from a chunk

This example will print the Kit names and tempos for all kits in the backup.

```python
import libtd0

with open('my_backup.TD0', 'rb') as infile:
    td0file = libtd0.TD0File(infile.read())

# Get the number of kits:
num_kits = [chunk for chunk in td0file.manifest().chunks if chunk.name == "KITa"][0].num_items

# Use TD0File.get_chunk_item() to retrieve each kit, and .field_value()
# on each kit to get the values.
kit_data = []
for idx in range(num_kits):
    kit = td0file.get_chunk_item("KITa", idx)
    kit_data.append((kit.field_value("name"), kit.field_value("tempo")))

# Print the values, with indexes matching how the device displays them.
for idx, item in enumerate(kit_data):
    print(f'{idx + 1: >3}: name: {item[0]: <16} tempo: {item[1]}')

# Outputs:
#    1: name: Dance            tempo: 120
#    2: name: Lofi HipHop      tempo: 85
#    3: name: Slow Beats       tempo: 92
#    4: name: Grime            tempo: 137
#    5: name: House            tempo: 120
#  ...
#  199: name: USER KIT         tempo: 120
#  200: name: USER KIT         tempo: 120
```

### Experimental: Modify a chunk field value

**NOTE** Editing features currently exist solely for the benefit of the developer(s) of this library or
others interested in the file format. Do **NOT** load files modified with this library into an acatual
hardware device unless you're positive that you understand the risks involved and can deal with any
failures this may cause. **You have been warned**

#### Change a kit name

```python
from libtd0 import TD0File

with open('my_backup.TD0', 'rb') as infile:
    td0file = TD0File(infile.read())

## Get the kit item.
kit = td0file.get_chunk_item("KITa", 0)
print(kit.field_value("name"))
# prints: "Dance"

kit.set_field_value("name", "Safety Dance")
# NOTE: The original td0file has not changed at this point. get_chunk_item()
# returns a *copy* of the chunk item (kit), not a reference to it.
# chunk_item_replace() must be called with the modified item to update the
# TD0File object.

print(kit.field_value("name"))
# prints: "Safety Dance"

## Update the TD0File.
td0file.chunk_item_replace("KITa", 0, kit)
print(td0.get_chunk_item("KITa", 0).field_value("name"))
# prints: "Safety Dance"

## Write the modified bytes to a file.

# NOTE: .finalize() must be called before calling to_bytes() on a TD0File that
# has been modified. A ValueError will be raised otherwise.
td0file.finalize()

with open('my_backup.edited.TD0', 'wb') as outfile:
    outfile.write(td0file.to_bytes())

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

## Changelog

See the [entire project changelog](https://github.com/natepugh/libtd0#changelog), which includes changes
for Python `libtd0` as well as the underlying Rust library.
