// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Clone, Debug, PartialEq)]
pub enum TD0Error {
    InvalidChunk(&'static str),
    InvalidChunkItem,
    InvalidFieldOrType,
    InvalidInput,
    InvalidInputWithMessage(String),
    InvalidTD0File(&'static str),
    OutOfRange,
    ReadOnlyField,
    UnknownChunk,
    UnsupportedDeviceModel,
    UnsupportedFirmwareVersion,
}

pub type TD0Result<T> = Result<T, TD0Error>;

impl core::fmt::Display for TD0Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let repr: String = match self {
            Self::InvalidChunk(err) => format!("invalid chunk `{err}`"),
            Self::InvalidChunkItem => "invalid chunk item.".to_string(),
            Self::InvalidFieldOrType => {
                "unknown field or invalid input type for field.".to_string()
            }
            Self::InvalidInput => "input is invalid for field.".to_string(),
            Self::InvalidInputWithMessage(err) => format!("invalid input: {err}"),
            Self::InvalidTD0File(err) => format!("TD0 file invalid: {err}"),
            Self::OutOfRange => "out of range.".to_string(),
            Self::ReadOnlyField => "field is read only".to_string(),
            Self::UnknownChunk => "unknown chunk.".to_string(),
            Self::UnsupportedDeviceModel => "unknown device model.".to_string(),
            Self::UnsupportedFirmwareVersion => {
                "firmware version not supported for this model.".to_string()
            }
        };
        write!(f, "{}", repr)
    }
}
