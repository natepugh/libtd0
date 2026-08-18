use crate::td0::header::{OFFSET_BYTES_REMAINING, TD0IdChunk};
use crate::td0::model::ssxp::rev::v1_10::chunk_item_from_bytes as v1_10_chunk_item_from_bytes;
use crate::td0::model::ssxp::rev::v1_10::get_default_chunk_item as v1_10_get_default_chunk_item;
use crate::td0::model::ssxp::rev::v2_0::chunk_item_from_bytes as v2_0_chunk_item_from_bytes;
use crate::td0::model::ssxp::rev::v2_0::get_default_chunk_item as v2_0_get_default_chunk_item;
use crate::td0::{SZ_HDR_CHUNK, TD0ManifestTag, validate_id_tag};
use core::fmt;
use libtd0_core::result::{TD0Error, TD0Result};
use libtd0_core::{
    ChunkManifest, SZ_MD5_DIGEST, TD0BackupType, TD0Chunk, TD0ChunkItem, TD0DeviceModel, TD0File,
    TD0Manifest, TD0Value, TD0ValueRaw,
};
use libtd0_derive::TD0ChunkItemDerive;
use md5::Digest;
use std::collections::{HashMap, HashSet};
use zerocopy::{FromBytes, LittleEndian, U32};
use zerocopy_derive::{IntoBytes, KnownLayout};

const BACKUP_TAG_KIT: &str = "SSXPROKT";
const BACKUP_TAG_SYSTEM: &str = "SSXPROBK";
const SZ_HDR_EXTRA_DATA: usize = 4;

#[derive(Copy, Clone, Debug, Default)]
pub struct Chunk {
    pub pos: usize,
    pub size: usize,
    pub first_item_pos: usize,
    pub num_items: usize,
    pub item_size: usize,
}

impl TD0Chunk for Chunk {
    fn pos(&self) -> usize {
        self.pos
    }

    fn size(&self) -> usize {
        self.size
    }

    fn item_pos(&self, item_index: usize) -> Option<usize> {
        if item_index >= self.num_items {
            None
        } else {
            Some(self.first_item_pos + (self.item_size * item_index))
        }
    }

    fn item_range(&self, item_index: usize) -> Option<::core::ops::Range<usize>> {
        self.item_pos(item_index)
            .map(|pos| pos..pos + self.item_size)
    }

    fn item_size(&self) -> usize {
        self.item_size
    }

    fn value(&self, _field: &str) -> Option<TD0Value> {
        None
    }

    fn value_raw(&self, _field: &str) -> Option<TD0ValueRaw> {
        None
    }

    fn list_fields(&self) -> Vec<String> {
        Vec::new()
    }

    fn num_items(&self) -> usize {
        self.num_items
    }

    fn set_value(&self, _field: &str, _value: &TD0Value) -> TD0Result<()> {
        Err(TD0Error::InvalidFieldOrType)
    }

    fn set_value_raw(&self, _field: &str, _raw_value: &TD0ValueRaw) -> TD0Result<()> {
        Err(TD0Error::InvalidFieldOrType)
    }
}

#[derive(FromBytes, IntoBytes, KnownLayout)]
#[repr(C, packed)]
pub struct ChunkHeader {
    pub num_items: U32<LittleEndian>,
    pub item_size: U32<LittleEndian>,
    pub first_item_offset: U32<LittleEndian>,
    unknown_header_data: [u8; SZ_HDR_EXTRA_DATA],
}

impl ChunkHeader {
    pub fn num_items(&self) -> usize {
        libtd0_core::usize_from_u32(self.num_items.get())
    }

    pub fn item_size(&self) -> usize {
        libtd0_core::usize_from_u32(self.item_size.get())
    }

    pub fn first_item_offset(&self) -> usize {
        libtd0_core::usize_from_u32(self.first_item_offset.get())
    }
}

impl fmt::Display for ChunkHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "num_items: {} : item_size: {}  first_item_offset: {}  unknown_header_data: {:?}",
            self.num_items, self.item_size, self.first_item_offset, self.unknown_header_data,
        )
    }
}

#[derive(Clone, Copy, Debug, FromBytes, IntoBytes, KnownLayout, TD0ChunkItemDerive)]
#[repr(C, packed)]
pub struct HDRaItem {
    #[td0_field(field_type = "Text", pad_byte = 0x2e)] // 0x2e = '.', ASCII period.
    tag: [u8; 8],

    #[td0_field(field_type = "Slice")]
    data: [u8; 8],

    #[td0_field(field_type = "Text", pad_byte = 0x2e)]
    name: [u8; 16],

    #[td0_field(field_type = "Text", pad_byte = 0x2e)]
    firmware: [u8; 4],

    #[td0_field(field_type = "Text", pad_byte = 0x2e)]
    build: [u8; 4],

    #[td0_field(field_type = "Text", pad_byte = 0x2e)]
    device_serial: [u8; 8],

    #[td0_field(field_type = "Slice")]
    suffix: [u8; 16],
}

fn parse_manifest(bytes: &[u8]) -> TD0Result<Vec<TD0ManifestTag>> {
    let mut manifest = Vec::<TD0ManifestTag>::new();
    let mut pos: usize = 0;
    while pos < bytes.len() {
        manifest.push(
            TD0ManifestTag::read_from_bytes(&bytes[pos..pos + SZ_HDR_CHUNK])
                .map_err(|_| TD0Error::InvalidTD0File("invalid manifest tag."))?,
        );
        pos += SZ_HDR_CHUNK;
    }
    Ok(manifest)
}

fn build_tag_indexes(tags: &[TD0ManifestTag]) -> TD0Result<HashMap<String, usize>> {
    Ok(tags
        .iter()
        .enumerate()
        .map(|(idx, tag)| (tag.tag(), idx))
        .collect())
}

fn validate_manifest(manifest: &[TD0ManifestTag]) -> TD0Result<()> {
    let mut unique: HashSet<String> = HashSet::<String>::new();
    for tag_name in manifest.iter().map(|tag| tag.tag()) {
        if unique.contains(&tag_name) {
            return Err(TD0Error::InvalidTD0File("duplicate manifest chunk"));
        }
        unique.insert(tag_name);
    }
    Ok(())
}

pub struct SSXPTD0File {
    buf: Vec<u8>,
    manifest: Vec<TD0ManifestTag>,
    tag_indexes: HashMap<String, usize>,
    chunk_headers: HashMap<String, ChunkHeader>,
    chunks: HashMap<String, Chunk>,
    firmware_version: String,
}

impl SSXPTD0File {
    #[allow(dead_code)]
    pub fn get_manifest_tag(&self, tag: &str) -> Option<&TD0ManifestTag> {
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

    fn validate_manifest_tags_all_same_model(&self) -> TD0Result<()> {
        if !self.manifest.is_empty() {
            let mut man_iter = self.manifest.iter();
            let model = man_iter.next().unwrap().model();
            for tag in man_iter {
                if tag.model() != model {
                    return Err(TD0Error::InvalidTD0File("mismatched manifest tags."));
                }
            }
        }
        Ok(())
    }

    fn get_backup_chunk_item(&self) -> TD0Result<HDRaItem> {
        let hdr_a_chunk = self
            .chunks
            .get("HDRa")
            .ok_or(TD0Error::InvalidTD0File("missing backup chunk."))?;
        let item_range = hdr_a_chunk
            .item_range(0)
            .ok_or(TD0Error::InvalidChunk("backup chunk invalid."))?;
        HDRaItem::read_from_bytes(&self.buf[item_range])
            .map_err(|_| TD0Error::InvalidChunk("backup chunk invalid."))
    }

    fn get_backup_type(&self) -> TD0BackupType {
        let Ok(backup_item) = self.get_backup_chunk_item() else {
            return TD0BackupType::Unknown;
        };

        match backup_item
            .get_value("tag")
            .expect("tag is a field")
            .to_string()
            .as_str()
        {
            BACKUP_TAG_SYSTEM => TD0BackupType::System,
            BACKUP_TAG_KIT => TD0BackupType::Kit,
            _ => TD0BackupType::Unknown,
        }
    }
}

impl TD0File for SSXPTD0File {
    fn as_bytes(&self) -> &[u8] {
        &self.buf
    }

    fn from_bytes(bytes: Vec<u8>) -> TD0Result<Self> {
        let id_tag = TD0IdChunk::read_from_bytes(&bytes[..SZ_HDR_CHUNK])
            .map_err(|_| TD0Error::InvalidTD0File("invalid or missing `TD0a` header chunk"))?;
        validate_id_tag(&id_tag)?;

        let manifest = parse_manifest(
            &bytes[SZ_HDR_CHUNK..id_tag.bytes_remaining() + OFFSET_BYTES_REMAINING],
        )?;
        validate_manifest(&manifest)?;
        let tag_indexes = build_tag_indexes(&manifest)?;

        let mut chunk_headers: HashMap<String, ChunkHeader> = HashMap::new();
        let mut chunks: HashMap<String, Chunk> = HashMap::new();

        for tag in manifest.iter() {
            let as_str =
                str::from_utf8(tag.tag_raw()).map_err(|_| TD0Error::InvalidChunk("unknown tag"))?;
            let tag_as_string = as_str.to_string();
            let chunk_header = ChunkHeader::read_from_bytes(
                &bytes[tag.chunk_pos()..tag.chunk_pos() + SZ_HDR_CHUNK],
            )
            .map_err(|_| TD0Error::InvalidChunk("invalid chunk"))?;

            chunks.insert(
                tag_as_string.clone(),
                Chunk {
                    pos: tag.chunk_pos(),
                    size: tag.chunk_size(),
                    first_item_pos: tag.chunk_pos() + chunk_header.first_item_offset(),
                    num_items: chunk_header.num_items(),
                    item_size: chunk_header.item_size(),
                },
            );
            chunk_headers.insert(tag_as_string, chunk_header);
        }

        let hdr_a_chunk = chunks
            .get("HDRa")
            .ok_or(TD0Error::InvalidTD0File("missing backup chunk."))?;
        let item_range = hdr_a_chunk
            .item_range(0)
            .ok_or(TD0Error::InvalidChunk("backup chunk invalid."))?;
        let hdr_item = HDRaItem::read_from_bytes(&bytes[item_range])
            .map_err(|_| TD0Error::InvalidChunk("backup chunk invalid."))?;

        Ok(Self {
            buf: bytes,
            manifest,
            tag_indexes,
            chunk_headers,
            chunks,
            firmware_version: String::from_utf8_lossy(&hdr_item.firmware[..4]).into(),
        })
    }

    fn get_chunk(&self, chunk_name: &str) -> Option<Box<dyn TD0Chunk>> {
        match self.chunks.get(chunk_name) {
            Some(ch) => Some(Box::new(*ch)),
            None => None,
        }
    }

    fn get_chunk_raw(&self, chunk_name: &str) -> Option<&[u8]> {
        match self.chunks.get(chunk_name) {
            Some(ch) => Some(&self.buf[ch.pos..(ch.pos + ch.size())]),
            None => None,
        }
    }

    fn get_chunk_item(
        &self,
        chunk_name: &str,
        item_index: usize,
    ) -> TD0Result<Box<dyn TD0ChunkItem>> {
        let Some(chunk) = self.chunks.get(chunk_name) else {
            return Err(TD0Error::UnknownChunk);
        };
        if item_index >= chunk.num_items {
            return Err(TD0Error::OutOfRange);
        }
        let Some(item_range) = chunk.item_range(item_index) else {
            return Err(TD0Error::InvalidChunk("invalid chunk format."));
        };

        match self.firmware_version.as_str() {
            "2.00" => Ok(v2_0_chunk_item_from_bytes(
                chunk_name,
                &self.buf[item_range],
            )?),
            "1.10" => Ok(v1_10_chunk_item_from_bytes(
                chunk_name,
                &self.buf[item_range],
            )?),
            _ => Err(TD0Error::FirmwareVersionNotImplemented),
        }
    }

    fn get_chunk_item_raw(&self, chunk_name: &str, item_index: usize) -> TD0Result<&[u8]> {
        let chunk = self.get_chunk(chunk_name).ok_or(TD0Error::UnknownChunk)?;
        Ok(&self.buf[chunk
            .item_range(item_index)
            .ok_or(TD0Error::InvalidChunkItem)?])
    }

    fn into_bytes(self) -> Vec<u8> {
        self.buf
    }

    fn manifest(&self) -> TD0Manifest {
        let sz_backup = self.buf.len() - SZ_MD5_DIGEST;
        let mut hasher: md5::Md5 = md5::Md5::new();
        hasher.update(&self.buf[0..sz_backup]);

        let mut checksum_actual: [u8; SZ_MD5_DIGEST] = [0; SZ_MD5_DIGEST];
        checksum_actual.clone_from_slice(&self.buf[sz_backup..]);

        let last_tag = self
            .manifest
            .iter()
            .last()
            .expect("at least one manifest tag exists.");
        let size_calculated = last_tag.chunk_pos() + last_tag.chunk_size() + SZ_MD5_DIGEST;

        TD0Manifest {
            backup_type: self.get_backup_type(),
            checksum_actual,
            checksum_calculated: hasher.finalize().into(),
            device_model: TD0DeviceModel::SPDSXPro,
            size_actual: self.buf.len(),
            size_calculated,
            chunks: self
                .manifest
                .iter()
                .map(|tag| {
                    let chunk_header = self
                        .chunk_headers
                        .get(&tag.tag())
                        .expect("each manifest tag has a chunk header.");
                    ChunkManifest {
                        name: tag.tag(),
                        pos: tag.chunk_pos(),
                        size: tag.chunk_size(),
                        num_items: chunk_header.num_items(),
                        item_size: chunk_header.item_size(),
                    }
                })
                .collect(),
        }
    }

    fn list_chunks(&self) -> Vec<String> {
        self.manifest.iter().map(|tag| tag.tag()).collect()
    }

    fn get_chunk_item_default(&self, chunk_name: &str) -> Option<Box<dyn TD0ChunkItem>> {
        match self.firmware_version.as_str() {
            "2.00" => v2_0_get_default_chunk_item(chunk_name),
            "1.10" => v1_10_get_default_chunk_item(chunk_name),
            _ => None,
        }
    }

    #[allow(unreachable_code)]
    fn validate(&self) -> TD0Result<()> {
        self.validate_manifest_tags_all_same_model()?;
        todo!("Validate: All manifest tags have the same model.");
        todo!("Validate: Total size equals the declared size.");
        todo!("Validate: File calculated checksum vs declared checksum.");
        Ok(())
    }
}
