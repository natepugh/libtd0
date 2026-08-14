use crate::td0::header::{SZ_HDR_CHUNK, TD0_MAGIC, TD0IdChunk, TD0ManifestTag};
use crate::td0::model::ssxp::common::SSXPTD0File;
use libtd0_core::TD0File;
use libtd0_core::result::{TD0Error, TD0Result};

pub mod header;
mod model;
mod tests;

pub fn parse_td0_file(bytes: &[u8]) -> TD0Result<Box<dyn TD0File>> {
    use zerocopy::FromBytes;

    let id_chunk: TD0IdChunk = TD0IdChunk::read_from_bytes(&bytes[..SZ_HDR_CHUNK])
        .map_err(|_| TD0Error::InvalidTD0File("unable to read first chunk."))?;
    validate_id_tag(&id_chunk)
        .map_err(|_| TD0Error::InvalidTD0File("invalid or missing `TD0a` id chunk."))?;
    let pos: usize = size_of::<TD0IdChunk>();
    let next_chunk: TD0ManifestTag =
        TD0ManifestTag::read_from_bytes(&bytes[pos..pos + SZ_HDR_CHUNK])
            .map_err(|_| TD0Error::InvalidTD0File("can't determine TD0 device model."))?;
    // The model of the first chunk should be the model of the entire backup file.
    // Verify that now and the backup file must validate further.
    match next_chunk.model().as_str() {
        "SSXP" => Ok(Box::new(SSXPTD0File::from_bytes(bytes.into())?)),
        _ => Err(TD0Error::UnknownDeviceModel),
    }
}

pub(crate) fn validate_id_tag(tag: &TD0IdChunk) -> TD0Result<()> {
    if *tag.magic_raw() == TD0_MAGIC {
        Ok(())
    } else {
        Err(TD0Error::InvalidTD0File(
            "invalid or missing `TD0a` header chunk.",
        ))
    }
}
