// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

#[cfg(doc)]
use crate::TD0BackupType;
use td0_core::{TD0DeviceModel, TD0Error, TD0File, TD0Result};

#[cfg(feature = "model-spdsx-pro")]
use td0_model_ssxp::{
    select_chunk_callbacks as spdsx_pro_select_chunk_callbacks,
    valid_chunk_names_by_firmware_version as spdsx_pro_valid_chunk_names_callback,
};

#[cfg(test)]
mod tests;

/// Parses a byte array into a [`TD0File`] struct, returning a [`TD0Error`]
/// if not successful.
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
/// let td0file : TD0File = parse_td0_file(TD0_BYTES)?;
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
/// let td0file : TD0Result<TD0File> = parse_td0_file(&BAD_BYTES);
/// assert!(matches!(td0file, Err(TD0Error::FileParse(_))));
/// # Ok::<(), TD0Error>(())
/// ```
///
pub fn parse_td0_file(bytes: &[u8]) -> TD0Result<TD0File> {
    let (model, _firmware) = TD0File::parse_model_and_firmware(bytes)?;
    match model {
        #[cfg(feature = "model-spdsx-pro")]
        TD0DeviceModel::SPDSXPro => TD0File::parse_with(
            bytes,
            spdsx_pro_select_chunk_callbacks,
            spdsx_pro_valid_chunk_names_callback,
        ),
        _ => TD0File::try_from(bytes),
    }
}

/// Creates a new [`TD0File`] object with the correct implementation based on the
/// `model` and `version` parameters.
///
/// # Warning
/// This function exists solely as a convenience to developers and testers of this library.
/// DO NOT attmept to load a backup created by this function into your hardware device.
///
/// # Errors
///
/// - [`TD0Error::UnsupportedDeviceFirmwareVersion`] - The firmware revision of
///   the device that created the backup file is not supported by this library.
///   Support may be added in the future.
/// - [`TD0Error::UnsupportedDeviceModel`] - The device model is not supported
///   by this library. Support may be added in the future.
///
/// # Example
/// ```
/// # use td0::TD0Error;
/// use td0::{TD0DeviceModel, TD0File, new_td0_file};
///
/// let td0file : TD0File = new_td0_file(TD0DeviceModel::SPDSXPro, "2.00")?;
/// assert_eq!(td0file.list_chunks(), vec!["HDRa"]);
/// # Ok::<(), TD0Error>(())
/// ```
pub fn new_td0_file(model: TD0DeviceModel, version: &str) -> TD0Result<TD0File> {
    match (model, version) {
        #[cfg(feature = "model-spdsx-pro")]
        (TD0DeviceModel::SPDSXPro, _) => TD0File::new_with(
            version,
            spdsx_pro_select_chunk_callbacks,
            spdsx_pro_valid_chunk_names_callback,
        ),
        _ => Err(TD0Error::UnsupportedDeviceModel(model.to_string())),
    }
}
