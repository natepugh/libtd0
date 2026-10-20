// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

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
    let major = version.split(".").next().unwrap_or("0");
    match major {
        "2" => "2.00",
        "1" => "1.10",
        _ => "0.00",
    }
}
