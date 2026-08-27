use crate::rev::v1_10::chunk_item_from_bytes as v1_10_chunk_item_from_bytes;
use crate::rev::v1_10::chunk_item_from_bytes_mut as v1_10_chunk_item_from_bytes_mut;
use crate::rev::v1_10::get_default_chunk_item as v1_10_get_default_chunk_item;
use crate::rev::v2_0::chunk_item_from_bytes as v2_0_chunk_item_from_bytes;
use crate::rev::v2_0::chunk_item_from_bytes_mut as v2_0_chunk_item_from_bytes_mut;
use crate::rev::v2_0::get_default_chunk_item as v2_0_get_default_chunk_item;
use libtd0_core::header::{SZ_HDR_CHUNK, TD0ManifestTag, validate_id_tag};
use ::core::ops::Range;
use core::fmt;
use libtd0_core::result::{TD0Error, TD0Result};
use libtd0_core::{
    ChunkManifest, SZ_MD5_DIGEST, TD0BackupType, TD0Chunk, TD0ChunkItem, TD0DeviceModel, TD0File,
    TD0Manifest, TD0Value, TD0ValueRaw,
};
use libtd0_core::header::{OFFSET_BYTES_REMAINING, TD0IdChunk};
use libtd0_derive::TD0ChunkItemDerive;
use md5::Digest as _;
use std::collections::{HashMap, HashSet};
use zerocopy::TryFromBytes as _;
use zerocopy::{FromBytes, LittleEndian, U32};
use zerocopy_derive::{Immutable, IntoBytes, KnownLayout};

/// Backup header tag for a single-kit backup.
const BACKUP_TAG_KIT: &str = "SSXPROKT";
/// Backup header tag for whole system backup.
const BACKUP_TAG_SYSTEM: &str = "SSXPROBK";

/// Represents a section of a `TD0File` containing a specific subtype of data.
/// (e.g. Kit configuration or LED colors.)
///
/// See the `TD0Chunk` docs for more information.
#[expect(clippy::arbitrary_source_item_ordering)] // ordering must match the binary format.
#[derive(Copy, Clone, Debug, Default)]
pub struct Chunk {
    /// Absolute chunk position in the file.
    pos: usize,
    /// The size of the chunk in bytes.
    size: usize,
    /// The absolute position of the first item in the chunk.
    first_item_pos: usize,
    /// The total number of items in the chunk.
    num_items: usize,
    /// The size in bytes of an individual chunk item.
    item_size: usize,
}

impl TD0Chunk for Chunk {
    fn item_pos(&self, item_index: usize) -> Option<usize> {
        if item_index >= self.num_items {
            None
        } else {
            Some(self.first_item_pos + (self.item_size * item_index))
        }
    }

    fn item_range(&self, item_index: usize) -> Option<Range<usize>> {
        self.item_pos(item_index)
            .map(|pos| pos..pos + self.item_size)
    }

    fn item_size(&self) -> usize {
        self.item_size
    }

    fn list_fields(&self) -> Vec<String> {
        Vec::new()
    }

    fn num_items(&self) -> usize {
        self.num_items
    }

    fn pos(&self) -> usize {
        self.pos
    }

    fn set_value(&self, _field: &str, _value: &TD0Value) -> TD0Result<()> {
        Err(TD0Error::InvalidFieldOrType)
    }

    fn set_value_raw(&self, _field: &str, _raw_value: &TD0ValueRaw) -> TD0Result<()> {
        Err(TD0Error::InvalidFieldOrType)
    }

    fn size(&self) -> usize {
        self.size
    }

    fn value(&self, _field: &str) -> Option<TD0Value> {
        None
    }

    fn value_raw(&self, _field: &str) -> Option<TD0ValueRaw> {
        None
    }
}

/// The header of a `TD0Chunk` in a `TD0File`.
#[derive(FromBytes, IntoBytes, KnownLayout)]
#[repr(C, packed)]
pub struct ChunkHeader {
    /// The number of items contained in the chunk.
    num_items: U32<LittleEndian>,

    /// The size of an individual chunk item.
    item_size: U32<LittleEndian>,

    /// The offset of the first item, relative to the first byte of the chunk header.
    first_item_offset: U32<LittleEndian>,

    /// Additional chunk header data of unknown purpose.
    unknown_header_data: [u8; 4],
}

impl ChunkHeader {
    pub fn first_item_offset(&self) -> usize {
        libtd0_core::usize_from_u32(self.first_item_offset.get())
    }

    pub fn item_size(&self) -> usize {
        libtd0_core::usize_from_u32(self.item_size.get())
    }

    pub fn num_items(&self) -> usize {
        libtd0_core::usize_from_u32(self.num_items.get())
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

#[derive(Clone, Copy, Debug, FromBytes, Immutable, IntoBytes, KnownLayout, TD0ChunkItemDerive)]
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
            TD0ManifestTag::read_from_bytes(
                bytes
                    .get(pos..pos + SZ_HDR_CHUNK)
                    .ok_or(TD0Error::InvalidTD0File("invalid manifest tag."))?,
            )
            .map_err(|_| TD0Error::InvalidTD0File("invalid manifest tag."))?,
        );
        pos += SZ_HDR_CHUNK;
    }
    Ok(manifest)
}

fn build_tag_indexes(tags: &[TD0ManifestTag]) -> HashMap<String, usize> {
    tags.iter()
        .enumerate()
        .map(|(idx, tag)| (tag.tag(), idx))
        .collect()
}

fn validate_manifest(manifest: &[TD0ManifestTag]) -> TD0Result<()> {
    let mut unique: HashSet<String> = HashSet::<String>::new();
    for tag_name in manifest.iter().map(TD0ManifestTag::tag) {
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

    fn get_backup_chunk_item(&self) -> TD0Result<&HDRaItem> {
        let hdr_a_chunk = self
            .chunks
            .get("HDRa")
            .ok_or(TD0Error::InvalidTD0File("missing backup chunk."))?;
        let item_range = hdr_a_chunk
            .item_range(0)
            .ok_or(TD0Error::InvalidChunk("backup chunk invalid."))?;
        HDRaItem::try_ref_from_bytes(
            self.buf
                .get(item_range)
                .ok_or(TD0Error::InvalidChunk("backup chunk invalid."))?,
        )
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

    fn chunk_items_copy(
        &mut self,
        chunk_name: &str,
        source_index: usize,
        dest_index: usize,
    ) -> TD0Result<()> {
        let Some(chunk) = self.get_chunk(chunk_name) else {
            return Err(TD0Error::UnknownChunk);
        };
        let Some(source_range) = chunk.item_range(source_index) else {
            return Err(TD0Error::InvalidChunkItem);
        };
        let Some(dest_range) = chunk.item_range(dest_index) else {
            return Err(TD0Error::InvalidChunkItem);
        };
        self.buf.copy_within(source_range, dest_range.start);
        Ok(())
    }

    fn chunk_items_swap(
        &mut self,
        chunk_name: &str,
        index_1: usize,
        index_2: usize,
    ) -> TD0Result<()> {
        let Some(chunk) = self.get_chunk(chunk_name) else {
            return Err(TD0Error::UnknownChunk);
        };
        let Some(item_1_range) = chunk.item_range(index_1) else {
            return Err(TD0Error::InvalidChunkItem);
        };
        let Some(item_2_range) = chunk.item_range(index_2) else {
            return Err(TD0Error::InvalidChunkItem);
        };

        // Copy item_1 to tmp
        let mut tmp: Vec<u8> = Vec::new();
        tmp.copy_from_slice(
            self.buf
                .get(item_1_range.clone())
                .expect("buffer is larger than this slice."),
        );
        // Copy item_2 to item_1
        self.buf
            .copy_within(item_2_range.clone(), item_1_range.start);
        // Copy tmp (formerly item_1) to item_2.
        self.buf
            .get_mut(item_2_range)
            .expect("buffer is larger than this slice")
            .copy_from_slice(&tmp);
        Ok(())
    }

    fn chunk_items_reorder(&mut self, chunk_name: &str, new_order: &[usize]) -> TD0Result<()> {
        let Some(chunk) = self.get_chunk(chunk_name) else {
            return Err(TD0Error::UnknownChunk);
        };

        // Ensure that 1) the length of new_order matches the number of chunk items
        //             2) All indexes from 0 to length - 1 are visited exactly once.
        let expected: Vec<usize> = (0..chunk.num_items()).collect();
        let mut new_order_test = new_order.to_vec();
        new_order_test.sort_unstable();
        if expected != new_order_test {
            return Err(TD0Error::InvalidInputWithMessage(format!(
                "The new order must contain each number from 0 - {} exactly once, in any order.",
                chunk.num_items() - 1
            )));
        }

        let first_item_pos = chunk
            .item_pos(0)
            .ok_or(TD0Error::InvalidChunk("Chunk must have at least one item."))?;

        // Copy items to a temp buffer in the correct order, then copy back to self.buf.
        //
        // TODO: This is inefficient, especially in cases where there are only a few changes.
        //       Put some thought into a more clever solution that minimizes the amount of
        //       data copied.
        let mut reordered_items: Vec<u8> = vec![0; chunk.item_size() * chunk.num_items()];
        let reordered_bytes: &mut [u8] = reordered_items.as_mut_slice();

        let mut dest_pos: usize = 0;
        let item_size = chunk.item_size();
        for source_index in new_order {
            reordered_bytes
                .get_mut(dest_pos..dest_pos + item_size)
                .expect("range is valid or math is wrong.")
                .copy_from_slice(
                    self.buf
                        .get(
                            chunk
                                .item_range(*source_index)
                                .expect("Pre-verified indexes are all in range."),
                        )
                        .expect("chunk range must be within whole file buffer range."),
                );
            dest_pos += item_size;
        }

        self.buf
            .get_mut(first_item_pos..chunk.pos() + chunk.size())
            .expect("calculated size is smaller than file buffer size.")
            .copy_from_slice(reordered_bytes);
        Ok(())
    }

    fn from_bytes(bytes: Vec<u8>) -> TD0Result<Self> {
        let id_tag = TD0IdChunk::read_from_bytes(bytes.get(..SZ_HDR_CHUNK).ok_or(
            TD0Error::InvalidTD0File("invalid or missing `TD0a` header chunk"),
        )?)
        .map_err(|_| TD0Error::InvalidTD0File("invalid or missing `TD0a` header chunk"))?;
        validate_id_tag(&id_tag)?;

        let manifest = parse_manifest(
            bytes
                .get(SZ_HDR_CHUNK..id_tag.bytes_remaining() + OFFSET_BYTES_REMAINING)
                .ok_or(TD0Error::InvalidTD0File(
                    "incomplete `TD0a` manifest section.",
                ))?,
        )?;
        validate_manifest(&manifest)?;
        let tag_indexes = build_tag_indexes(&manifest);

        let mut chunk_headers: HashMap<String, ChunkHeader> = HashMap::new();
        let mut chunks: HashMap<String, Chunk> = HashMap::new();

        for tag in &manifest {
            let as_str =
                str::from_utf8(tag.tag_raw()).map_err(|_| TD0Error::InvalidChunk("unknown tag"))?;
            let tag_as_string = as_str.to_string();
            let chunk_header = ChunkHeader::read_from_bytes(
                bytes
                    .get(tag.chunk_pos()..tag.chunk_pos() + SZ_HDR_CHUNK)
                    .ok_or(TD0Error::InvalidTD0File("missing chunk header"))?,
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
        let hdr_item = HDRaItem::read_from_bytes(
            bytes
                .get(item_range)
                .ok_or(TD0Error::InvalidChunk("backup chunk invalid."))?,
        )
        .map_err(|_| TD0Error::InvalidChunk("backup chunk invalid."))?;

        Ok(Self {
            buf: bytes,
            manifest: manifest.into(),
            tag_indexes,
            chunk_headers,
            chunks,
            firmware_version: String::from_utf8_lossy(&hdr_item.firmware[..4]).into(),
        })
    }

    fn get_chunk(&self, chunk_name: &str) -> Option<Box<dyn TD0Chunk>> {
        self.chunks
            .get(chunk_name)
            .map(|ch| Box::new(*ch) as Box<dyn TD0Chunk>)
    }

    fn get_chunk_raw(&self, chunk_name: &str) -> Option<&[u8]> {
        let chunk = self.chunks.get(chunk_name)?;
        self.buf.get(chunk.pos..(chunk.pos + chunk.size()))
    }

    fn get_chunk_item_mut(
        &mut self,
        chunk_name: &str,
        item_index: usize,
    ) -> TD0Result<&mut dyn TD0ChunkItem> {
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
            "2.00" => Ok(v2_0_chunk_item_from_bytes_mut(
                chunk_name,
                self.buf
                    .get_mut(item_range)
                    .ok_or(TD0Error::InvalidTD0File("chunk item out of file bounds"))?,
            )?),
            "1.10" => Ok(v1_10_chunk_item_from_bytes_mut(
                chunk_name,
                self.buf
                    .get_mut(item_range)
                    .ok_or(TD0Error::InvalidTD0File("chunk item out of file bounds"))?,
            )?),
            _ => Err(TD0Error::UnsupportedFirmwareVersion),
        }
    }
    fn get_chunk_item(&self, chunk_name: &str, item_index: usize) -> TD0Result<&dyn TD0ChunkItem> {
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
                self.buf
                    .get(item_range)
                    .ok_or(TD0Error::InvalidTD0File("chunk item out of file bounds"))?,
            )?),
            "1.10" => Ok(v1_10_chunk_item_from_bytes(
                chunk_name,
                self.buf
                    .get(item_range)
                    .ok_or(TD0Error::InvalidTD0File("chunk item out of file bounds"))?,
            )?),
            _ => Err(TD0Error::UnsupportedFirmwareVersion),
        }
    }

    fn get_chunk_item_raw(&self, chunk_name: &str, item_index: usize) -> TD0Result<&[u8]> {
        let chunk = self.get_chunk(chunk_name).ok_or(TD0Error::UnknownChunk)?;
        self.buf
            .get(
                chunk
                    .item_range(item_index)
                    .ok_or(TD0Error::InvalidChunkItem)?,
            )
            .ok_or(TD0Error::InvalidTD0File("chunk item out of file bounds"))
    }

    fn into_bytes(self) -> Vec<u8> {
        self.buf
    }

    fn manifest(&self) -> TD0Manifest {
        let sz_backup = self.buf.len() - SZ_MD5_DIGEST;
        let mut hasher: md5::Md5 = md5::Md5::new();
        hasher.update(self.buf.get(0..sz_backup).unwrap());

        let mut checksum_actual: [u8; SZ_MD5_DIGEST] = [0; SZ_MD5_DIGEST];
        checksum_actual.clone_from_slice(self.buf.get(sz_backup..).unwrap());

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
        self.manifest.iter().map(TD0ManifestTag::tag).collect()
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
