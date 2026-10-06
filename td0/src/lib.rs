// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! # td0, a library for viewing and editing the .TD0 e-kit/sample pad backup file format.
//!
//! ## Compatible models and firmware revisions
//!
//! |    Model   | Firmware version |
//! | ---------- | ---------------- |
//! | SPD-SX Pro |       1.10       |
//! | SPD-SX Pro |       2.0        |
//!
#![allow(clippy::allow_attributes_without_reason)]
#![allow(clippy::pub_use)]
pub(crate) mod td0;

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
pub use td0::{new_td0_file, parse_td0_file};
pub use td0_core::{
    ChunkManifest, TD0BackupType, TD0ChunkItem, TD0DeviceModel, TD0File, TD0Manifest, TD0Value,
    TD0ValueRaw,
};
pub use td0_core::{TD0Error, TD0Result};
