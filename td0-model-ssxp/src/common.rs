// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use td0_core::HDRaItem;
use td0_core::{TD0Error, TD0Result};

/// Backup header tag for whole system backup.
const BACKUP_TAG_SYSTEM: &str = "SSXPROBK";

const FACTORY_BACKUP_FIRMWARE: &str = "9.67";
const FACTORY_BACKUP_BUILD: &str = "0067";
const FACTORY_BACKUP_SERIAL: &str = "0142345";

pub fn parse_firmware_version(backup_header: &HDRaItem) -> TD0Result<String> {
    let declared = backup_header.firmware();

    if declared.as_str() == FACTORY_BACKUP_FIRMWARE {
        if !(backup_header.build().as_str() == FACTORY_BACKUP_BUILD
            && backup_header.device_serial() == FACTORY_BACKUP_SERIAL
            && backup_header.tag() == BACKUP_TAG_SYSTEM)
        {
            Err(TD0Error::FileParse(
                "unable to parse firmware version.".to_string(),
            ))
        } else {
            // Extract actual firmware version from the 'name' field.
            let val = backup_header.name();
            let val = val
                .trim()
                .strip_prefix("Contents_V")
                .ok_or(TD0Error::FileParse(
                    "uanble to parse firmware version.".to_string(),
                ))?;

            for char in val.chars() {
                if !char.is_ascii_digit() {
                    return Err(TD0Error::FileParse(
                        "unable to parse firmware version.".to_string(),
                    ));
                }
            }
            let mut val = val.to_string();
            val.insert(1, '.');

            Ok(val
                .get(0..4)
                .ok_or(TD0Error::FileParse(
                    "unable to parse firmware version.".to_string(),
                ))?
                .to_string())
        }
    } else {
        Ok(declared)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_firmware_normal() {
        let mut backup_chunk = HDRaItem::default();
        backup_chunk.set_name("TEST-BACKUP").unwrap();
        backup_chunk.set_firmware("2.00").unwrap();

        assert_eq!(
            parse_firmware_version(&backup_chunk),
            Ok("2.00".to_string())
        );
    }

    #[test]
    fn test_parse_firmware_factory_backup() {
        let mut backup_chunk = HDRaItem::default();
        backup_chunk.set_tag(BACKUP_TAG_SYSTEM).unwrap();
        backup_chunk.set_name("Contents_V1020  ").unwrap();
        backup_chunk.set_firmware(FACTORY_BACKUP_FIRMWARE).unwrap();
        backup_chunk.set_build(FACTORY_BACKUP_BUILD).unwrap();
        backup_chunk
            .set_device_serial(FACTORY_BACKUP_SERIAL)
            .unwrap();

        assert_eq!(
            parse_firmware_version(&backup_chunk),
            Ok("1.02".to_string())
        );
    }
}
