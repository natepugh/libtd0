use thiserror::Error;

#[derive(Error, Debug)]
pub enum TD0Error {
    #[error("Checksum mismatch")]
    ChecksumMismatchError,

    #[error("Can't convert {field} to native type: {reason}")]
    ConvertToNativeTypeError { field: String, reason: String },

    #[error("Value out of range during conversion. Allowed: {min}...{max}")]
    ConvertRangeError{ min: i128, max: i128 },

    #[error("Unable to convert to `{type_name}` from String value: `{value}`")]
    ConvertFromStringError{ type_name: String, value: String },

    #[error("Duplicate chunk data for chunk {chunk_name}")]
    DuplicateChunkError { chunk_name: String },

    #[error("Firmware version {version} not implemented")]
    FirmwareVersionNotImplementedError { version: String },

    #[error("Invalid chunk data for chunk {chunk_name}")]
    InvalidChunkError { chunk_name: String },

    #[error("Invalid TD0 file header")]
    InvalidHeaderError,

    #[error("Invalid TD0 tag")]
    InvalidTD0TagError,

    #[error("Item index out of bounds: the len is {len} but the index is {index}")]
    ItemIndexError { len: usize, index: usize },

    #[error("TD0 tag not found: {tag_name}")]
    TagNotFoundError { tag_name: String },
}

pub type TD0Result<T> = Result<T, TD0Error>;
