// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use core::error::Error;
use core::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq)]
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
    pub fn new(chunk_name: impl Into<String>, reason: &'static str) -> Self {
        Self {
            chunk_name: chunk_name.into(),
            reason,
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

#[derive(Clone, Debug, Eq, PartialEq)]
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

/// The Error type for all errors emitted by this library.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum TD0Error {
    /// The wrong data type was used when attempting to set a value.
    DataType(String),
    /// The checksum stored in the file does not match the file data.
    ChecksumMismatch,
    /// The requested [`TD0ChunkItem`](super::TD0ChunkItem) is invalid or otherwise unparsable.
    ChunkItemParse,
    /// The chunks aren't a comparable type (for e.g. diff).
    ChunksNotComparable,
    /// The chunk is duplicated.
    DuplicateChunk(String),
    /// The file is unable to be parsed into a [`TD0File`](super::TD0File).
    FileParse(String),
    /// The file is too large for limits inherent to the file type.
    FileTooLarge,
    /// The requested chunk is invalid, missing, or otherwise unparsable.
    InvalidChunk(InvalidChunkError),
    /// The chunk name provided is not a valid one.
    InvalidChunkName,
    /// The requested [`TD0ChunkItem`](super::TD0ChunkItem) field doesn't exist.
    InvalidField(String),
    /// The supplied input isn't valid.
    InvalidInput(String),
    /// The string provided isn't entirely ASCII printable characters.
    NotAsciiPrintable,
    /// This file type (default, most likely) does not support returning item references.
    OperationNotSupported,
    /// The supplied input isn't in the correct range.
    OutOfRange(OutOfRangeError),
    /// The supplied input isn't in the correct range (Decimal input).
    OutOfRangeDecimal(OutOfRangeErrorTD0Decimal),
    /// The field is Read-Only and can't be edited.
    ReadOnlyField(String),
    /// The firmware version is not supported by this library. Support may be added in the future.
    UnsupportedDeviceFirmwareVersion(String),
    /// The device model is not supported by this library. Support may be added in the future.
    UnsupportedDeviceModel(String),
    /// A validaton check failed.
    ValidationFailed(String),
}

impl Display for TD0Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let repr: String = match self {
            Self::DataType(message) | Self::ValidationFailed(message) => message.clone(),
            Self::ChecksumMismatch => "checksum mismatch.".to_string(),
            Self::ChunkItemParse => "unable to parse chunk item.".to_string(),
            Self::ChunksNotComparable => "chunks are not comparable.".to_string(),
            Self::DuplicateChunk(chunk_name) => format!("duplicate chunk '{chunk_name}'"),
            Self::FileParse(err) => err.clone(),
            Self::FileTooLarge => "the file is too large to complete this operation.".to_string(),
            Self::InvalidChunk(val) => val.to_string(),
            Self::InvalidChunkName => "invalid chunk name.".to_string(),
            Self::InvalidField(field) => format!("unknown field '{field:.128}'"),
            Self::InvalidInput(msg) => msg.clone(),
            Self::NotAsciiPrintable => "not an ascii printable string.".to_string(),
            Self::OperationNotSupported => "not supported with this file type.".to_string(),
            Self::OutOfRange(val) => val.to_string(),
            Self::OutOfRangeDecimal(val) => val.to_string(),
            Self::ReadOnlyField(field_name) => format!("field '{field_name}' is read-only."),
            Self::UnsupportedDeviceFirmwareVersion(version) => {
                format!("unsupported firmware version '{version}'")
            }
            Self::UnsupportedDeviceModel(model) => format!("unsupported device model '{model}'"),
        };

        write!(f, "{repr}")
    }
}
impl Error for TD0Error {}

/// A result type for functons that return data from this library.
/// All errors are [`TD0Error`].
pub type TD0Result<T> = Result<T, TD0Error>;

impl TD0Error {
    pub fn file_parse_error(message: impl Into<String>) -> Self {
        Self::FileParse(message.into())
    }

    pub fn unknown_chunk_error(chunk_name: impl Into<String>) -> Self {
        Self::InvalidChunk(InvalidChunkError {
            chunk_name: chunk_name.into(),
            reason: "unknown chunk.",
        })
    }

    pub fn invalid_chunk_error(chunk_name: impl Into<String>, reason: &'static str) -> Self {
        Self::InvalidChunk(InvalidChunkError {
            chunk_name: chunk_name.into(),
            reason,
        })
    }

    pub fn out_of_range_error(min: impl Into<i64>, max: impl Into<i64>) -> Self {
        Self::OutOfRange(OutOfRangeError {
            min: min.into(),
            max: max.into(),
        })
    }

    pub fn out_of_range_error_from_usize(min: usize, max: usize) -> Self {
        let Ok(i64_min) = i64::try_from(min) else {
            return Self::DataType(
                "min value of too great a magnitude to fit in a i64!".to_string(),
            );
        };

        let Ok(i64_max) = i64::try_from(max) else {
            return Self::DataType(
                "max value of too great a magnitude to fit in a i64!".to_string(),
            );
        };

        Self::OutOfRange(OutOfRangeError {
            min: i64_min,
            max: i64_max,
        })
    }

    pub fn out_of_range_error_u32() -> Self {
        Self::OutOfRange(OutOfRangeError::new(0, u32::MAX))
    }

    pub fn out_of_range_error_decimal(min: impl Into<f64>, max: impl Into<f64>) -> Self {
        Self::OutOfRangeDecimal(OutOfRangeErrorTD0Decimal::new(min.into(), max.into()))
    }

    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::ValidationFailed(message.into())
    }
}
