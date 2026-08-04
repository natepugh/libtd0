use std::collections::{HashMap, HashSet};

use zerocopy::FromBytes;

use crate::td0::{
    chunks::{Chunk, ChunkHeader, ChunkItem, HDRaItem, KITaItem, CURaItem},
    header::{OFFSET_BYTES_REMAINING, SZ_HDR_CHUNK, TD0IdChunk, TD0ManifestTag, TDO_MAGIC},
    result::{TD0Error, TD0Result},
};

pub mod chunks;
pub mod header;
pub mod result;
mod strings;
mod tests;

fn parse_manifest(bytes: &[u8]) -> TD0Result<Vec<TD0ManifestTag>> {
    let mut manifest = Vec::<TD0ManifestTag>::new();
    let mut pos: usize = 0;
    while pos < bytes.len() {
        manifest.push(
            TD0ManifestTag::read_from_bytes(&bytes[pos..pos + SZ_HDR_CHUNK])
                .map_err(|_| TD0Error::InvalidHeaderError)?,
        );
        pos += SZ_HDR_CHUNK;
    }
    Ok(manifest)
}

fn build_tag_indexes(tags: &Vec<TD0ManifestTag>) -> TD0Result<HashMap<String, usize>> {
    let mut indexes: HashMap<String, usize> = HashMap::<String, usize>::new();
    for (idx, tag) in tags.iter().enumerate() {
        // let tag_name = str::from_utf8(&tag.tag).map_err(|e| { TD0Error::InvalidTD0TagError })?;
        let tag_name = String::from_utf8_lossy(&tag.tag);
        indexes.insert(tag_name.to_string().to_owned(), idx);
    }

    Ok(indexes)
}

fn validate_id_tag(tag: &TD0IdChunk) -> TD0Result<()> {
    if tag.magic == TDO_MAGIC {
        Ok(())
    } else {
        Err(TD0Error::InvalidTD0TagError)
    }
}

fn validate_manifest(manifest: &Vec<TD0ManifestTag>) -> TD0Result<()> {
    let mut unique: HashSet<String> = HashSet::<String>::new();
    for tag_name in manifest.iter().map(|tag| tag.get_tag()) {
        if unique.contains(&tag_name) {
            return Err(TD0Error::DuplicateChunkError {
                chunk_name: tag_name,
            });
        }
        unique.insert(tag_name);
    }
    Ok(())
}

pub struct TD0File {
    buf: Vec<u8>,
    // pub header: header::TD0Header,
    // chunks: Vec<Box<dyn chunk::Chunk>>,
    pub manifest: Vec<TD0ManifestTag>,
    tag_indexes: HashMap<String, usize>,
    chunk_headers: HashMap<String, ChunkHeader>,
    pub chunks: HashMap<String, Chunk>,
}

impl TD0File {
    pub fn from_bytes(bytes: Vec<u8>) -> TD0Result<Self> {
        let id_tag = header::TD0IdChunk::read_from_bytes(&bytes[..header::SZ_HDR_CHUNK])
            .map_err(|_| TD0Error::InvalidHeaderError)?;
        let _ = validate_id_tag(&id_tag)?;

        let manifest = parse_manifest(
            &bytes[SZ_HDR_CHUNK
                ..id_tag.get_bytes_remaining() as usize + OFFSET_BYTES_REMAINING as usize],
        )?;
        let _ = validate_manifest(&manifest)?;
        let tag_indexes = build_tag_indexes(&manifest)?;

        let mut chunk_headers: HashMap<String, ChunkHeader> = HashMap::new();
        let mut chunks: HashMap<String, Chunk> = HashMap::new();

        for tag in manifest.iter() {
            let as_str = str::from_utf8(&tag.tag).map_err(|_| TD0Error::InvalidTD0TagError)?;
            let tag_as_string = as_str.to_string();
            let chunk_header = ChunkHeader::read_from_bytes(
                &bytes[tag.get_pos() as usize..tag.get_pos() as usize + SZ_HDR_CHUNK],
            )
            .map_err(|_| TD0Error::InvalidChunkError {
                chunk_name: tag.get_tag(),
            })?;

            chunks.insert(
                tag_as_string.clone(),
                Chunk {
                    pos: tag.get_pos() as usize,
                    first_item_pos: tag.get_pos() as usize
                        + chunk_header.get_first_item_offset() as usize,
                    num_items: chunk_header.get_num_items() as usize,
                    sz_item: chunk_header.get_item_size() as usize,
                },
            );
            chunk_headers.insert(tag_as_string, chunk_header);
        }

        Ok(Self {
            buf: bytes,
            manifest: manifest,
            tag_indexes: tag_indexes,
            chunk_headers: chunk_headers,
            chunks: chunks,
        })
    }

    pub fn get_tag(&self, tag: &str) -> Option<&TD0ManifestTag> {
        if !self.tag_indexes.contains_key(tag) {
            return None;
        }

        if let Some(tag_index) = self.tag_indexes.get(tag)
            && let Some(tag_item) = self.manifest.get(*tag_index)
        {
            return Some(tag_item);
        }

        None
    }

    pub fn get_chunk_header(&self, chunk_tag: &TD0ManifestTag) -> TD0Result<Option<&ChunkHeader>> {
        let tag_name = chunk_tag.get_tag();
        if !self.tag_indexes.contains_key(&tag_name) {
            return Ok(None);
        }

        Ok(self.chunk_headers.get(&tag_name))
    }

    pub fn get_chunk_item(
        &self,
        chunk_tag: &TD0ManifestTag,
        item_index: usize,
    ) -> TD0Result<Option<Box<dyn ChunkItem>>> {
        let Some(chunk) = self.chunks.get(&chunk_tag.get_tag()) else {
            return Ok(None);
        };
        if item_index >= chunk.num_items {
            return Ok(None);
        }
        let Some(start) = chunk.item_pos(item_index) else {
            return Ok(None);
        };
        let end: usize = start + chunk.sz_item;

        return match chunk_tag.get_tag().as_str() {
            "HDRa" => Ok(Some(Box::new(HDRaItem::from_bytes(
                &self.buf[start..end],
                Some("HDRaItem"),
            )?))),
            "KITa" => Ok(Some(Box::new(KITaItem::from_bytes(
                &self.buf[start..end],
                Some("KITaItem"),
            )?))),
            "CURa" => Ok(Some(Box::new(CURaItem::from_bytes(
                &self.buf[start..end],
                Some("CURaItem"),
            )?))),
            _ => Ok(None),
        };
    }

    pub fn get_chunk_item_default(
        &self,
        chunk_name: &str,
    ) -> Option<Box<dyn ChunkItem>> {
        match chunk_name {
            "HDRa" => Some(Box::new(HDRaItem::default())),
            "KITa" => Some(Box::new(KITaItem::default())),
            "CURa" => Some(Box::new(CURaItem::default())),
            _ => None
        }
    }

    pub fn get_chunk_raw(&self, chunk_name: &str) -> Option<&[u8]> {
        let chunk_tag = self.get_tag(chunk_name)?;
        println!(
            "chunk pos: {:04X} length: {:04X} end: {:04X}",
            chunk_tag.get_pos() as usize,
            chunk_tag.get_length() as usize,
            chunk_tag.get_pos() as usize + chunk_tag.get_length() as usize,
        );
        Some(
            &self.buf[chunk_tag.get_pos() as usize
                ..chunk_tag.get_pos() as usize + chunk_tag.get_length() as usize],
        )
    }

    pub fn get_chunk_item_raw(&self, chunk_name: &str, item_index: usize) -> Option<&[u8]> {
        let chunk_tag = self.get_tag(chunk_name)?;
        return match self.chunk_headers.get(chunk_name) {
            Some(chunk_header) => {
                if item_index >= chunk_header.get_num_items() as usize {
                    return None;
                }

                let mut start = chunk_tag.get_pos() as usize;
                start += chunk_header.get_first_item_offset() as usize
                    + (chunk_header.get_item_size() as usize * item_index);
                let end = start + chunk_header.get_item_size() as usize;
                Some(&self.buf[start..end])
            }
            _ => None,
        };
    }
}

pub fn get_default_chunk_item(chunk_name: &str) -> Option<Box<dyn ChunkItem>> {
    match chunk_name {
        "HDRa" => Some(Box::new(HDRaItem::default())),
        "KITa" => Some(Box::new(KITaItem::default())),
        "CURa" => Some(Box::new(CURaItem::default())),
        _ => None,
    }
}

pub fn get_chunk_item_fields(chunk_name: &str) -> Option<&'static [&'static str]> {
    match get_default_chunk_item(chunk_name) {
        Some(default_item) => Some(default_item.get_fields()),
        None => None,
    }
}
