// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use td0_core::ChunkItemAccessorCallbacks;

pub mod v1_10;
pub mod v2_00;

/// So far the observed pattern has been that:
/// 1) New chunk names are only added to major version increments
/// 2) No known chunk item values change between versions (even between
///    major versions.)
///
/// If information is found contrary to the above, this will need to become
/// more sophisticated.
pub fn fuzzy_match_firmware(version: &str) -> &'static str {
    let major = version.split('.').next().unwrap_or("0");
    match major {
        "2" => "2.00",
        "1" => "1.10",
        _ => "0.00",
    }
}

pub fn select_chunk_callbacks(
    chunk_name: &str,
    version: &str,
) -> Option<ChunkItemAccessorCallbacks> {
    match fuzzy_match_firmware(version) {
        "2.00" => v2_00::build_chunk_callbacks(chunk_name),
        "1.10" => v1_10::build_chunk_callbacks(chunk_name),
        _ => Some(ChunkItemAccessorCallbacks::default()),
    }
}

pub fn valid_chunk_names_by_firmware_version(version: &str) -> Option<&'static [&'static str]> {
    match fuzzy_match_firmware(version) {
        "2.00" => Some(&v2_00::CHUNK_NAMES),
        "1.10" => Some(&v1_10::CHUNK_NAMES),
        _ => None,
    }
}
