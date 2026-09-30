// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use libtd0_core::header::{SZ_HDR_CHUNK, TD0IdChunk, TD0ManifestTag, validate_id_tag};
use libtd0_core::result::{TD0Error, TD0Result};
use libtd0_core::{TD0DeviceModel, TD0File};
#[cfg(feature = "model-ssxp")]
use model_ssxp::common::SSXPTD0File;

fn read_td0_id_chunk(bytes: &[u8]) -> TD0Result<TD0IdChunk> {
    use zerocopy::FromBytes;

    let buf = bytes.get(..SZ_HDR_CHUNK).ok_or(TD0Error::FileParse(
        "unable to read first chunk".to_string(),
    ))?;

    Ok(TD0IdChunk::read_from_bytes(buf).expect("SZ_HDR_CHUNK is correct."))
}

/// Parses a byte array into a `TD0File` struct.
///
/// # Errors
/// There are numerous reasons why parsing a TD0 file may fail. All errors will be `TD0Error` variants.
/// The following are the expected variants:
///
/// - `TD0Error::UnsupportedDeviceModel` - The device model is not supported by this library.
///   Support may be added in the future.
/// - `TD0Error::UnsupportedDeviceFirmwareVersion` - The firmware revision of the device that created the backup file is
///   not supported by this library. Support may be added in the future.
/// - `TD0Error::FileParse` - The backup file is improperly formatted or otherwise not parsable.
pub fn parse_td0_file(bytes: &[u8]) -> TD0Result<Box<dyn TD0File>> {
    use zerocopy::FromBytes;

    let id_chunk = read_td0_id_chunk(bytes)?;
    validate_id_tag(&id_chunk)
        .map_err(|_| TD0Error::FileParse("invalid or missing `TD0a` id chunk.".to_string()))?;

    let pos: usize = size_of::<TD0IdChunk>();
    let next_chunk: TD0ManifestTag =
        TD0ManifestTag::read_from_bytes(bytes.get(pos..pos + SZ_HDR_CHUNK).ok_or(
            TD0Error::FileParse("can't determine TD0 device model.".to_string()),
        )?)
        .map_err(|_| TD0Error::FileParse("can't determine TD0 device model.".to_string()))?;
    // The model of the first chunk should be the model of the entire backup file.
    // Verify that now and the backup file must validate further.
    match next_chunk.model().as_str() {
        #[cfg(feature = "model-ssxp")]
        "SSXP" => Ok(Box::new(SSXPTD0File::try_from_bytes(bytes)?)),
        unexpected_model => Err(TD0Error::UnsupportedDeviceModel(
            unexpected_model.to_string(),
        )),
    }
}

pub fn new_td0_file(model: TD0DeviceModel, version: &str) -> TD0Result<Box<dyn TD0File>> {
    match (model, version) {
        #[cfg(feature = "model-ssxp")]
        (TD0DeviceModel::SPDSXPro, "1.10") => Ok(Box::new(SSXPTD0File::new()?) as Box<dyn TD0File>),
        #[cfg(feature = "model-ssxp")]
        (TD0DeviceModel::SPDSXPro, "2.0") => todo!(),
        #[cfg(feature = "model-ssxp")]
        (TD0DeviceModel::SPDSXPro, _) => Err(TD0Error::UnsupportedDeviceFirmwareVersion(
            version.to_string(),
        )),

        _ => Err(TD0Error::UnsupportedDeviceModel(model.to_string())),
    }
}
