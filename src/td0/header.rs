use zerocopy::{FromBytes, LittleEndian, U16, U32};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::td0::{
    chunks::ChunkHeader,
    result::{TD0Error, TD0Result},
};
use std::{collections::HashMap, fmt};

pub const SZ_HDR_CHUNK: usize = 16;
pub const OFFSET_BYTES_REMAINING: u16 = 2;
pub const TDO_MAGIC: [u8; 4] = [b'T', b'D', b'0', b'a'];

#[derive(FromBytes, Debug, Immutable, IntoBytes, KnownLayout)]
#[repr(C, packed)]
pub struct TD0IdChunk {
    pub bytes_remaining: U16<LittleEndian>,
    pub magic: [u8; 4],
    pub unknown: [u8; 10],
}

impl fmt::Display for TD0IdChunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TD0IdChunk: bytes_remaining: {} magic: \"{}\" unknown: {:?}",
            self.bytes_remaining,
            String::from_utf8_lossy(&self.magic).to_string(),
            self.unknown
        )
    }
}

impl TD0IdChunk {
    pub fn get_bytes_remaining(&self) -> u16 {
        return self.bytes_remaining.get();
    }
}

#[derive(Clone, Copy, Debug, FromBytes, Immutable, IntoBytes, KnownLayout)]
#[repr(C, packed)]
pub struct TD0ManifestTag {
    pub tag: [u8; 4],
    pub model: [u8; 4],
    pub pos: U32<LittleEndian>,
    pub length: U32<LittleEndian>,
}

impl TD0ManifestTag {
    pub fn get_model(&self) -> String {
        return String::from_utf8_lossy(&self.model).to_string();
    }

    pub fn get_tag(&self) -> String {
        return String::from_utf8_lossy(&self.tag).to_string();
    }

    pub fn get_pos(&self) -> u32 {
        return self.pos.get();
    }

    pub fn get_length(&self) -> u32 {
        return self.length.get();
    }
}

impl fmt::Display for TD0ManifestTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} : model: {}  pos: {}  length: {}",
            self.get_tag(),
            self.get_model(),
            self.pos,
            self.length,
        )
    }
}

pub struct TD0Header {
    pub id_chunk: TD0IdChunk,
    pub manifest: Vec<TD0ManifestTag>,
    pub tags: HashMap<String, usize>,
}

impl TD0Header {
    pub fn init_from_id_chunk(id_chunk: TD0IdChunk, data: &[u8]) -> TD0Result<Self> {
        let mut pos: usize = 0;
        pos += size_of::<TD0IdChunk>();

        let mut manifest: Vec<TD0ManifestTag> = Vec::new();
        let mut tags: HashMap<String, usize> = HashMap::new();
        let header_len = (id_chunk.bytes_remaining.get() + OFFSET_BYTES_REMAINING) as usize;
        let data_len = data.len();

        let mut idx: usize = 0;
        while pos < header_len && pos < data_len {
            let Ok(chunk) = TD0ManifestTag::read_from_bytes(&data[pos..pos + SZ_HDR_CHUNK]) else {
                return Err(TD0Error::InvalidHeaderError);
            };
            if tags.contains_key(&chunk.get_tag()) {
                // Duplicate chunks!
                return Err(TD0Error::DuplicateChunkError {
                    chunk_name: chunk.get_tag(),
                });
            }
            tags.insert(chunk.get_tag(), idx);
            manifest.push(chunk);

            pos += SZ_HDR_CHUNK;
            idx += 1;
        }

        Ok(Self {
            id_chunk: id_chunk,
            manifest: manifest,
            tags: tags,
        })
    }

    pub fn get_chunk_header(&self, bytes: &[u8], tag: &str) -> TD0Result<ChunkHeader> {
        let Some(manifest_tag) = self.get_tag(tag) else {
            return Err(TD0Error::TagNotFoundError {
                tag_name: tag.to_string(),
            });
        };
        let chunk_header = ChunkHeader::read_from_bytes(
            &bytes[manifest_tag.pos.get() as usize..manifest_tag.pos.get() as usize + SZ_HDR_CHUNK],
        )
        .map_err(|_| TD0Error::InvalidChunkError {
            chunk_name: tag.to_string(),
        })?;
        Ok(chunk_header)
    }

    pub fn get_tag(&self, tag: &str) -> Option<&TD0ManifestTag> {
        if !self.tags.contains_key(tag) {
            return None;
        }

        if let Some(tag_index) = self.tags.get(tag)
            && let Some(tag_item) = self.manifest.get(*tag_index)
        {
            return Some(tag_item);
        }

        None
    }

    pub fn len(&self) -> usize {
        return self.id_chunk.bytes_remaining.get() as usize + OFFSET_BYTES_REMAINING as usize;
    }
}
