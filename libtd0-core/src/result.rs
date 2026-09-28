// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use core::error::Error;
use core::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct InvalidChunkError {
    chunk_name: String,
    reason: &'static str,
}
impl Display for InvalidChunkError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("chunk '{}' {}", self.chunk_name, self.reason))
    }
}
impl Error for InvalidChunkError {}
impl InvalidChunkError {
    pub fn new(chunk_name: String, reason: &'static str) -> Self {
        Self {
            chunk_name: chunk_name.clone(),
            reason,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeviceModelString {
    model: String,
}
impl Display for DeviceModelString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("unsupported device model: '{}'", self.model))
    }
}
impl Error for DeviceModelString {}
impl From<&[u8]> for DeviceModelString {
    fn from(val: &[u8]) -> Self {
        Self {
            model: String::from_utf8_lossy(&val[0..4]).to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeviceFirmwareVersionString {
    firmware_version: String,
}
impl Display for DeviceFirmwareVersionString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "unsupported device firmware version: '{}'",
            self.firmware_version
        ))
    }
}
impl Error for DeviceFirmwareVersionString {}
impl From<&[u8]> for DeviceFirmwareVersionString {
    fn from(val: &[u8]) -> Self {
        Self {
            firmware_version: String::from_utf8_lossy(&val[0..4]).to_string(),
        }
    }
}
impl From<String> for DeviceFirmwareVersionString {
    fn from(val: String) -> Self {
        Self {
            firmware_version: String::from_utf8_lossy(&val.as_bytes()[0..4]).to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OutOfRangeErrorTD0Decimal {
    min: f64,
    max: f64,
}
impl Display for OutOfRangeErrorTD0Decimal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "out of range. valid range: {:.1} - {:.1}",
            self.min, self.max
        ))
    }
}
impl OutOfRangeErrorTD0Decimal {
    pub fn new(min: impl Into<f64>, max: impl Into<f64>) -> Self {
        Self {
            min: min.into(),
            max: max.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OutOfRangeError {
    min: i64,
    max: i64,
}
impl Display for OutOfRangeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "out of range. valid range: {} - {}",
            self.min, self.max
        ))
    }
}
impl OutOfRangeError {
    pub fn new(min: impl Into<i64>, max: impl Into<i64>) -> Self {
        Self {
            min: min.into(),
            max: max.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum TD0Error {
    ChecksumMismatch,
    ChunkItemParse,
    FileParse(String),
    InvalidChunk(InvalidChunkError),
    InvalidFieldOrType,
    InvalidInput(String),
    OutOfRange(OutOfRangeError),
    OutOfRangeDecimal(OutOfRangeErrorTD0Decimal),
    ReadOnlyField(String),
    UnsupportedDeviceFirmwareVersion(String),
    UnsupportedDeviceModel(String),
}

impl Display for TD0Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let repr: String = match self {
            Self::ChecksumMismatch => "checksum mismatch.".to_string(),
            Self::ChunkItemParse => "unable to parse chunk item.".to_string(),
            Self::InvalidFieldOrType => {
                "unknown field or invalid input type for field.".to_string()
            }
            Self::InvalidInput(msg) => msg.clone(),
            Self::ReadOnlyField(field_name) => format!("field '{field_name}' is read-only."),
            Self::UnsupportedDeviceFirmwareVersion(version) => {
                format!("unsupported firmware version '{version}'")
            }
            Self::UnsupportedDeviceModel(model) => format!("unsupported device model '{model}'"),
            Self::FileParse(err) => err.to_string(),
            Self::InvalidChunk(val) => val.to_string(),
            Self::OutOfRange(val) => val.to_string(),
            Self::OutOfRangeDecimal(val) => val.to_string(),
        };

        write!(f, "{}", repr)
    }
}
impl Error for TD0Error {}

pub type TD0Result<T> = Result<T, TD0Error>;
