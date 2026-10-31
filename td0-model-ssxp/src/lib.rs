// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub(crate) mod common;
pub(crate) mod rev;
pub(crate) mod strings;

// ============================================================================
//                                   PUBLIC API
//      Items below this line are public (re-)exports constituting the public
//      API of this library.
//
//      Only items with public visibility are permitted here. Any change to
//      their signatures (for functions) or publicly accessible members
//      require updating the library version according to The Cargo Book's
//      [SemVer Compatibility] https://doc.rust-lang.org/cargo/reference/semver.html
//      guidelines.
//
pub use common::parse_firmware_version;
pub use rev::{select_chunk_callbacks, valid_chunk_names_by_firmware_version};
