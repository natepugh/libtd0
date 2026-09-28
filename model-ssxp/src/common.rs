// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::rev::v1_10::{
    chunk_item_from_bytes as v1_10_chunk_item_from_bytes,
    chunk_item_from_bytes_mut as v1_10_chunk_item_from_bytes_mut,
    chunk_item_from_bytes_owned as v1_10_chunk_item_from_bytes_owned,
    get_default_chunk_item as v1_10_get_default_chunk_item,
};

use crate::rev::v2_0::{
    chunk_item_from_bytes as v2_0_chunk_item_from_bytes,
    chunk_item_from_bytes_mut as v2_0_chunk_item_from_bytes_mut,
    chunk_item_from_bytes_owned as v2_0_chunk_item_from_bytes_owned,
    get_default_chunk_item as v2_0_get_default_chunk_item,
};

use ::core::fmt;
use ::core::ops::Range;
use libtd0_core::header::{OFFSET_BYTES_REMAINING, TD0_MAGIC, TD0IdChunk, TD0ManifestTag};
use libtd0_core::result::{InvalidChunkError, TD0Error, TD0Result};
use libtd0_core::{
    ChunkManifest, SZ_MD5_DIGEST, TD0BackupType, TD0ChunkItem, TD0DeviceModel, TD0File, TD0Manifest,
};
use libtd0_derive::TD0ChunkItemDerive;
use md5::Digest as _;
use std::collections::{HashMap, HashSet};
use zerocopy::{FromBytes, FromZeros, IntoBytes, LittleEndian, U32};
use zerocopy_derive::{Immutable, KnownLayout};

/// Backup header tag for a single-kit backup.
const BACKUP_TAG_KIT: &str = "SSXPROKT";
/// Backup header tag for whole system backup.
const BACKUP_TAG_SYSTEM: &str = "SSXPROBK";

fn device_model_from_tag_value(val: &str) -> TD0DeviceModel {
    match val {
        "SSXP" => TD0DeviceModel::SPDSXPro,
        _ => TD0DeviceModel::Unknown,
    }
}

/// The header of a `TD0Chunk` in a `TD0File`.
#[derive(Copy, Clone, Debug, FromBytes, Immutable, IntoBytes, KnownLayout)]
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
    #[td0_field(field_type = "Text", pad_byte = 0)]
    tag: [u8; 8],

    #[td0_field(field_type = "Slice")]
    data: [u8; 8],

    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    name: [u8; 16],

    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    firmware: [u8; 4],

    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    build: [u8; 4],

    #[td0_field(field_type = "Text", pad_byte = 0x20)] // 0x20 = ' ', ASCII space.
    device_serial: [u8; 8],

    #[td0_field(field_type = "Slice")]
    suffix: [u8; 16],
}

impl HDRaItem {
    const DEFAULT_BACKUP_NAME_KIT: [u8; 16] = *b"USER KIT        ";
    const DEFAULT_BUILD_V1_10: [u8; 4] = *b"0083";
    const DEFAULT_BUILD_V2_0: [u8; 4] = *b"0000";
    const DEFAULT_DEVICE_SERIAL: [u8; 8] = *b"XXXXXXX ";
    const DEFAULT_FIRMWARE_V1_10: [u8; 4] = *b"1.10";
    const DEFAULT_FIRMWARE_V2_0: [u8; 4] = *b"2.00";
    const DEFAULT_SUFFIX: [u8; 16] = [0xFFu8; 16];
    const DEFAULT_TAG: [u8; 8] = *b"SSXPROKT";

    pub fn new_default_v1_10() -> Self {
        let data = [1u8, 1u8, 0, 0, 0, 0, 0, 0];

        let mut tag = [0u8; 8];
        let mut name = [0u8; 16];
        let mut firmware = [0u8; 4];
        let mut build = [0u8; 4];
        let mut device_serial = [0u8; 8];
        let mut suffix = [0u8; 16];

        tag.copy_from_slice(&HDRaItem::DEFAULT_TAG);
        name.copy_from_slice(&HDRaItem::DEFAULT_BACKUP_NAME_KIT);
        firmware.copy_from_slice(&HDRaItem::DEFAULT_FIRMWARE_V1_10);
        build.copy_from_slice(&HDRaItem::DEFAULT_BUILD_V1_10);
        device_serial.copy_from_slice(&HDRaItem::DEFAULT_DEVICE_SERIAL);
        suffix.copy_from_slice(&HDRaItem::DEFAULT_SUFFIX);

        Self {
            tag,
            data,
            name,
            firmware,
            build,
            device_serial,
            suffix,
        }
    }

    pub fn new_default_v2_0() -> Self {
        let data = [0u8; 8];

        let mut tag = [0u8; 8];
        let mut name = [0u8; 16];
        let mut firmware = [0u8; 4];
        let mut build = [0u8; 4];
        let mut device_serial = [0u8; 8];
        let mut suffix = [0u8; 16];

        tag.copy_from_slice(&HDRaItem::DEFAULT_TAG);
        name.copy_from_slice(&HDRaItem::DEFAULT_BACKUP_NAME_KIT);
        firmware.copy_from_slice(&HDRaItem::DEFAULT_FIRMWARE_V2_0);
        build.copy_from_slice(&HDRaItem::DEFAULT_BUILD_V2_0);
        device_serial.copy_from_slice(&HDRaItem::DEFAULT_DEVICE_SERIAL);
        suffix.copy_from_slice(&HDRaItem::DEFAULT_SUFFIX);

        Self {
            tag,
            data,
            name,
            firmware,
            build,
            device_serial,
            suffix,
        }
    }

    pub fn default_header() -> ChunkHeader {
        ChunkHeader {
            num_items: U32::from(1u32),
            item_size: U32::from(u32::try_from(size_of::<HDRaItem>()).unwrap()),
            first_item_offset: U32::from(u32::try_from(size_of::<ChunkHeader>()).unwrap()),
            unknown_header_data: [0; 4],
        }
    }
}

pub struct SSXPTD0File {
    buf: Vec<u8>,
    chunk_order: Vec<String>,
    chunks: HashMap<String, Chunk>,
    dirty: bool,
    tag_offsets: HashMap<String, usize>,
}

fn get_chunk_header_bytes<'a>(buf: &'a [u8], manifest_tag: &TD0ManifestTag) -> TD0Result<&'a [u8]> {
    buf.get(manifest_tag.chunk_pos()..manifest_tag.chunk_pos() + size_of::<ChunkHeader>())
        .ok_or(TD0Error::InvalidChunk(InvalidChunkError::new(
            manifest_tag.tag(),
            "invalid chunk size or position.",
        )))
}

fn chunk_header_from_buf(buf: &[u8], manifest_tag: &TD0ManifestTag) -> TD0Result<ChunkHeader> {
    let bytes = get_chunk_header_bytes(buf, manifest_tag)?;
    Ok(ChunkHeader::read_from_bytes(bytes).expect("Did a checked retrieve of bytes prior."))
}

fn chunk_header_ref_from_buf<'a>(
    buf: &'a [u8],
    manifest_tag: &TD0ManifestTag,
) -> TD0Result<&'a ChunkHeader> {
    let bytes = get_chunk_header_bytes(buf, manifest_tag)?;
    ChunkHeader::ref_from_bytes(bytes).map_err(|_| {
        TD0Error::InvalidChunk(InvalidChunkError::new(
            manifest_tag.tag(),
            "unable to parse chunk header.",
        ))
    })
}

fn id_chunk_from_buf(buf: &[u8]) -> TD0Result<&TD0IdChunk> {
    TD0IdChunk::ref_from_bytes(
        buf.get(..size_of::<TD0IdChunk>())
            .ok_or(TD0Error::FileParse(
                "unable to read TD0 ID chunk.".to_string(),
            ))?,
    )
    .map_err(|_| TD0Error::FileParse("unable to parse TD0 ID chunk.".to_string()))
}

fn id_chunk_from_buf_mut(buf: &mut [u8]) -> TD0Result<&mut TD0IdChunk> {
    TD0IdChunk::mut_from_bytes(buf.get_mut(..size_of::<TD0IdChunk>()).ok_or(
        TD0Error::FileParse("unable to read TD0 ID chunk.".to_string()),
    )?)
    .map_err(|_| TD0Error::FileParse("unable to parse TD0 ID chunk.".to_string()))
}

fn manifest_tag_from_buf(buf: &[u8], pos: usize) -> TD0Result<&TD0ManifestTag> {
    TD0ManifestTag::ref_from_bytes(buf.get(pos..pos + size_of::<TD0ManifestTag>()).ok_or(
        TD0Error::FileParse("unable to read manifest tag.".to_string()),
    )?)
    .map_err(|_| TD0Error::FileParse("invalid manifest tag.".to_string()))
}

fn manifest_tag_from_buf_mut(buf: &mut [u8], pos: usize) -> TD0Result<&mut TD0ManifestTag> {
    TD0ManifestTag::mut_from_bytes(buf.get_mut(pos..pos + size_of::<TD0ManifestTag>()).ok_or(
        TD0Error::FileParse("unable to read manifest tag.".to_string()),
    )?)
    .map_err(|_| TD0Error::FileParse("invalid manifest tag.".to_string()))
}

fn collect_offsets(buf: &[u8]) -> TD0Result<(Vec<String>, HashMap<String, usize>)> {
    let mut chunk_order: Vec<String> = Vec::new();
    let mut tag_offsets: HashMap<String, usize> = HashMap::new();

    let header_size = id_chunk_from_buf(buf)?.bytes_remaining() + OFFSET_BYTES_REMAINING;
    let mut pos: usize = size_of::<TD0IdChunk>();
    while pos < header_size {
        let tag = manifest_tag_from_buf(buf, pos)?;
        let chunk_name = str::from_utf8(tag.tag_raw())
            .map_err(|_| TD0Error::FileParse("non-utf8 TD0 tag.".to_string()))?
            .to_string();
        chunk_order.push(chunk_name.clone());
        tag_offsets.insert(chunk_name, pos);

        pos += size_of::<TD0ManifestTag>();
    }

    Ok((chunk_order, tag_offsets))
}

/// Represents a section of a `TD0File` containing a specific subtype of data.
/// (e.g. Kit configuration or LED colors.)
#[derive(Clone, Copy, Debug)]
pub struct Chunk {
    pub header: ChunkHeader,
    pub pos: usize,
    pub size: usize,
}

impl Chunk {
    pub fn item_pos(&self, index: usize) -> Option<usize> {
        if index >= self.header.num_items() {
            None
        } else {
            Some(self.pos + self.header.first_item_offset() + (index * self.header.item_size()))
        }
    }

    pub fn item_range(&self, index: usize) -> Option<::core::ops::Range<usize>> {
        Some(self.item_pos(index)?..self.item_pos(index).unwrap() + self.header.item_size())
    }

    pub fn item_size(&self) -> usize {
        self.header.item_size()
    }

    pub fn num_items(&self) -> usize {
        self.header.num_items()
    }

    pub fn range(&self) -> Range<usize> {
        self.pos..self.pos + self.size
    }
}

impl SSXPTD0File {
    fn backup_header(&self) -> TD0Result<&HDRaItem> {
        let chunk = self.chunks.get("HDRa").ok_or(TD0Error::FileParse(
            "missing HDRa backup manifest.".to_string(),
        ))?;
        let bytes = chunk
            .item_range(0)
            .and_then(|irange| self.buf.get(irange))
            .ok_or(TD0Error::FileParse(
                "unable to read HDRa backup data.".to_string(),
            ))?;

        HDRaItem::ref_from_bytes(bytes)
            .map_err(|_| TD0Error::FileParse("unable to parse HDRa backup data.".to_string()))
    }

    pub fn calc_checksum(&self) -> [u8; 16] {
        let mut hasher: md5::Md5 = md5::Md5::new();
        hasher.update(
            self.buf
                .get(..self.buf.len() - SZ_MD5_DIGEST)
                .expect("buf is large enough for checksum."),
        );
        let mut checksum = [0u8; 16];
        checksum.copy_from_slice(hasher.finalize().as_bytes());
        checksum
    }

    fn calc_header_size(&self) -> usize {
        size_of::<TD0IdChunk>() + (self.chunk_order.len() * (size_of::<TD0ManifestTag>()))
    }

    pub fn calc_expected_size(&self) -> usize {
        self.calc_header_size()
            + self.chunks.values().map(|ch| ch.size).sum::<usize>()
            + SZ_MD5_DIGEST
    }

    pub fn firmware_version(&self) -> TD0Result<String> {
        let backup_header = self.backup_header()?;

        Ok(backup_header.field_value("firmware").unwrap().to_string())
    }

    pub fn manifest_tag(&self, chunk_name: &str) -> TD0Result<&TD0ManifestTag> {
        let &pos = self
            .tag_offsets
            .get(chunk_name)
            .ok_or(TD0Error::InvalidChunk(InvalidChunkError::new(
                chunk_name.to_string(),
                "unknown or missing chunk.",
            )))?;
        manifest_tag_from_buf(&self.buf, pos)
    }

    pub fn manifest_tag_mut(&mut self, chunk_name: &str) -> TD0Result<&mut TD0ManifestTag> {
        let &pos = self
            .tag_offsets
            .get(chunk_name)
            .ok_or(TD0Error::InvalidChunk(InvalidChunkError::new(
                chunk_name.to_string(),
                "unknown or missing chunk.",
            )))?;
        manifest_tag_from_buf_mut(&mut self.buf, pos)
    }

    pub fn read_checksum(&self) -> Option<&[u8]> {
        self.buf.get(self.buf.len() - SZ_MD5_DIGEST..)
    }

    fn validate_expected_file_size(&self) -> TD0Result<()> {
        if self.buf.len() == self.calc_expected_size() {
            Ok(())
        } else {
            Err(TD0Error::FileParse("incorrect file size.".to_string()))
        }
    }

    fn validate_has_backup_hdr_chunk(&self) -> TD0Result<()> {
        if self.tag_offsets.contains_key("HDRa") {
            Ok(())
        } else {
            Err(TD0Error::FileParse(
                "missing TD0a backup header.".to_string(),
            ))
        }
    }

    fn validate_manifest_tags_are_unique(&self) -> TD0Result<()> {
        let mut unique: HashSet<String> = HashSet::new();

        for offset in self.tag_offsets.values() {
            let tag = manifest_tag_from_buf(&self.buf, *offset)?;
            if !unique.insert(tag.tag()) {
                return Err(TD0Error::FileParse("duplicate manifest chunk".to_string()));
            }
        }
        Ok(())
    }

    fn validate_manifest_tags_correct_model(&self) -> TD0Result<()> {
        for offset in self.tag_offsets.values() {
            let tag = manifest_tag_from_buf(&self.buf, *offset)?;
            if tag.model() != "SSXP" {
                return Err(TD0Error::FileParse(
                    "manifest tag found with incorrect model.".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn validate_manifest_tags_have_chunk_headers(&self) -> TD0Result<()> {
        for offset in self.tag_offsets.values() {
            let tag = manifest_tag_from_buf(&self.buf, *offset)?;
            // Read the Chunk header directly at the location specified by the tag rather
            // than relying on this file's cached chunk objects.
            //
            // Parsing it is enough.
            let _ = chunk_header_ref_from_buf(&self.buf, tag)?;
        }

        Ok(())
    }

    fn validate_manifest_tags_in_order(&self) -> TD0Result<()> {
        let mut last_range = 0..size_of::<TD0IdChunk>();

        for offset in self.tag_offsets.values() {
            let tag = manifest_tag_from_buf(&self.buf, *offset)?;
            if libtd0_core::calc_range_overlap(tag.chunk_range(), last_range.clone()).is_some() {
                return Err(TD0Error::FileParse("overlapping chunks.".to_string()));
            }

            if tag.chunk_range().start <= last_range.start {
                return Err(TD0Error::FileParse(
                    "manifest tags out of order.".to_string(),
                ));
            }

            last_range = tag.chunk_range();
        }

        Ok(())
    }

    fn validate_chunk_headers_chunks_exist(&self) -> TD0Result<()> {
        for offset in self.tag_offsets.values() {
            let tag = manifest_tag_from_buf(&self.buf, *offset)?;
            let chunk_header = chunk_header_ref_from_buf(&self.buf, tag)?;
            // Calculate the pos of the first element of the next part of the file.
            // (either a chunk or the checksum.)
            let next_part_pos = tag.chunk_pos()
                + chunk_header.first_item_offset()
                + (chunk_header.item_size() * chunk_header.num_items());
            if next_part_pos > self.buf.len() {
                return Err(TD0Error::FileParse(
                    "not enough bytes for all chunks in the manifest.".to_string(),
                ));
            }
        }

        Ok(())
    }

    fn validate_checksum(&self) -> TD0Result<()> {
        if self.calc_checksum()
            != self
                .read_checksum()
                .expect("file has enough bytes for a checksum.")
        {
            Err(TD0Error::FileParse("checksum mismatch".to_string()))
        } else {
            Ok(())
        }
    }

    fn validate_is_not_dirty(&self) -> TD0Result<()> {
        if self.dirty {
            Err(TD0Error::FileParse(
                "file must be finalized before saving. Call TD0File::finalize() first.".to_string(),
            ))
        } else {
            Ok(())
        }
    }
}

impl TD0File for SSXPTD0File {
    fn add_chunk(&mut self, chunk_name: &str, num_items: usize) -> TD0Result<()> {
        let default_item = match self.firmware_version()?.as_str() {
            "1.10" => v1_10_get_default_chunk_item(chunk_name),
            "2.00" => v2_0_get_default_chunk_item(chunk_name),
            unexpected_version => {
                return Err(TD0Error::UnsupportedDeviceFirmwareVersion(
                    unexpected_version.to_string(),
                ));
            }
        }
        .ok_or(TD0Error::InvalidChunk(InvalidChunkError::new(
            chunk_name.to_string(),
            "unknown chunk.",
        )))?;

        let item_size = size_of_val(&*default_item);

        // To insert a new chunk (after the current last chunk):
        // - Update the headers of all self.chunks items, size_of::<TD0ManifestTag>() is added to all chunk positions.
        //  - First update the offsets in the bytes the manifest tags in self.buf.
        //  - Then update the self.chunks[item].header metadata to match.
        // - Insert the new TD0ManifestTag's bytes into self.buf at the correct position.
        // - Add its offset to self.tag_offsets.
        //
        // - Insert the new Chunk's bytes in self.buf at the correct position. (right before the checksum.)
        //  - Create a ChunkHeader and insert it.
        //  - Create a default TD0ChunkItem and clone it n times. Insert the bytes of each.
        // - Add a new Chunk object to self.chunks.
        // - Add its name to chunk_order.
        let mut manifest_tag = TD0ManifestTag::new_zeroed();
        manifest_tag.set_model("SSXP")?;
        manifest_tag.set_tag(chunk_name)?;
        manifest_tag.set_chunk_pos(
            u32::try_from(self.buf.len() - SZ_MD5_DIGEST + size_of::<TD0ManifestTag>()).map_err(
                |_| TD0Error::FileParse("chunk position exceeded 32-bit in bounds.".to_string()),
            )?,
        );
        manifest_tag.set_chunk_size(
            u32::try_from(size_of::<ChunkHeader>() + (item_size * num_items)).map_err(|_| {
                TD0Error::InvalidChunk(InvalidChunkError::new(
                    chunk_name.to_string(),
                    "chunk size is larger than a 32-bit int.",
                ))
            })?,
        );

        for offset in self.tag_offsets.values() {
            let tag = manifest_tag_from_buf_mut(&mut self.buf, *offset)?;
            tag.set_chunk_pos(
                u32::try_from(tag.chunk_pos() + size_of::<TD0ManifestTag>())
                    .map_err(|_| TD0Error::FileParse("file data size is too large.".to_string()))?,
            );
            self.chunks
                .get_mut(&tag.tag())
                .expect("all tags should have corresponding metadata.")
                .pos = tag.chunk_pos();
        }
        let new_tag_offset = self.calc_header_size();
        self.buf.splice(
            new_tag_offset..new_tag_offset,
            manifest_tag.as_bytes().iter().cloned(),
        );
        self.tag_offsets
            .insert(chunk_name.to_string(), new_tag_offset);

        let chunk_header: ChunkHeader =
            ChunkHeader {
                num_items: U32::from(u32::try_from(num_items).map_err(|_| {
                    TD0Error::InvalidInput("num_items larger than a u32.".to_string())
                })?),
                item_size: U32::from(u32::try_from(item_size).map_err(|_| {
                    TD0Error::InvalidInput("item_size larger than a u32.".to_string())
                })?),
                first_item_offset: U32::from(
                    u32::try_from(size_of::<ChunkHeader>())
                        .expect("chunk header is smaller than u32::MAX bytes."),
                ),
                unknown_header_data: [0u8; 4],
            };
        let chunk: Chunk = Chunk {
            header: chunk_header,
            pos: manifest_tag.chunk_pos(),
            size: manifest_tag.chunk_size(),
        };
        self.chunks.insert(chunk_name.to_string(), chunk);

        let buflen = self.buf.len();
        // let (buf, checksum) = self.buf.split_at_mut(buflen - SZ_MD5_DIGEST);
        let checksum = self.buf.split_off(buflen - SZ_MD5_DIGEST);
        self.buf.extend_from_slice(chunk_header.as_bytes());
        let default_item_bytes = default_item.as_bytes();
        for _ in 0..num_items {
            self.buf.extend_from_slice(default_item_bytes);
        }
        self.buf.extend_from_slice(&checksum);
        self.chunk_order.push(chunk_name.to_string());
        self.dirty = true;
        Ok(())
    }

    fn chunk_item_replace(
        &mut self,
        chunk_name: &str,
        dest_index: usize,
        source: &dyn TD0ChunkItem,
    ) -> TD0Result<()> {
        let chunk =
            self.chunks
                .get(chunk_name)
                .ok_or(TD0Error::InvalidChunk(InvalidChunkError::new(
                    chunk_name.to_string(),
                    "unknown chunk",
                )))?;
        let dest_range = chunk
            .item_range(dest_index)
            .ok_or(TD0Error::ChunkItemParse)?;

        let source_bytes = source.as_bytes();
        if source_bytes.len() != dest_range.len() {
            return Err(TD0Error::InvalidInput(
                "invalid item type for this chunk.".to_string(),
            ));
        }

        self.buf
            .get_mut(dest_range)
            .expect("chunk item is within source buffer.")
            .copy_from_slice(source_bytes);
        self.dirty = true;

        Ok(())
    }

    fn chunk_items_copy(
        &mut self,
        chunk_name: &str,
        source_index: usize,
        dest_index: usize,
    ) -> TD0Result<()> {
        let chunk =
            self.chunks
                .get(chunk_name)
                .ok_or(TD0Error::InvalidChunk(InvalidChunkError::new(
                    chunk_name.to_string(),
                    "unknown chunk",
                )))?;
        let source_range = chunk
            .item_range(source_index)
            .ok_or(TD0Error::ChunkItemParse)?;
        let dest_range = chunk
            .item_range(dest_index)
            .ok_or(TD0Error::ChunkItemParse)?;
        self.buf.copy_within(source_range, dest_range.start);
        self.dirty = true;
        Ok(())
    }

    fn chunk_items_swap(
        &mut self,
        chunk_name: &str,
        index_1: usize,
        index_2: usize,
    ) -> TD0Result<()> {
        let chunk =
            self.chunks
                .get(chunk_name)
                .ok_or(TD0Error::InvalidChunk(InvalidChunkError::new(
                    chunk_name.to_string(),
                    "unknown chunk",
                )))?;
        let item_1_range = chunk.item_range(index_1).ok_or(TD0Error::ChunkItemParse)?;
        let item_2_range = chunk.item_range(index_2).ok_or(TD0Error::ChunkItemParse)?;

        let item_1_bytes = self
            .buf
            .get(item_1_range.clone())
            .expect("all items are within buf.")
            .to_owned();
        self.buf
            .copy_within(item_2_range.clone(), item_1_range.start);
        self.buf
            .get_mut(item_2_range)
            .expect("all items are within buf.")
            .copy_from_slice(item_1_bytes.as_slice());
        self.dirty = true;
        Ok(())
    }

    fn chunk_items_reorder(&mut self, chunk_name: &str, new_order: &[usize]) -> TD0Result<()> {
        let chunk =
            self.chunks
                .get(chunk_name)
                .ok_or(TD0Error::InvalidChunk(InvalidChunkError::new(
                    chunk_name.to_string(),
                    "unknown chunk",
                )))?;

        // Check preconditions
        // new_order must have correct number of elements
        // new order must visit contain every index from 0..chunk items len. (in any order.)
        let expected: Vec<usize> = (0usize..chunk.num_items()).collect();
        let mut param_sorted = new_order.to_vec();
        param_sorted.sort_unstable();
        if !expected.eq(&param_sorted) {
            return Err(TD0Error::InvalidInput(format!(
                "new_order must contain all indexes from 0..{} with no repeated entries.",
                chunk.num_items()
            )));
        }

        let mut new_buf: Vec<u8> = vec![0; chunk.size - size_of::<ChunkHeader>()];

        let item_size = chunk.item_size();
        let mut pos: usize = 0;
        for &index in new_order {
            new_buf
                .get_mut(pos..pos + item_size)
                .expect("new_buf was allocated correctly.")
                .copy_from_slice(
                    self.buf
                        .get(chunk.item_range(index).expect("chunk index is valid"))
                        .expect("source file indexes are valid."),
                );
            pos += item_size;
        }

        self.buf
            .get_mut(chunk.pos + chunk.header.first_item_offset()..chunk.pos + chunk.size)
            .expect("item indexes are correct.")
            .copy_from_slice(&new_buf);

        self.dirty = true;
        Ok(())
    }

    fn finalize(&mut self) -> TD0Result<()> {
        let header_len = self.calc_header_size();
        let id_chunk = id_chunk_from_buf_mut(&mut self.buf)?;
        if id_chunk.bytes_remaining() != header_len - OFFSET_BYTES_REMAINING {
            id_chunk.set_bytes_remaining(
                u16::try_from(header_len - OFFSET_BYTES_REMAINING)
                    .expect("does not have 4095 manifest tags."),
            );
        }
        // TODO: Edit any reordered or moved chunk objects in in self.chunks to reflect the new positions.

        // Update the checksum.
        let buf_len = self.buf.len();
        let checksum = self.calc_checksum();
        self.buf
            .get_mut(buf_len - SZ_MD5_DIGEST..)
            .expect("buf is large enough to fit a checksum")
            .copy_from_slice(&checksum);
        self.dirty = false;
        Ok(())
    }

    fn try_from_bytes(bytes: &[u8]) -> TD0Result<Self> {
        let _ = id_chunk_from_buf(bytes)?;
        let (chunk_order, tag_offsets) = collect_offsets(bytes)?;
        let mut chunks: HashMap<String, Chunk> = HashMap::new();
        for (chunk_name, &offset) in tag_offsets.iter() {
            if chunks.contains_key(chunk_name) {
                return Err(TD0Error::FileParse("duplicate manifest chunk".to_string()));
            }

            let tag = manifest_tag_from_buf(bytes, offset)?;
            chunks.insert(
                chunk_name.clone(),
                Chunk {
                    header: chunk_header_from_buf(bytes, tag)?,
                    pos: tag.chunk_pos(),
                    size: tag.chunk_size(),
                },
            );
        }

        Ok(Self {
            buf: bytes.to_vec(),
            chunks,
            dirty: false,
            chunk_order,
            tag_offsets,
        })
    }

    fn try_into_bytes(self) -> TD0Result<Vec<u8>> {
        if self.dirty {
            Err(TD0Error::FileParse(
                "file must be finalized before saving. Call TD0File::finalize() first.".to_string(),
            ))
        } else {
            Ok(self.buf)
        }
    }

    fn chunk_num_items(&self, chunk_name: &str) -> Option<usize> {
        self.chunks.get(chunk_name).map(|ch| ch.num_items())
    }

    fn chunk_raw(&self, chunk_name: &str) -> Option<&[u8]> {
        let &tag_offset = self.tag_offsets.get(chunk_name)?;

        let tag = manifest_tag_from_buf(self.buf.as_bytes(), tag_offset)
            .expect("verified that tag exists.");
        self.buf.get(tag.chunk_range())
    }

    fn chunk_item(&self, chunk_name: &str, item_index: usize) -> TD0Result<&dyn TD0ChunkItem> {
        let chunk =
            self.chunks
                .get(chunk_name)
                .ok_or(TD0Error::InvalidChunk(InvalidChunkError::new(
                    chunk_name.to_string(),
                    "unknown chunk",
                )))?;
        let bytes = chunk
            .item_range(item_index)
            .and_then(|irange| self.buf.get(irange))
            .ok_or(TD0Error::ChunkItemParse)?;

        match self.firmware_version()?.as_str() {
            "1.10" => v1_10_chunk_item_from_bytes(chunk_name, bytes),
            "2.00" => v2_0_chunk_item_from_bytes(chunk_name, bytes),
            unexpected_version => Err(TD0Error::UnsupportedDeviceFirmwareVersion(
                unexpected_version.to_string(),
            )),
        }
    }

    fn chunk_item_mut(
        &mut self,
        chunk_name: &str,
        item_index: usize,
    ) -> TD0Result<&mut dyn TD0ChunkItem> {
        let chunk =
            self.chunks
                .get(chunk_name)
                .ok_or(TD0Error::InvalidChunk(InvalidChunkError::new(
                    chunk_name.to_string(),
                    "unknown chunk",
                )))?;
        let firmware_version = self.firmware_version()?.clone();
        let bytes = chunk
            .item_range(item_index)
            .and_then(|irange| self.buf.get_mut(irange))
            .ok_or(TD0Error::ChunkItemParse)?;

        match firmware_version.as_str() {
            "1.10" => v1_10_chunk_item_from_bytes_mut(chunk_name, bytes),
            "2.00" => v2_0_chunk_item_from_bytes_mut(chunk_name, bytes),
            unexpected_version => Err(TD0Error::UnsupportedDeviceFirmwareVersion(
                unexpected_version.to_string(),
            )),
        }
    }

    fn chunk_item_owned(
        &self,
        chunk_name: &str,
        item_index: usize,
    ) -> TD0Result<Box<dyn TD0ChunkItem>> {
        let chunk =
            self.chunks
                .get(chunk_name)
                .ok_or(TD0Error::InvalidChunk(InvalidChunkError::new(
                    chunk_name.to_string(),
                    "unknown chunk",
                )))?;
        let bytes = chunk
            .item_range(item_index)
            .and_then(|irange| self.buf.get(irange))
            .ok_or(TD0Error::ChunkItemParse)?;

        match self.firmware_version()?.as_str() {
            "1.10" => v1_10_chunk_item_from_bytes_owned(chunk_name, bytes),
            "2.00" => v2_0_chunk_item_from_bytes_owned(chunk_name, bytes),
            unexpected_version => Err(TD0Error::UnsupportedDeviceFirmwareVersion(
                unexpected_version.to_string(),
            )),
        }
    }

    fn chunk_item_default(&self, chunk_name: &str) -> Option<Box<dyn TD0ChunkItem>> {
        match self
            .firmware_version()
            .unwrap_or("unknown".to_string())
            .as_str()
        {
            "1.10" => v1_10_get_default_chunk_item(chunk_name),
            "2.00" => v2_0_get_default_chunk_item(chunk_name),
            _ => None,
        }
    }

    fn chunk_item_raw(&self, chunk_name: &str, item_index: usize) -> TD0Result<&[u8]> {
        let chunk =
            self.chunks
                .get(chunk_name)
                .ok_or(TD0Error::InvalidChunk(InvalidChunkError::new(
                    chunk_name.to_string(),
                    "unknown chunk",
                )))?;
        self.buf
            .get(
                chunk
                    .item_range(item_index)
                    .ok_or(TD0Error::ChunkItemParse)?,
            )
            .ok_or(TD0Error::InvalidChunk(InvalidChunkError::new(
                chunk_name.to_string(),
                "unable to read raw chunk data.",
            )))
    }

    fn chunk_pos(&self, chunk_name: &str) -> Option<usize> {
        self.chunks.get(chunk_name).map(|ch| ch.pos)
    }

    fn chunk_size(&self, chunk_name: &str) -> Option<usize> {
        self.chunks.get(chunk_name).map(|ch| ch.size)
    }

    fn manifest(&self) -> TD0Result<TD0Manifest> {
        let backup_chunk = self.backup_header()?;
        let backup_tag = self.manifest_tag("HDRa")?;
        let backup_type = match backup_chunk
            .field_value("tag")
            .unwrap()
            .to_string()
            .as_str()
        {
            BACKUP_TAG_KIT => TD0BackupType::Kit,
            BACKUP_TAG_SYSTEM => TD0BackupType::System,
            _ => TD0BackupType::Unknown,
        };

        let mut checksum_actual: [u8; 16] = [0; 16];
        checksum_actual.clone_from_slice(self.read_checksum().expect("enough bytes for a hash."));

        let chunks: Vec<ChunkManifest> = self
            .chunk_order
            .iter()
            .map(|ch_name| {
                (
                    ch_name,
                    self.chunks
                        .get(ch_name)
                        .expect("should be a chunk for each item in the chunk index."),
                )
            })
            .map(|(name, ch)| {
                ChunkManifest::new(
                    name.clone(),
                    ch.pos,
                    ch.size,
                    ch.num_items(),
                    ch.item_size(),
                )
            })
            .collect();

        Ok(TD0Manifest::new(
            backup_chunk
                .field_value("name")
                .map_or("Unknown".to_string(), |val| val.to_string()),
            backup_type,
            checksum_actual.as_slice(),
            &self.calc_checksum()[..],
            device_model_from_tag_value(backup_tag.model().as_str()),
            backup_chunk
                .field_value("build")
                .map_or("Unknown".to_string(), |val| val.to_string()),
            backup_chunk
                .field_value("firmware")
                .map_or("Unknown".to_string(), |val| val.to_string()),
            backup_chunk
                .field_value("device_serial")
                .map_or("Unknown".to_string(), |val| val.to_string()),
            self.buf.len(),
            self.calc_expected_size(),
            chunks,
        ))
    }

    fn new() -> TD0Result<Self> {
        const HEADER_SIZE: usize = size_of::<TD0IdChunk>() + size_of::<TD0ManifestTag>();
        const BODY_SIZE: usize = size_of::<ChunkHeader>() + size_of::<HDRaItem>();
        const BUF_SIZE: usize = HEADER_SIZE + BODY_SIZE + SZ_MD5_DIGEST;

        // Set values for TDOa ID tag.
        let mut buf = vec![0u8; BUF_SIZE];
        let id_tag = TD0IdChunk::mut_from_bytes(buf.get_mut(..size_of::<TD0IdChunk>()).unwrap())
            .map_err(|_| TD0Error::FileParse("can't create TD0a id chunk.".to_string()))?;
        id_tag.set_magic(&TD0_MAGIC);
        id_tag.set_bytes_remaining(
            u16::try_from(HEADER_SIZE - OFFSET_BYTES_REMAINING)
                .expect("much smaller than u16::MAX."),
        );
        let mut pos: usize = size_of::<TD0IdChunk>();

        // Set values for HDRa tag.
        let header_tag = TD0ManifestTag::mut_from_bytes(
            buf.get_mut(pos..pos + size_of::<TD0ManifestTag>())
                .expect("allocated more than this."),
        )
        .map_err(|_| TD0Error::FileParse("can't create HDRa manifest tag.".to_string()))?;
        pos += size_of::<TD0ManifestTag>();
        header_tag.set_tag("HDRa")?;
        header_tag.set_model("SSXP")?;
        header_tag.set_chunk_pos(u32::try_from(HEADER_SIZE).unwrap());
        header_tag.set_chunk_size(
            u32::try_from(size_of::<ChunkHeader>() + size_of::<HDRaItem>()).unwrap(),
        );

        // Set buf bytes from the default backup chunk header.
        let chunk_header = HDRaItem::default_header();
        buf.get_mut(pos..pos + size_of::<ChunkHeader>())
            .expect("allocated more than this.")
            .copy_from_slice(IntoBytes::as_bytes(&chunk_header));
        pos += size_of::<ChunkHeader>();

        // Set buf bytes from the default (v1.10) backup HDRaItem.
        let item = HDRaItem::new_default_v1_10();
        buf.get_mut(pos..pos + size_of::<HDRaItem>())
            .expect("allocated more than this.")
            .copy_from_slice(IntoBytes::as_bytes(&item));

        // Write checksum.
        let mut hasher: md5::Md5 = md5::Md5::new();
        hasher.update(
            buf.get(..BUF_SIZE - SZ_MD5_DIGEST)
                .expect("buf is large enough for checksum."),
        );
        buf.get_mut(BUF_SIZE - SZ_MD5_DIGEST..)
            .expect("buf is large enough for checksum.")
            .copy_from_slice(hasher.finalize().as_bytes());

        // Re-read chunk header to get an owned copy for this object.
        let chunk_header = ChunkHeader::read_from_bytes(
            buf.get_mut(HEADER_SIZE..HEADER_SIZE + size_of::<ChunkHeader>())
                .expect("allocated more than this."),
        )
        .map_err(|_| TD0Error::FileParse("can't read created HDRa chunk header.".to_string()))?;

        let backup_chunk: Chunk = Chunk {
            header: chunk_header,
            pos: HEADER_SIZE,
            size: size_of::<ChunkHeader>() + size_of::<HDRaItem>(),
        };

        Ok(Self {
            buf,
            chunk_order: vec!["HDRa".to_string()],
            chunks: [("HDRa".to_string(), backup_chunk)]
                .iter()
                .cloned()
                .collect(),
            dirty: false,
            tag_offsets: [("HDRa".to_string(), size_of::<TD0IdChunk>())]
                .iter()
                .cloned()
                .collect(),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.buf.clone()
    }

    fn list_chunks(&self) -> Vec<String> {
        self.chunk_order.clone()
    }

    fn validate_load(&self) -> TD0Result<()> {
        // - Has a HDRa backup chunk
        // - Manifest tags all have "SSXP" for the model.
        // - Every manifest tag has its existing chunk header within the file bytes.
        // - The file has at least the amount of bytes as declared in the each chunk header remaining in it.
        // - The checksum is correct.
        self.validate_expected_file_size()?;
        self.validate_manifest_tags_in_order()?;
        self.validate_manifest_tags_are_unique()?;
        self.validate_has_backup_hdr_chunk()?;
        self.validate_manifest_tags_correct_model()?;
        self.validate_manifest_tags_have_chunk_headers()?;
        self.validate_chunk_headers_chunks_exist()?;
        self.validate_checksum()?;
        Ok(())
    }

    fn validate_save(&self) -> TD0Result<()> {
        self.validate_is_not_dirty()?;
        self.validate_load()?;
        Ok(())
    }
}

impl core::fmt::Debug for SSXPTD0File {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "SSXPTD0File(buffer size: {}, num chunks: {})",
            self.buf.len(),
            self.chunk_order.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_item_pos() {
        static CHUNK_POS: usize = 212;
        static HDR_SIZE: usize = size_of::<ChunkHeader>();

        let ch_header: ChunkHeader = ChunkHeader {
            num_items: U32::from(5),
            item_size: U32::from(20),
            first_item_offset: U32::from(u32::try_from(HDR_SIZE).unwrap()),
            unknown_header_data: [0; 4],
        };
        let chunk: Chunk = Chunk {
            header: ch_header,
            pos: CHUNK_POS,
            size: 100 + HDR_SIZE,
        };
        assert_eq!(chunk.item_pos(0), Some(CHUNK_POS + HDR_SIZE));
        assert_eq!(chunk.item_pos(1), Some(CHUNK_POS + HDR_SIZE + 20));
        assert_eq!(chunk.item_pos(4), Some(CHUNK_POS + HDR_SIZE + 80));
        assert_eq!(chunk.item_pos(5), None);
    }

    #[test]
    fn test_chunk_item_range() {
        static CHUNK_POS: usize = 84;
        static HDR_SIZE: usize = size_of::<ChunkHeader>();
        static ITEM_0_POS: usize = 100;

        assert_eq!(
            ITEM_0_POS,
            CHUNK_POS + HDR_SIZE,
            "failed math class while setting up the test."
        );

        let ch_header: ChunkHeader = ChunkHeader {
            num_items: U32::from(5),
            item_size: U32::from(20),
            first_item_offset: U32::from(u32::try_from(HDR_SIZE).unwrap()),
            unknown_header_data: [0; 4],
        };
        let chunk: Chunk = Chunk {
            header: ch_header,
            pos: CHUNK_POS,
            size: 116,
        };

        assert_eq!(chunk.item_range(0), Some(ITEM_0_POS..ITEM_0_POS + 20));
        assert_eq!(chunk.item_range(1), Some(ITEM_0_POS + 20..ITEM_0_POS + 40));
        assert_eq!(chunk.item_range(4), Some(ITEM_0_POS + 80..ITEM_0_POS + 100));
        assert_eq!(chunk.item_range(5), None);
    }
}
