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
use td0_core::header::{OFFSET_BYTES_REMAINING, TD0_MAGIC, TD0IdChunk, TD0ManifestTag};
use td0_core::result::{InvalidChunkError, TD0Error, TD0Result, create_u32_oob_error};
use td0_core::{
    ChunkManifest, SZ_MD5_DIGEST, TD0BackupType, TD0ChunkItem, TD0DeviceModel, TD0File,
    TD0Manifest, try_u32_from_usize, usize_from_u32,
};
use td0_derive::TD0ChunkItemDerive;
use md5::Digest as _;
use std::collections::{HashMap, HashSet};
use zerocopy::{FromBytes, FromZeros, IntoBytes, LittleEndian, TryFromBytes, U32};
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
#[derive(Copy, Clone, Debug, Default, FromBytes, PartialEq, Immutable, IntoBytes, KnownLayout)]
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
        usize_from_u32(self.first_item_offset.get())
    }

    pub fn item_size(&self) -> usize {
        usize_from_u32(self.item_size.get())
    }

    pub fn num_items(&self) -> usize {
        usize_from_u32(self.num_items.get())
    }

    pub fn new(
        num_items: u32,
        item_size: u32,
        first_item_offset: u32,
        unknown_header_data: Option<&[u8; 4]>,
    ) -> Self {
        let mut unknown_buf: [u8; 4] = [0; 4];
        if let Some(buf) = unknown_header_data {
            unknown_buf.copy_from_slice(buf);
        };

        Self {
            num_items: U32::from(num_items),
            item_size: U32::from(item_size),
            first_item_offset: U32::from(first_item_offset),
            unknown_header_data: unknown_buf,
        }
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

    // NOTE: SSXPROBK (whole system) backups use 0x0 as a pad byte.
    //       SSXPROKT (single kit) backups use 0x20, likely copied
    //       verbatim from the kit name.
    #[td0_field(field_type = "Text", pad_byte = 0x0)]
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

#[derive(Clone, Debug, Default)]
struct SSXPTD0FileMetadata {
    chunks: HashMap<String, Chunk>,
    tag_order: Vec<String>,
    tags: HashMap<String, TD0ManifestTag>,
}

fn manifest_tag_from_chunk(
    chunk_name: &str,
    chunk: &Chunk,
    device_model: TD0DeviceModel,
) -> TD0Result<TD0ManifestTag> {
    let mut tag: TD0ManifestTag = TD0ManifestTag::new_zeroed();
    tag.set_chunk_pos(usize_from_u32(chunk.pos))?;
    tag.set_tag(chunk_name)?;
    tag.set_model(device_model.as_str())?;
    tag.set_chunk_size(usize_from_u32(chunk.size))?;
    Ok(tag)
}

impl SSXPTD0FileMetadata {
    pub fn new_from_buf(buf: &[u8]) -> TD0Result<Self> {
        let id_chunk = id_chunk_from_buf(buf)?;
        let mut pos = size_of::<TD0IdChunk>();
        let mut tags: HashMap<String, TD0ManifestTag> = HashMap::new();
        let mut tag_order: Vec<String> = Vec::new();
        let mut chunks: HashMap<String, Chunk> = HashMap::new();

        while pos < id_chunk.bytes_remaining() + OFFSET_BYTES_REMAINING && pos < buf.len() {
            let tag = manifest_tag_from_buf_owned(buf, pos)?;
            tags.insert(tag.tag(), tag);
            tag_order.push(tag.tag());
            pos += size_of::<TD0ManifestTag>();
        }

        for tag in tags.values() {
            chunks.insert(
                tag.tag(),
                Chunk {
                    header: chunk_header_from_buf(buf, tag)?,
                    pos: u32::try_from(tag.chunk_pos()).map_err(|_| {
                        TD0Error::FileParse(format!("chunk '{}' has an invalid offset.", tag.tag()))
                    })?,
                    size: u32::try_from(tag.chunk_size()).map_err(|_| {
                        TD0Error::FileParse(format!("chunk '{}' has an invalid size.", tag.tag()))
                    })?,
                },
            );
        }

        Ok(Self {
            chunks,
            tag_order,
            tags,
        })
    }

    pub fn chunk(&self, chunk_name: impl Into<String>) -> Option<&Chunk> {
        self.chunks.get(&chunk_name.into())
    }

    pub fn manifest_tag(&self, manifest_tag_name: impl Into<String>) -> Option<&TD0ManifestTag> {
        self.tags.get(&manifest_tag_name.into())
    }

    pub fn manifest_tag_pos(&self, tag_name: impl Into<String>) -> Option<usize> {
        let tag_name = tag_name.into();
        self.tag_order
            .iter()
            .position(|val| val.as_str() == tag_name)
            .map(|index| index * size_of::<TD0ManifestTag>() + size_of::<TD0IdChunk>())
    }

    pub fn header_size(&self) -> usize {
        size_of::<TD0IdChunk>() + (size_of::<TD0ManifestTag>() * self.tags.len())
    }

    pub fn chunk_data_size(&self) -> usize {
        usize_from_u32(self.chunks.values().map(|ch| ch.size).sum::<u32>())
    }

    fn next_chunk_pos(&self) -> usize {
        // NOTE: This method assumes that the new header for the chunk hasn't
        //       been added to the file.

        // Inefficient, but shouldn't be called often. If this becomes a
        // bottleneck, consider saving the chunk order as a struct member.
        //
        // An ASSUMED invariant is that the tag order in the manifest matches
        // the chunk order in the file data. Caching the order rather than
        // relying on this assumption is probably the safest option.
        let mut last_chunk: Option<&Chunk> = None;
        for chunk in self.chunks.values() {
            if last_chunk.is_none() || last_chunk.unwrap().pos < chunk.pos {
                last_chunk = Some(chunk);
            }
        }
        let before_header_added = match last_chunk {
            Some(chunk) => usize_from_u32(chunk.pos + chunk.size),
            None => self.header_size(),
        };
        before_header_added + size_of::<TD0ManifestTag>()
    }

    fn create_new_chunk(&self, num_items: u32, item_size: u32) -> TD0Result<Chunk> {
        let first_item_offset = u32::try_from(size_of::<ChunkHeader>()).unwrap();
        let header = ChunkHeader::new(num_items, item_size, first_item_offset, None);

        let chunk_pos: u32 =
            u32::try_from(self.next_chunk_pos()).map_err(|_| TD0Error::FileTooLarge)?;
        let chunk_data_size: u32 =
            num_items
                .checked_mul(item_size)
                .ok_or(TD0Error::InvalidInput(
                    "num_items * item_size may not be larger than u32::MAX.".to_string(),
                ))?;
        let chunk_size: u32 =
            first_item_offset
                .checked_add(chunk_data_size)
                .ok_or(TD0Error::InvalidInput(
                    "num_items * item_size + ChunkHeader size may not be larger than u32::MAX."
                        .to_string(),
                ))?;
        Ok(Chunk {
            header,
            pos: chunk_pos,
            size: chunk_size,
        })
    }

    pub fn push_chunk(
        &mut self,
        chunk_name: impl Into<String>,
        device_model: TD0DeviceModel,
        num_items: u32,
        item_size: u32,
    ) -> TD0Result<()> {
        let chunk_name: String = chunk_name.into();
        if self.chunks.contains_key(&chunk_name) {
            return Err(TD0Error::InvalidInput(format!(
                "chunk '{chunk_name}' already exists."
            )));
        }

        let chunk = self.create_new_chunk(num_items, item_size)?;
        let tag: TD0ManifestTag = manifest_tag_from_chunk(&chunk_name, &chunk, device_model)?;
        // Since we're pushing, all chunks' positions are incremented by the size of a TD0ManifestTag.
        let all_chunks: Vec<String> = self.chunks.keys().cloned().collect();
        for chunk_name in all_chunks.iter() {
            self.update_chunk_offset(
                chunk_name,
                i64::try_from(size_of::<TD0ManifestTag>()).unwrap(),
            )?;
        }

        self.chunks.insert(chunk_name.clone(), chunk);
        self.tags.insert(chunk_name.clone(), tag);
        self.tag_order.push(chunk_name);
        Ok(())
    }

    fn update_chunk_offset(&mut self, chunk_name: &str, offset: i64) -> TD0Result<()> {
        // Update the position in the manifest.
        let tag = self
            .tags
            .get_mut(chunk_name)
            .ok_or(TD0Error::unknown_chunk_error(chunk_name))?;
        let chunk_pos: i64 = i64::try_from(tag.chunk_pos()).map_err(|_| {
            TD0Error::InvalidChunk(InvalidChunkError::new(tag.tag(), "chunk offset invalid."))
        })?;
        tag.set_chunk_pos(usize_from_u32(
            u32::try_from(chunk_pos + offset).map_err(|_| create_u32_oob_error())?,
        ))?;

        // Update the position in the chunk metadata.
        self.chunks.get_mut(&tag.tag()).unwrap().pos = try_u32_from_usize(tag.chunk_pos())?;
        Ok(())
    }

    pub fn remove_chunk(&mut self, chunk_name: &str) -> TD0Result<()> {
        let remove_tag_index: usize = self
            .tag_order
            .iter()
            .position(|val| val.as_str() == chunk_name)
            .ok_or(TD0Error::unknown_chunk_error(chunk_name))?;
        let (remove_tag_pos, remove_tag_size) = self
            .chunk(chunk_name)
            .map(|ch| (usize_from_u32(ch.pos), usize_from_u32(ch.size)))
            .ok_or(TD0Error::unknown_chunk_error(chunk_name))?;

        // The chunks before remove_tag_pos positions are decremented by the size of a TD0ManifestTag.
        // The chunks afterware are decremented by the size of the chunk in addition to the manfiest tag.
        let before_offset: i64 = -i64::try_from(size_of::<TD0ManifestTag>()).unwrap();
        let after_offset: i64 = -before_offset
            - i64::try_from(remove_tag_size).map_err(|_| {
                TD0Error::InvalidChunk(InvalidChunkError::new(chunk_name, "chunk size invalid."))
            })?;

        let mut chunk_offset_changes: HashMap<String, i64> = HashMap::new();

        for (name, tag) in self.tags.iter() {
            if tag.chunk_pos() < remove_tag_pos {
                chunk_offset_changes.insert(name.clone(), before_offset);
            } else if tag.chunk_pos() > remove_tag_pos {
                chunk_offset_changes.insert(name.clone(), after_offset);
            }
            // Intentionally excluding chunk_pos == remove_tag_pos.
        }

        for (modify_chunk_name, offset) in chunk_offset_changes.iter() {
            self.update_chunk_offset(modify_chunk_name, *offset)?;
        }

        self.chunks.remove(chunk_name).unwrap();
        self.tags.remove(chunk_name).unwrap();
        self.tag_order.remove(remove_tag_index);

        Ok(())
    }
}

pub struct SSXPTD0File {
    buf: Vec<u8>,
    dirty: bool,
    meta: SSXPTD0FileMetadata,
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
    ChunkHeader::try_read_from_bytes(bytes).map_err(|_| {
        TD0Error::InvalidChunk(InvalidChunkError::new(
            manifest_tag.tag(),
            "unable to parse chunk header.",
        ))
    })
}

fn chunk_header_ref_from_buf<'a>(
    buf: &'a [u8],
    manifest_tag: &TD0ManifestTag,
) -> TD0Result<&'a ChunkHeader> {
    let bytes = get_chunk_header_bytes(buf, manifest_tag)?;
    ChunkHeader::try_ref_from_bytes(bytes).map_err(|_| {
        TD0Error::InvalidChunk(InvalidChunkError::new(
            manifest_tag.tag(),
            "unable to parse chunk header.",
        ))
    })
}

fn id_chunk_from_buf(buf: &[u8]) -> TD0Result<&TD0IdChunk> {
    TD0IdChunk::try_ref_from_bytes(buf.get(..size_of::<TD0IdChunk>()).ok_or(
        TD0Error::FileParse("unable to read TD0 ID chunk.".to_string()),
    )?)
    .map_err(|_| TD0Error::FileParse("unable to parse TD0 ID chunk.".to_string()))
}

fn id_chunk_from_buf_mut(buf: &mut [u8]) -> TD0Result<&mut TD0IdChunk> {
    TD0IdChunk::try_mut_from_bytes(buf.get_mut(..size_of::<TD0IdChunk>()).ok_or(
        TD0Error::FileParse("unable to read TD0 ID chunk.".to_string()),
    )?)
    .map_err(|_| TD0Error::FileParse("unable to parse TD0 ID chunk.".to_string()))
}

fn manifest_tag_from_buf(buf: &[u8], pos: usize) -> TD0Result<&TD0ManifestTag> {
    TD0ManifestTag::try_ref_from_bytes(buf.get(pos..pos + size_of::<TD0ManifestTag>()).ok_or(
        TD0Error::FileParse("unable to read manifest tag.".to_string()),
    )?)
    .map_err(|_| TD0Error::FileParse("invalid manifest tag.".to_string()))
}

fn manifest_tag_from_buf_owned(buf: &[u8], pos: usize) -> TD0Result<TD0ManifestTag> {
    TD0ManifestTag::try_read_from_bytes(buf.get(pos..pos + size_of::<TD0ManifestTag>()).ok_or(
        TD0Error::FileParse("unable to read manifest tag.".to_string()),
    )?)
    .map_err(|_| TD0Error::FileParse("invalid manifest tag.".to_string()))
}

fn manifest_tag_from_buf_mut(buf: &mut [u8], pos: usize) -> TD0Result<&mut TD0ManifestTag> {
    TD0ManifestTag::try_mut_from_bytes(buf.get_mut(pos..pos + size_of::<TD0ManifestTag>()).ok_or(
        TD0Error::FileParse("unable to read manifest tag.".to_string()),
    )?)
    .map_err(|_| TD0Error::FileParse("invalid manifest tag.".to_string()))
}

/// Represents a section of a `TD0File` containing a specific subtype of data.
/// (e.g. Kit configuration or LED colors.)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Chunk {
    pub header: ChunkHeader,
    pub pos: u32,
    pub size: u32,
}

impl Chunk {
    pub fn item_pos(&self, index: usize) -> Option<usize> {
        if index >= self.header.num_items() {
            None
        } else {
            Some(
                usize_from_u32(self.pos)
                    + self.header.first_item_offset()
                    + (index * self.header.item_size()),
            )
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
        usize_from_u32(self.pos)..usize_from_u32(self.pos + self.size)
    }
}

impl SSXPTD0File {
    fn backup_header(&self) -> TD0Result<&HDRaItem> {
        let chunk = self.meta.chunk("HDRa").ok_or(TD0Error::FileParse(
            "missing HDRa backup manifest.".to_string(),
        ))?;

        let bytes = chunk
            .item_range(0)
            .and_then(|irange| self.buf.get(irange))
            .ok_or(TD0Error::FileParse(
                "unable to read HDRa backup data.".to_string(),
            ))?;

        HDRaItem::try_ref_from_bytes(bytes)
            .map_err(|_| TD0Error::FileParse("unable to parse HDRa backup data.".to_string()))
    }

    fn backup_header_mut(&mut self) -> TD0Result<&mut HDRaItem> {
        let chunk = self.meta.chunk("HDRa").ok_or(TD0Error::FileParse(
            "missing HDRa backup manifest.".to_string(),
        ))?;

        let bytes = chunk
            .item_range(0)
            .and_then(|irange| self.buf.get_mut(irange))
            .ok_or(TD0Error::FileParse(
                "unable to read HDRa backup data.".to_string(),
            ))?;

        HDRaItem::try_mut_from_bytes(bytes)
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
        self.meta.header_size()
    }

    pub fn calc_expected_size(&self) -> usize {
        self.calc_header_size() + self.meta.chunk_data_size() + SZ_MD5_DIGEST
    }

    pub fn firmware_version(&self) -> TD0Result<String> {
        let backup_header = self.backup_header()?;

        Ok(backup_header.field_value("firmware").unwrap().to_string())
    }

    pub fn manifest_tag(&self, chunk_name: &str) -> TD0Result<&TD0ManifestTag> {
        let pos = self
            .meta
            .chunk(chunk_name)
            .ok_or(TD0Error::unknown_chunk_error(chunk_name))?
            .pos;
        manifest_tag_from_buf(&self.buf, usize_from_u32(pos))
    }

    pub fn manifest_tag_mut(&mut self, chunk_name: &str) -> TD0Result<&mut TD0ManifestTag> {
        let pos = self
            .meta
            .chunk(chunk_name)
            .ok_or(TD0Error::unknown_chunk_error(chunk_name))?
            .pos;
        manifest_tag_from_buf_mut(&mut self.buf, usize_from_u32(pos))
    }

    pub fn new_with(firmware_version: &str) -> TD0Result<Self> {
        let mut newobj = Self::new();
        let backup_chunk = newobj.backup_header_mut().unwrap();
        backup_chunk.firmware.copy_from_slice(firmware_version.as_bytes());
        Ok(newobj)
    }

    pub fn read_checksum(&self) -> Option<&[u8]> {
        self.buf.get(self.buf.len() - SZ_MD5_DIGEST..)
    }

    fn update_buf_manifest_from_meta_changes(
        &mut self,
        old_meta: &SSXPTD0FileMetadata,
    ) -> TD0Result<()> {
        // NOTE: This function should be run _before_ any insertions or deletions
        // are made to to self.buf!
        //
        // NOTE2: old_meta **MUST** be a clone of the metadata that was created
        //        prior to adding or removing chunks.
        for name in old_meta.tags.keys() {
            let Some(new_tag) = self.meta.manifest_tag(name) else {
                continue;
            };
            let tag_real = manifest_tag_from_buf_mut(
                &mut self.buf,
                old_meta
                    .manifest_tag_pos(name)
                    .expect("old metadata was consistent."),
            )?;
            tag_real.set_chunk_pos(new_tag.chunk_pos())?;
            // That's it. In-file chunk headers don't carry their own position.
        }
        Ok(())
    }

    fn validate_expected_file_size(&self) -> TD0Result<()> {
        if self.buf.len() == self.calc_expected_size() {
            Ok(())
        } else {
            Err(TD0Error::ValidationFailed(
                "incorrect file size.".to_string(),
            ))
        }
    }

    fn validate_has_backup_hdr_chunk(&self) -> TD0Result<()> {
        if self.meta.tags.contains_key("HDRa") {
            Ok(())
        } else {
            Err(TD0Error::ValidationFailed(
                "missing TD0a backup header.".to_string(),
            ))
        }
    }

    fn validate_manifest_tags_are_unique(&self) -> TD0Result<()> {
        let mut unique: HashSet<&String> = HashSet::new();

        for tag in self.meta.tags.keys() {
            if !unique.insert(tag) {
                return Err(TD0Error::ValidationFailed(format!(
                    "duplicate manifest tag '{tag}'"
                )));
            }
        }
        Ok(())
    }

    fn validate_manifest_tags_correct_model(&self) -> TD0Result<()> {
        for (chunk_name, tag) in self.meta.tags.iter() {
            if tag.model() != "SSXP" {
                return Err(TD0Error::ValidationFailed(format!(
                    "manifest tag '{}' has incorrect model '{}'.",
                    chunk_name,
                    tag.model()
                )));
            }
        }

        Ok(())
    }

    fn validate_manifest_tags_have_chunk_headers(&self) -> TD0Result<()> {
        for tag in self.meta.tags.values() {
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

        for tag_name in self.meta.tag_order.iter() {
            let tag = self
                .meta
                .manifest_tag(tag_name)
                .ok_or(TD0Error::ValidationFailed(format!(
                    "manifest tag order has name '{}' with no corresponding tag.",
                    tag_name
                )))?;

            if td0_core::calc_range_overlap(tag.chunk_range(), last_range.clone()).is_some() {
                return Err(TD0Error::ValidationFailed(
                    "overlapping chunks.".to_string(),
                ));
            }

            if tag.chunk_range().start <= last_range.start {
                return Err(TD0Error::ValidationFailed(
                    "manifest tags out of order.".to_string(),
                ));
            }

            last_range = tag.chunk_range();
        }

        Ok(())
    }

    fn validate_chunk_headers_chunks_exist(&self) -> TD0Result<()> {
        for tag in self.meta.tags.values() {
            let chunk_header = chunk_header_ref_from_buf(&self.buf, tag)?;

            // Calculate the pos of the first element of the next part of the file.
            // (either a chunk or the checksum.)
            let next_part_pos = tag.chunk_pos()
                + chunk_header.first_item_offset()
                + (chunk_header.item_size() * chunk_header.num_items());
            if next_part_pos > self.buf.len() {
                return Err(TD0Error::ValidationFailed(
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
        .ok_or(TD0Error::unknown_chunk_error(chunk_name))?;

        let item_size = size_of_val(&*default_item);

        // The order here matters quite a lot:
        //  1) Make a clone of the old metadata.
        //  2) Push the new chunk to self.meta.
        //  3) Call self.update_buf_manifest_from_meta_changes with the old metadata.
        //  - 4 and 5 can trade places without any ill effect.
        //  4) Insert new manifest tag at old meta's .header_size() offset.
        //  5) Insert the new chunk header + default chunk items at the end
        //     of the buffer right before the checksum.
        let old_meta = self.meta.clone();

        self.meta.push_chunk(
            chunk_name,
            TD0DeviceModel::SPDSXPro,
            try_u32_from_usize(num_items)?,
            try_u32_from_usize(item_size)?,
        )?;
        self.update_buf_manifest_from_meta_changes(&old_meta)?;
        let new_chunk_header = self
            .meta
            .chunk(chunk_name)
            .ok_or(TD0Error::InvalidChunk(InvalidChunkError::new(
                chunk_name,
                "failed to add chunk to file metadata.",
            )))?
            .header;
        let new_manifest_tag = self
            .meta
            .manifest_tag(chunk_name)
            .expect("just added this manifest tag.");
        let new_manifest_tag_pos = self
            .meta
            .manifest_tag_pos(chunk_name)
            .expect("just added this manifest tag.");

        // Insert new manifest tag:
        self.buf.splice(
            new_manifest_tag_pos..new_manifest_tag_pos,
            new_manifest_tag.as_bytes().iter().cloned(),
        );

        let buflen = self.buf.len();
        let checksum = self.buf.split_off(buflen - SZ_MD5_DIGEST);
        self.buf.extend_from_slice(new_chunk_header.as_bytes());
        let default_item_bytes = default_item.as_bytes();
        for _ in 0..num_items {
            self.buf.extend_from_slice(default_item_bytes);
        }
        self.buf.extend_from_slice(&checksum);
        self.dirty = true;

        Ok(())
    }

    fn remove_chunk(&mut self, chunk_name: &str) -> TD0Result<()> {
        let old_meta = self.meta.clone();
        self.meta.remove_chunk(chunk_name)?;

        let remove_chunk_range = old_meta
            .manifest_tag(chunk_name)
            .map(|tag| tag.chunk_range())
            .expect("chunk's existence in metadata verified above.");
        let remove_tag_offset = old_meta.manifest_tag_pos(chunk_name).unwrap();

        self.update_buf_manifest_from_meta_changes(&old_meta)?;

        self.buf.drain(remove_chunk_range);
        self.buf
            .drain(remove_tag_offset..remove_tag_offset + size_of::<TD0ManifestTag>());

        self.dirty = true;
        Ok(())
    }

    fn chunk_item_replace(
        &mut self,
        chunk_name: &str,
        dest_index: usize,
        source: &dyn TD0ChunkItem,
    ) -> TD0Result<()> {
        let chunk = self
            .meta
            .chunk(chunk_name)
            .ok_or(TD0Error::unknown_chunk_error(chunk_name))?;
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
        let chunk = self
            .meta
            .chunk(chunk_name)
            .ok_or(TD0Error::unknown_chunk_error(chunk_name))?;
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
        let chunk = self
            .meta
            .chunk(chunk_name)
            .ok_or(TD0Error::unknown_chunk_error(chunk_name))?;
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
        let chunk = self
            .meta
            .chunk(chunk_name)
            .ok_or(TD0Error::unknown_chunk_error(chunk_name))?;

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

        let mut new_buf: Vec<u8> = vec![0; usize_from_u32(chunk.size) - size_of::<ChunkHeader>()];

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
            .get_mut(
                usize_from_u32(chunk.pos) + chunk.header.first_item_offset()
                    ..usize_from_u32(chunk.pos + chunk.size),
            )
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
        let meta: SSXPTD0FileMetadata = SSXPTD0FileMetadata::new_from_buf(bytes)?;

        Ok(Self {
            buf: bytes.to_vec(),
            dirty: false,
            meta,
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
        self.meta.chunk(chunk_name).map(|ch| ch.num_items())
    }

    fn chunk_raw(&self, chunk_name: &str) -> Option<&[u8]> {
        let tag = self.meta.manifest_tag(chunk_name)?;
        self.buf.get(tag.chunk_range())
    }

    fn chunk_item(&self, chunk_name: &str, item_index: usize) -> TD0Result<&dyn TD0ChunkItem> {
        let chunk = self
            .meta
            .chunk(chunk_name)
            .ok_or(TD0Error::unknown_chunk_error(chunk_name))?;
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
        let chunk = self
            .meta
            .chunk(chunk_name)
            .ok_or(TD0Error::unknown_chunk_error(chunk_name))?;
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
        let chunk = self
            .meta
            .chunk(chunk_name)
            .ok_or(TD0Error::unknown_chunk_error(chunk_name))?;

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
        let chunk = self
            .meta
            .chunk(chunk_name)
            .ok_or(TD0Error::unknown_chunk_error(chunk_name))?;

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
        self.meta.chunk(chunk_name).map(|ch| usize_from_u32(ch.pos))
    }

    fn chunk_size(&self, chunk_name: &str) -> Option<usize> {
        self.meta
            .chunk(chunk_name)
            .map(|ch| usize_from_u32(ch.size))
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
            .meta
            .tag_order
            .iter()
            .map(|ch_name| {
                (
                    ch_name,
                    self.meta
                        .chunk(ch_name)
                        .expect("should be a chunk for each item in the chunk index."),
                )
            })
            .map(|(name, ch)| {
                ChunkManifest::new(
                    name.clone(),
                    usize_from_u32(ch.pos),
                    usize_from_u32(ch.size),
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

    fn new() -> Self {
        const HEADER_SIZE: usize = size_of::<TD0IdChunk>() + size_of::<TD0ManifestTag>();
        const BODY_SIZE: usize = size_of::<ChunkHeader>() + size_of::<HDRaItem>();
        const BUF_SIZE: usize = HEADER_SIZE + BODY_SIZE + SZ_MD5_DIGEST;

        // Set values for TDOa ID tag.
        let mut buf = vec![0u8; BUF_SIZE];
        let id_tag = TD0IdChunk::mut_from_bytes(
            buf.get_mut(..size_of::<TD0IdChunk>()).unwrap()
        ).unwrap();
        id_tag.set_magic(&TD0_MAGIC);
        id_tag.set_bytes_remaining(
            u16::try_from(HEADER_SIZE - OFFSET_BYTES_REMAINING).unwrap()
        );
        let mut pos: usize = size_of::<TD0IdChunk>();

        // Set values for HDRa tag.
        let header_tag = TD0ManifestTag::mut_from_bytes(
            buf.get_mut(pos..pos + size_of::<TD0ManifestTag>()).unwrap()
        ).unwrap();
        pos += size_of::<TD0ManifestTag>();
        header_tag.set_tag("HDRa").unwrap();
        header_tag.set_model("SSXP").unwrap();
        header_tag.set_chunk_pos(HEADER_SIZE).unwrap();
        header_tag.set_chunk_size(size_of::<ChunkHeader>() + size_of::<HDRaItem>()).unwrap();

        // Set buf bytes from the default backup chunk header.
        let chunk_header = HDRaItem::default_header();
        buf.get_mut(pos..pos + size_of::<ChunkHeader>()).unwrap()
            .copy_from_slice(IntoBytes::as_bytes(&chunk_header));
        pos += size_of::<ChunkHeader>();

        // Set buf bytes from the default (v1.10) backup HDRaItem.
        let item = HDRaItem::new_default_v1_10();
        buf.get_mut(pos..pos + size_of::<HDRaItem>()).unwrap()
            .copy_from_slice(IntoBytes::as_bytes(&item));

        // Write checksum.
        let mut hasher: md5::Md5 = md5::Md5::new();
        hasher.update(buf.get(..BUF_SIZE - SZ_MD5_DIGEST).unwrap());
        buf.get_mut(BUF_SIZE - SZ_MD5_DIGEST..).unwrap()
            .copy_from_slice(hasher.finalize().as_bytes());

        let meta: SSXPTD0FileMetadata = SSXPTD0FileMetadata::new_from_buf(&buf).expect("all data is bespoke.");

        Self {
            buf,
            dirty: false,
            meta,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.buf.clone()
    }

    fn list_chunks(&self) -> Vec<String> {
        self.meta.tag_order.clone()
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
            self.meta.tag_order.len()
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
            pos: try_u32_from_usize(CHUNK_POS).expect("or test is broken."),
            size: try_u32_from_usize(100 + HDR_SIZE).expect("or test is broken."),
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
            pos: try_u32_from_usize(CHUNK_POS).expect("or test is broken."),
            size: 116,
        };

        assert_eq!(chunk.item_range(0), Some(ITEM_0_POS..ITEM_0_POS + 20));
        assert_eq!(chunk.item_range(1), Some(ITEM_0_POS + 20..ITEM_0_POS + 40));
        assert_eq!(chunk.item_range(4), Some(ITEM_0_POS + 80..ITEM_0_POS + 100));
        assert_eq!(chunk.item_range(5), None);
    }
}
