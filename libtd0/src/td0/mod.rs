// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

#[cfg(doc)]
use crate::TD0BackupType;
use td0_core::header::{SZ_HDR_CHUNK, TD0IdChunk, TD0ManifestTag, validate_id_tag};
use td0_core::result::{TD0Error, TD0Result};
use td0_core::{TD0DeviceModel, TD0File};
#[cfg(feature = "model-spdsx-pro")]
use td0_model_ssxp::common::SSXPTD0File;

#[cfg(test)]
mod tests;

fn read_td0_id_chunk(bytes: &[u8]) -> TD0Result<TD0IdChunk> {
    use zerocopy::FromBytes;

    let buf = bytes.get(..SZ_HDR_CHUNK).ok_or_else(|| TD0Error::FileParse(
        "unable to read first chunk".to_string(),
    ))?;

    Ok(TD0IdChunk::read_from_bytes(buf).expect("SZ_HDR_CHUNK is correct."))
}

/// Parses a byte array into a struct that implements the [`TD0File`] trait,
/// returning a `Box<dyn TD0File>` if successful or a [`TD0Error`] otherwise.
///
/// Parser implementations vary based on the [`TD0DeviceModel`] and firmware
/// version stored in the file as part of its backup manifest, `TD0File` aims
/// to provide a sane unified interface for interacting with these
/// implementations.
///
/// # Errors
/// There are numerous reasons why parsing a TD0 file may fail. All errors will be [`TD0Error`] variants.
/// The expected variants are:
///
/// - [`TD0Error::FileParse`] - The backup file is improperly formatted or otherwise not parsable.
/// - [`TD0Error::InvalidChunk`] - A chunk in the file is unparsable or otherwise invalid.
/// - [`TD0Error::UnsupportedDeviceFirmwareVersion`] - The firmware revision of
///   the device that created the backup file is
///   not supported by this library. Support may be added in the future.
/// - [`TD0Error::UnsupportedDeviceModel`] - The device model is not supported by this library.
///   Support may be added in the future.
///
/// # Examples
///
/// ## Successful parsing of a .TD0 file
/// The example file is the smallest file parsable by this library. It has
/// a single data chunk, the `HDRa` backup manifest chunk, which contains the
/// data necessary to determine the device model, firmware version, and
/// [`TD0BackupType`] of the file.
///
/// After parsing the file we call [`TD0File::list_chunks`] to verify the result.
/// ```
/// # use td0::TD0Error;
/// use td0::{TD0File, parse_td0_file};
///
/// const TD0_BYTES : &[u8] = include_bytes!("../../example/data/minimal_v1_10.TD0");
/// let td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
/// assert_eq!(td0file.list_chunks(), vec!["HDRa"]);
/// # Ok::<(), TD0Error>(())
/// ```
///
/// ## Unsuccessful parsing
/// The example buffer here doesn't contain the correct file header signature,
/// resulting in a [`TD0Error::FileParse`] error upon attempting to parse it.
/// ```
/// # use td0::TD0Error;
/// use td0::{TD0File, TD0Result, parse_td0_file};
/// // Whoops, wrong file format! Should be 'TD0a'.
/// const BAD_BYTES : [u8; 16] = [0x0e, 0, b'N', b'E', b'S', 0x1a, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
/// let td0file : TD0Result<Box<dyn TD0File>> = parse_td0_file(&BAD_BYTES);
/// assert!(matches!(td0file, Err(TD0Error::FileParse(_))));
/// # Ok::<(), TD0Error>(())
/// ```
///
pub fn parse_td0_file(bytes: &[u8]) -> TD0Result<Box<dyn TD0File>> {
    use zerocopy::FromBytes;

    let id_chunk = read_td0_id_chunk(bytes)?;
    validate_id_tag(&id_chunk)
        .map_err(|_| TD0Error::FileParse("invalid or missing `TD0a` id chunk.".to_string()))?;

    let pos: usize = size_of::<TD0IdChunk>();
    let next_chunk: TD0ManifestTag =
        TD0ManifestTag::read_from_bytes(bytes.get(pos..pos + SZ_HDR_CHUNK).ok_or_else(||
            TD0Error::FileParse("can't determine TD0 device model.".to_string()),
        )?)
        .map_err(|_| TD0Error::FileParse("can't determine TD0 device model.".to_string()))?;
    // The model of the first chunk should be the model of the entire backup file.
    // Verify that now and the backup file must validate further.
    match next_chunk.model().as_str() {
        #[cfg(feature = "model-spdsx-pro")]
        "SSXP" => Ok(Box::new(SSXPTD0File::try_from_bytes(bytes)?)),
        unexpected_model => Err(TD0Error::UnsupportedDeviceModel(
            unexpected_model.to_string(),
        )),
    }
}

/// Creates a new [`TD0File`] object with the correct implementation based on the
/// `model` and `version` parameters. 
///
/// # Errors
/// 
/// - [`TD0Error::UnsupportedDeviceFirmwareVersion`] - The firmware revision of
///   the device that created the backup file is not supported by this library. 
///   Support may be added in the future.
/// - [`TD0Error::UnsupportedDeviceModel`] - The device model is not supported 
///   by this library. Support may be added in the future.
/// 
pub fn new_td0_file(model: TD0DeviceModel, version: &str) -> TD0Result<Box<dyn TD0File>> {
    match (model, version) {
        #[cfg(feature = "model-spdsx-pro")]
        (TD0DeviceModel::SPDSXPro, "1.10") => Ok(Box::new(SSXPTD0File::new()) as Box<dyn TD0File>),
        #[cfg(feature = "model-spdsx-pro")]
        (TD0DeviceModel::SPDSXPro, "2.0") => todo!(),
        #[cfg(feature = "model-spdsx-pro")]
        (TD0DeviceModel::SPDSXPro, _) => Err(TD0Error::UnsupportedDeviceFirmwareVersion(
            version.to_string(),
        )),

        _ => Err(TD0Error::UnsupportedDeviceModel(model.to_string())),
    }
}
