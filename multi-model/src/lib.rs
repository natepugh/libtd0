use libtd0_core::TD0File;
use libtd0_core::header::{SZ_HDR_CHUNK, TD0IdChunk, TD0ManifestTag, validate_id_tag};
use libtd0_core::result::TD0Error::InvalidTD0File;
use libtd0_core::result::{TD0Error, TD0Result};
#[cfg(feature = "model-ssxp")]
use model_ssxp::common::SSXPTD0File;

/// Parses a byte array into a `TD0File` struct.
///
/// # Errors
/// There are numerous reasons why parsing a TD0 file may fail. All errors will be `TD0Error` variants.
/// The following are the expected variants:
///
/// - `TD0Error::UnsupportedDeviceModel` - The device model is not supported by this library.
///   Support may be added in the future.
/// - `TD0Error::UnsupportedFirmwareVersion` - The firmware revision of the device that created the backup file is
///   not supported by this library. Support may be added in the future.
/// - `TD0Error::InvalidTD0File` - The backup file is improperly formatted or otherwise not parsable.
pub fn parse_td0_file(bytes: &[u8]) -> TD0Result<Box<dyn TD0File>> {
    use zerocopy::FromBytes;

    let id_chunk: TD0IdChunk = TD0IdChunk::read_from_bytes(
        bytes
            .get(..SZ_HDR_CHUNK)
            .ok_or(InvalidTD0File("unable to read first chunk."))?,
    )
    .map_err(|_| TD0Error::InvalidTD0File("unable to read first chunk."))?;
    validate_id_tag(&id_chunk)
        .map_err(|_| TD0Error::InvalidTD0File("invalid or missing `TD0a` id chunk."))?;
    let pos: usize = size_of::<TD0IdChunk>();
    let next_chunk: TD0ManifestTag = TD0ManifestTag::read_from_bytes(
        bytes
            .get(pos..pos + SZ_HDR_CHUNK)
            .ok_or(InvalidTD0File("can't determine TD0 device model."))?,
    )
    .map_err(|_| TD0Error::InvalidTD0File("can't determine TD0 device model."))?;
    // The model of the first chunk should be the model of the entire backup file.
    // Verify that now and the backup file must validate further.
    match next_chunk.model().as_str() {
        #[cfg(feature = "model-ssxp")]
        "SSXP" => Ok(Box::new(SSXPTD0File::from_bytes(bytes.into())?)),
        _ => Err(TD0Error::UnsupportedDeviceModel),
    }
}
