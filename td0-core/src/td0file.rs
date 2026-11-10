// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use bytes::BytesMut;
use std::collections::{HashMap, HashSet};
use zerocopy::{FromBytes as _, IntoBytes as _};

use super::ascii_string::{ChunkTagStr, ChunkTagString};
use super::td0chunk::{ChunkHeader, ChunkItemAccessorCallbacks, TD0Chunk, TD0ChunkDiff};
use crate::header::{HDRaItem, OFFSET_BYTES_REMAINING, TD0IdChunk, TD0ManifestTag};
use crate::helpers::{SZ_MD5_DIGEST, bytes_or_error, calc_md5_checksum};
use crate::result::{TD0Error, TD0Result};
use crate::td0chunk::FieldsDiff;
use crate::types::{ChunkManifest, Diff, TD0BackupType, TD0ChunkItem, TD0DeviceModel, TD0Manifest};

pub const ERR_CHUNK_OOB: &str = "chunk extends beyond file buffer boundaries.";
pub const ERR_MANIFEST_SIZE: &str = "incorrect manifest size.";
pub const ERR_MANIFEST_TAG_INVALID: &str = "invalid manifest tag.";
pub const ERR_MANIFEST_TAG_DUPLICATE: &str = "duplicate manifest tag.";
const ERR_BACKUP_HEADER_MISSING: &str = "file is missing backup header.";
const ERR_MANIFEST_CHUNK_OUT_OF_ORDER: &str = "manifest chunks out of order.";
const ERR_MANIFEST_TAG_PARSE: &str = "unable to parse manifest tag.";
const ERR_MANIFEST_TAG_READ: &str = "unable to read manifest tag.";
const ERR_TD0_CHUNK_READ: &str = "unable to read TD0 ID chunk.";

const DEFAULT_VALID_CHUNKS: [&str; 1] = ["HDRa"];

pub struct TD0FileDiff {
    pub id_chunk_unknown: Option<Diff<[u8; 10]>>,
    pub manifest_tags: Option<Vec<Diff<TD0ManifestTag>>>,
    pub chunk_order: Option<Diff<Vec<String>>>,
    pub chunks: Option<Vec<TD0ChunkDiff>>,
    pub checksum: Option<Diff<Vec<u8>>>,
}

type ChunkTagMap = HashMap<ChunkTagString, BytesMut>;

/// The parsed, in-memory representation of a .TD0 backup file.
///
/// A `TD0File` consists of file manifest, which lists the file's data chunks
/// as well as the data chunks themselves (represented in this library as
/// [`TD0Chunk`] structs.) `TD0Chunk`s contain one or more items, structs that
/// implement the [`TD0ChunkItem`] trait, which provide access to the values
/// (e.g. Kit config data, wav file metadata, playlist items) stored within the
/// backup file.
#[derive(Clone, Eq)]
pub struct TD0File {
    firmware_version: String,
    id_chunk: BytesMut,
    tags: ChunkTagMap,
    tag_order: Vec<ChunkTagString>,
    chunks: HashMap<String, TD0Chunk>,
    checksum: BytesMut,

    chunk_callbacks_factory:
        fn(chunk_name: &str, firmware_version: &str) -> Option<ChunkItemAccessorCallbacks>,
    valid_chunk_names_callback: fn(firmware_version: &str) -> Option<&'static [&'static str]>,
    is_dirty: bool,
}

impl std::fmt::Debug for TD0File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.manifest())
    }
}

impl Default for TD0File {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for TD0File {
    fn eq(&self, other: &Self) -> bool {
        self.firmware_version == other.firmware_version
            && self.id_chunk == other.id_chunk
            && self.tags == other.tags
            && self.tag_order == other.tag_order
            && self.checksum == other.checksum
    }
}

impl TD0File {
    const FACTORY_BACKUP_FIRMWARE: &str = "9.67";
    const FACTORY_BACKUP_BUILD: &str = "0067";
    const FACTORY_BACKUP_SERIAL: &str = "0142345";

    const BACKUP_TAG_SYSTEM: &str = "SSXPROBK";
    const BACKUP_TAG_KIT: &str = "SSXPROKT";

    const DEFAULT_BACKUP_TAG: &str = Self::BACKUP_TAG_SYSTEM;
    const DEFAULT_HDR_TAG: &str = "HDRa";
    const DEFAULT_MODEL: &str = "SSXP";

    /// Add a chunk of `chunk_name` type to the file, without verifying that it is appropriate for the file.
    ///
    /// A mutable reference to the newly added chunk is returned.
    ///
    /// A default [`TD0ChunkItem`] must be supplied, which will be copied `num_items` times to create the
    /// chunk buffer.
    ///
    /// # Errors
    /// Errors may vary based on model/firmware-specific [`TD0ChunkItem`] implementation.
    /// Expected errors include:
    /// - [`TD0Error::FileTooLarge`](crate::result::TD0Error::OutOfRange)
    ///   - if the supplied `default_item`'s size is larger than the file's native
    ///     representation allows.
    ///   - if the new chunk offset would be larger than the file format
    ///     allows.
    ///
    /// # Example
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, ChunkTagStr, TD0Chunk, TD0ChunkItem, TD0File};
    ///
    /// // Load the initial file, which contains a single chunk: `HDRa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_v1_10.TD0");
    /// let mut target_file : TD0File = parse_td0_file(TD0_BYTES)?;
    ///
    /// // Load another file to get a chunk item from. This file contains a `TGLa` chunk.
    /// const CHUNK_SOURCE_BYTES: &[u8] = include_bytes!("../example/data/example_tgl_items.TD0");
    /// let source_file : TD0File = parse_td0_file(CHUNK_SOURCE_BYTES)?;
    /// let source_chunk : &TD0Chunk = source_file.chunk("TGLa").ok_or(TD0Error::unknown_chunk_error("TGLa"))?;
    /// let default_item : &dyn TD0ChunkItem = source_chunk.item_at_ref(0)?;
    ///
    /// // Add a new chunk with five items, and verify that it was added.
    /// let chunk_name : &ChunkTagStr = "FOOa".try_into()?;
    /// target_file.add_chunk_unchecked(chunk_name, 5, default_item)?;
    /// assert_eq!(target_file.list_chunks(), vec!["HDRa", "FOOa"]);
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    pub fn add_chunk_unchecked(
        &mut self,
        chunk_name: &ChunkTagStr,
        num_items: usize,
        default_item: &dyn TD0ChunkItem,
    ) -> TD0Result<&mut TD0Chunk> {
        let chunk_name: ChunkTagString = chunk_name.into();
        let chunk = TD0Chunk::new_with(num_items, default_item)?;
        self.add_tag(&chunk_name, chunk.size())?;
        self.chunks.insert(chunk_name.to_string(), chunk);

        self.is_dirty = true;
        Ok(self
            .chunks
            .get_mut(&chunk_name.to_string())
            .expect("just added this"))
    }

    /// Return the valid chunk names for this `TD0File`, based on its device model and firmware revision.
    ///
    /// ## Note
    /// All possible valid chunk names are returned, regardless of their presence or absence in the current file.
    /// To retrieve only the chunks found in the current file, use the [`TD0File::list_chunks`] method.
    ///
    /// # Example
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{new_td0_file, TD0File, TD0DeviceModel, TD0BackupType};
    ///
    /// let file_v1_10 : TD0File = new_td0_file(TD0DeviceModel::SPDSXPro, "1.10")?;
    /// assert_eq!(file_v1_10.valid_chunks(), &[
    ///     "HDRa", "KITa", "CURa", "STLa", "STPa", "TGLa", "TRGa", "WVPa",
    /// ]);
    ///
    /// let file_v2_00 : TD0File = new_td0_file(TD0DeviceModel::SPDSXPro, "2.00")?;
    /// assert_eq!(file_v2_00.valid_chunks(), &[
    ///     "HDRa", "KITa", "KITb", "CURa", "PVRa", "STLa", "STPa", "STPb", "TGLa", "TRGa", "WVPa",
    /// ]);
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    pub fn valid_chunks(&self) -> &'static [&'static str] {
        let firmware = self.firmware_version();
        (self.valid_chunk_names_callback)(firmware).unwrap_or(&DEFAULT_VALID_CHUNKS)
    }

    /// Add a chunk of `chunk_name` type to the file, containing `num_items` [`TD0ChunkItem`] items of the
    /// appropriate type.
    ///
    /// The newly created items will be default [`TD0ChunkItem`] items of the expected type for `chunk_name`.
    ///
    /// # Errors
    /// Errors may vary based on model/firmware-specific [`TD0ChunkItem`] implementation.
    /// Expected errors include:
    /// - [`TD0Error::InvalidChunk`](crate::result::TD0Error::InvalidChunk)
    ///   - if `chunk_name` is not a valid chunk name. (This can differ per
    ///     device model and per firmware revision.)
    ///   - if a valid chunk header tag was unable to be created.
    /// - [`TD0Error::FileParse`](crate::result::TD0Error::FileParse) if this
    ///   file's firmware version and/or model can't be determined.
    /// - [`TD0Error::FileTooLarge`](crate::result::TD0Error::OutOfRange)
    ///   - if the supplied `item_size` is larger than the file's native
    ///     representation allows.
    ///   - if the new chunk offset would be larger than the file format
    ///     allows.
    ///
    /// # Example
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, ChunkTagStr, TD0File};
    ///
    /// // Load the initial file, which contains a single chunk: `HDRa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_v1_10.TD0");
    /// let mut td0file : TD0File = parse_td0_file(TD0_BYTES)?;
    ///
    /// // Add a `TGLa` chunk with five items, and verify that it was added.
    /// let chunk_name : &ChunkTagStr = "TGLa".try_into()?;
    /// td0file.add_chunk(chunk_name, 5)?;
    /// assert_eq!(td0file.list_chunks(), vec!["HDRa", "TGLa"]);
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    pub fn add_chunk(
        &mut self,
        chunk_name: &ChunkTagStr,
        num_items: usize,
    ) -> TD0Result<&mut TD0Chunk> {
        if self.valid_chunks().contains(&chunk_name.as_ref()) {
            let Some(chunk_callbacks) =
                (self.chunk_callbacks_factory)(chunk_name.as_ref(), self.firmware_version())
            else {
                return Err(TD0Error::invalid_chunk_error(
                    chunk_name,
                    "unable to determine default chunk item.",
                ));
            };

            let default_item = (chunk_callbacks.default_chunk_item_cb)();
            let mut chunk = TD0Chunk::new_with(num_items, &*default_item)?;
            chunk.set_item_callbacks(chunk_callbacks);

            self.add_tag(&chunk_name.into(), chunk.size())?;
            self.chunks.insert(chunk_name.to_string(), chunk);
            self.is_dirty = true;
            Ok(self
                .chunks
                .get_mut(chunk_name.as_ref())
                .expect("just added this"))
        } else {
            Err(TD0Error::unknown_chunk_error(chunk_name))
        }
    }

    /// Calculate the checksum for this file's data.
    ///
    /// The file's data in this case means the entirety of the file *except* the included
    /// checksum. The calculated checksum should match the included checksum if the file
    /// hasn't been modified.
    ///
    /// # Example
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0File};
    ///
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_v1_10.TD0");
    /// let expected : Vec<u8> = TD0_BYTES[TD0_BYTES.len() - 16..].to_vec();
    /// let td0file : TD0File = parse_td0_file(TD0_BYTES)?;
    ///
    /// assert_eq!(td0file.calc_checksum(), expected);
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    pub fn calc_checksum(&self) -> Vec<u8> {
        let mut all_bufs: Vec<&[u8]> = self
            .collect_buffers()
            .into_iter()
            .map(core::convert::AsRef::as_ref)
            .collect();
        all_bufs.pop(); // Remove the existing checksum before calculating.

        calc_md5_checksum(&all_bufs).into()
    }

    /// Retrieve a reference to [`TD0Chunk`] `chunk_name`, or None if no such chunk exists.
    ///
    /// # Example
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0Chunk, TD0File};
    /// use std::assert_matches;
    ///
    /// // Load the example file, which contains a single chunk: `HDRa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_v1_10.TD0");
    /// let td0file : TD0File = parse_td0_file(TD0_BYTES)?;
    ///
    /// // Retrieve a chunk and verify that it worked.
    /// let result = td0file.chunk("HDRa");
    /// assert_matches!(result, Some(TD0Chunk{..}));
    ///
    /// // An invalid chunk name will return None:
    /// let result = td0file.chunk("ZZZa");
    /// assert_eq!(result, None);
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    pub fn chunk(&self, chunk_name: &str) -> Option<&TD0Chunk> {
        self.chunks.get(chunk_name)
    }

    pub fn chunk_info(&self, chunk_name: &str) -> Option<ChunkManifest> {
        let chunk_name: &ChunkTagStr = chunk_name.try_into().ok()?;

        self.tag(chunk_name).map(|tag| {
            let chunk_header = self
                .chunk(chunk_name)
                .expect("file manifest and chunks are in sync.")
                .header();

            ChunkManifest {
                name: chunk_name.to_string(),
                item_size: chunk_header.item_size(),
                num_items: chunk_header.num_items(),
                pos: tag.chunk_pos(),
                size: tag.chunk_size(),
            }
        })
    }

    /// Retrieve a mutable reference to [`TD0Chunk`] `chunk_name`, or None if no such chunk exists.
    ///
    /// # Example
    /// ```
    /// use td0::{parse_td0_file, TD0Chunk, TD0ChunkItem, TD0Error, TD0File};
    /// use std::assert_matches;
    ///
    /// // Load the example file, which contains a single chunk: `HDRa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_v1_10.TD0");
    /// let mut td0file : TD0File = parse_td0_file(TD0_BYTES)?;
    ///
    /// let chunk : &mut TD0Chunk = td0file.chunk_mut("HDRa").ok_or(TD0Error::unknown_chunk_error("HDRa"))?;
    /// let _chunk_item : &mut dyn TD0ChunkItem = chunk.item_at_mut(0)?;
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    pub fn chunk_mut(&mut self, chunk_name: &str) -> Option<&mut TD0Chunk> {
        match self.chunks.get_mut(chunk_name) {
            Some(chunk) => {
                self.is_dirty = true;
                Some(chunk)
            }
            None => None,
        }
    }

    /// Return an Option containing the [`TD0FileDiff`] of this file and `other`.
    ///
    /// Ok(None) is returned if there are no differences.
    ///
    /// # Errors
    /// - [`TD0Error::FileParse`] if chunks in either file can't be parsed.
    /// - [`TD0Error::ChunksNotComparable`] if identically named chunks have differing
    ///   item types.
    pub fn diff(&self, other: &Self) -> TD0Result<Option<TD0FileDiff>> {
        use zerocopy::FromBytes;

        let mut id_chunk_unknown: Option<Diff<[u8; 10]>> = None;
        let mut manifest_tags: Option<Vec<Diff<TD0ManifestTag>>> = None;
        let mut chunk_order: Option<Diff<Vec<String>>> = None;
        let mut backup_header: Option<FieldsDiff> = None;
        let mut chunks: Option<Vec<TD0ChunkDiff>> = None;
        let mut checksum: Option<Diff<Vec<u8>>> = None;

        // ID chunk
        if self.id_chunk != other.id_chunk {
            let (left, right) = (
                *TD0IdChunk::ref_from_bytes(&self.id_chunk)
                    .map_err(|_| TD0Error::file_parse_error(ERR_TD0_CHUNK_READ))?
                    .unknown(),
                *TD0IdChunk::ref_from_bytes(&other.id_chunk)
                    .map_err(|_| TD0Error::file_parse_error(ERR_TD0_CHUNK_READ))?
                    .unknown(),
            );
            if left != right {
                id_chunk_unknown = Some(Diff { left, right });
            }
        }

        // Manifest Tags + Chunks
        let mut manifest_tag_diffs: Vec<Diff<TD0ManifestTag>> = Vec::new();
        let mut chunk_diffs: Vec<TD0ChunkDiff> = Vec::new();
        for chunk_name in &self.tag_order {
            if !other.tags.contains_key(chunk_name) {
                continue;
            }

            let (left, right) = (
                self.tag(chunk_name)
                    .expect("tag_order and tags are consistent."),
                other.tag(chunk_name).unwrap(),
            );
            if left != right {
                manifest_tag_diffs.push(Diff {
                    left: *left,
                    right: *right,
                });
            }

            let (left, right) = (
                self.chunk(chunk_name).unwrap(),
                other.chunk(chunk_name).unwrap(),
            );
            if let Some(chunk_diff) = left.diff(right)? {
                chunk_diffs.push(chunk_diff);
            }
        }
        if !manifest_tag_diffs.is_empty() {
            manifest_tags = Some(manifest_tag_diffs);
        }
        if !chunk_diffs.is_empty() {
            chunks = Some(chunk_diffs);
        }

        // Backup Header
        let (left_backup_header, right_backup_header) =
            (self.backup_manifest(), other.backup_manifest());
        if left_backup_header != right_backup_header {
            let mut backup_fields: FieldsDiff = FieldsDiff::new();
            for field_name in left_backup_header.list_fields() {
                let (left, right) = (
                    left_backup_header.field_value(field_name).unwrap(),
                    right_backup_header.field_value(field_name).unwrap(),
                );
                if left != right {
                    backup_fields.push((field_name.to_string(), Diff { left, right }));
                }
            }

            backup_header = Some(backup_fields);
        }

        // Chunk Order
        if self.tag_order != other.tag_order {
            chunk_order = Some(Diff::<Vec<String>> {
                left: self
                    .tag_order
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
                right: other
                    .tag_order
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            });
        }

        // Checksum
        let (left, right) = (self.calc_checksum(), other.calc_checksum());
        if left != right {
            checksum = Some(Diff { left, right });
        }

        if id_chunk_unknown.is_some()
            || manifest_tags.is_some()
            || backup_header.is_some()
            || chunks.is_some()
            || checksum.is_some()
        {
            Ok(Some(TD0FileDiff {
                id_chunk_unknown,
                manifest_tags,
                chunk_order,
                chunks,
                checksum,
            }))
        } else {
            Ok(None)
        }
    }

    /// Return the firmware version of this file.
    pub const fn firmware_version(&self) -> &str {
        self.firmware_version.as_str()
    }

    /// Return true if this file contains a chunk named `chunk_name`.
    pub fn has_chunk(&self, chunk_name: &str) -> bool {
        self.chunks.contains_key(chunk_name)
    }

    /// Return a list of the names of the chunks present in this file.
    ///
    /// ## Note
    /// This does not list all possible valid chunk names, only those that exist in this file.
    /// To retrieve all valid chunk names for this type of file, use the [`TD0File::valid_chunks`] method.
    ///
    /// # Example
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0File};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_two_chunks.TD0");
    /// let td0file : TD0File = parse_td0_file(TD0_BYTES)?;
    ///
    /// assert_eq!(td0file.list_chunks(), vec!["HDRa", "TGLa"]);
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    pub fn list_chunks(&self) -> Vec<&ChunkTagString> {
        self.tag_order.iter().collect()
    }

    pub fn model(&self) -> TD0DeviceModel {
        let model_str = TD0ManifestTag::ref_from_bytes(
            self.tags[&ChunkTagString::from_string_unchecked("HDRa")].as_ref(),
        )
        .expect("file has a HDRa manifest tag.")
        .model();

        TD0DeviceModel::try_from(model_str.as_str()).unwrap_or(TD0DeviceModel::Unknown)
    }

    /// Return a [`TD0Manifest`] containing metadata about the file and its data chunks.
    ///
    /// # Errors:
    /// - [`TD0Error::FileParse`](crate::result::TD0Error::FileParse)
    ///   - if the file header cannot be parsed.
    ///   - if the file manifest tags can't be parsed.
    ///   - if the file backup manifest chunk (`HDRa`) isn't present or can't be parsed.
    /// - [`TD0Error::InvalidChunk`](crate::result::TD0Error::InvalidChunk) if chunk header data in the file can't
    ///   be parsed.
    ///
    /// # Example
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0BackupType, TD0DeviceModel, TD0File, TD0Manifest};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_two_chunks.TD0");
    /// let td0file : TD0File = parse_td0_file(TD0_BYTES)?;
    ///
    /// let manifest : TD0Manifest = td0file.manifest();
    ///
    /// // Test that the manifest has retrievable data.
    /// assert_eq!(manifest.backup_name(), "USER KIT        ");
    /// assert_eq!(manifest.backup_type(), TD0BackupType::Kit);
    /// assert_eq!(manifest.device_model(), TD0DeviceModel::SPDSXPro);
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    pub fn manifest(&self) -> TD0Manifest {
        let backup_header: &HDRaItem = self.backup_manifest();
        let backup_type: TD0BackupType = match backup_header.tag().as_str() {
            Self::BACKUP_TAG_KIT => TD0BackupType::Kit,
            Self::BACKUP_TAG_SYSTEM => TD0BackupType::System,
            _ => TD0BackupType::Unknown,
        };

        let chunks: Vec<ChunkManifest> = self
            .tag_order
            .iter()
            .map(|tag_name| self.chunk_info(tag_name).unwrap())
            .collect();

        TD0Manifest {
            backup_name: backup_header.name(),
            backup_type,
            checksum_actual: self.read_checksum(),
            checksum_calculated: self.calc_checksum(),
            device_model: self.model(),
            device_firmware_version: self.firmware_version().to_string(),
            device_firmware_build: backup_header.build(),
            device_serial: backup_header.device_serial(),
            size_actual: self.size(),
            size_calculated: self.calc_size(),
            chunks,
        }
    }

    /// Return a new file with defaults for model and firmware revision.
    pub fn new() -> Self {
        use zerocopy::IntoBytes;

        const SZ_HEADER: usize = size_of::<TD0IdChunk>() + size_of::<TD0ManifestTag>();

        // Create id chunk
        let mut id_chunk_item: TD0IdChunk = TD0IdChunk::default();
        id_chunk_item
            .set_bytes_remaining(u16::try_from(SZ_HEADER - OFFSET_BYTES_REMAINING).unwrap());

        // Create manifest tag.
        let mut tag_item: TD0ManifestTag = TD0ManifestTag::default();
        tag_item.set_chunk_pos(SZ_HEADER).unwrap();
        tag_item
            .set_chunk_size(size_of::<ChunkHeader>() + size_of::<HDRaItem>())
            .unwrap();
        tag_item.set_tag(Self::DEFAULT_HDR_TAG).unwrap();
        tag_item.set_model(Self::DEFAULT_MODEL).unwrap();

        // Create backup header (HDRa).
        let mut backup_chunk_header: ChunkHeader = ChunkHeader::default();
        backup_chunk_header.set_first_item_offset(u32::try_from(size_of::<ChunkHeader>()).unwrap());
        backup_chunk_header.set_item_size(u32::try_from(size_of::<HDRaItem>()).unwrap());
        backup_chunk_header.set_num_items(1);

        // Create backup chunk item.
        let backup_item: HDRaItem = HDRaItem::default_parsable();

        let tag_name: ChunkTagString = ChunkTagString::from_string_unchecked("HDRa".to_string());

        // Coerce created items into BytesMut items.
        let id_chunk: BytesMut = BytesMut::from(id_chunk_item.as_bytes());
        let tag_bytes: BytesMut = BytesMut::from(tag_item.as_bytes());
        let backup_item_bytes: BytesMut = BytesMut::from(TD0ChunkItem::as_bytes(&backup_item));

        // Create checksum.
        let backup_chunk_header_bytes: BytesMut = BytesMut::from(backup_chunk_header.as_bytes());
        let checksum: BytesMut = BytesMut::from(
            calc_md5_checksum(&[
                &id_chunk,
                &tag_bytes,
                &backup_chunk_header_bytes,
                &backup_item_bytes,
            ])
            .as_slice(),
        );

        let chunk: TD0Chunk = TD0Chunk::new(
            BytesMut::from(backup_chunk_header.as_bytes()),
            BytesMut::new(),
            vec![backup_item_bytes],
            ChunkItemAccessorCallbacks::default(),
        );

        Self {
            firmware_version: backup_item.firmware(),
            id_chunk,
            tags: ChunkTagMap::from_iter(vec![(tag_name.clone(), tag_bytes)]),
            tag_order: vec![tag_name],
            chunks: HashMap::from_iter(vec![("HDRa".to_string(), chunk)]),
            checksum,

            chunk_callbacks_factory: default_chunk_callbacks_factory,
            valid_chunk_names_callback: default_valid_chunks_callback,
            is_dirty: false,
        }
    }

    fn update_chunk_callbacks(&mut self) {
        let firmware = &self.firmware_version().to_string();

        for (chunk_name, chunk) in &mut self.chunks {
            let callbacks_obj =
                (self.chunk_callbacks_factory)(chunk_name, firmware).unwrap_or_default();
            chunk.set_item_callbacks(callbacks_obj);
        }
    }

    /// Set the firmware version of this file.
    ///
    /// # Errors
    /// [`TD0Error::OutOfRange`] - If the length of `firmware` is incorrect.
    /// (Should be 4 characters.)
    /// [`TD0Error::InvalidInput`] - If `firmware` is not in the format "N.NN"
    /// where N is an ASCII digit (0-9).
    pub fn set_firmware_version(&mut self, version: &str) -> TD0Result<()> {
        self.backup_manifest_mut().set_firmware(version)?;
        self.firmware_version = self.backup_manifest().firmware();
        self.update_chunk_callbacks();
        self.is_dirty = true;
        Ok(())
    }

    /// Create a new `TD0File` with a specific firmware version and chunk item callbacks.
    ///
    /// # Errors
    /// See [`TD0File::set_firmware_version`].
    pub fn new_with(
        firmware_version: &str,
        chunk_callbacks_factory: fn(
            chunk_name: &str,
            firmware_version: &str,
        ) -> Option<ChunkItemAccessorCallbacks>,
        valid_chunk_names_callback: fn(firmware_version: &str) -> Option<&'static [&'static str]>,
    ) -> TD0Result<Self> {
        let mut newobj = Self::new();
        newobj.chunk_callbacks_factory = chunk_callbacks_factory;
        newobj.valid_chunk_names_callback = valid_chunk_names_callback;
        newobj.set_firmware_version(firmware_version)?;

        // Recalc the checksum because the firmware version changed.
        let checksum = newobj.calc_checksum();
        newobj.checksum.clone_from_slice(&checksum);
        newobj.is_dirty = false;

        Ok(newobj)
    }

    /// Parse the firmware version from a `HDRaItem` file backup header.
    ///
    /// # Errors
    ///
    pub fn parse_firmware_version(backup_header: &HDRaItem) -> TD0Result<String> {
        let declared = backup_header.firmware();

        if declared.as_str() == Self::FACTORY_BACKUP_FIRMWARE {
            if backup_header.build().as_str() == Self::FACTORY_BACKUP_BUILD
                && backup_header.device_serial() == Self::FACTORY_BACKUP_SERIAL
                && backup_header.tag() == Self::DEFAULT_BACKUP_TAG
            {
                // Extract actual firmware version from the 'name' field.
                let val = backup_header.name();
                let val = val.trim().strip_prefix("Contents_V").ok_or_else(|| {
                    TD0Error::file_parse_error("uanble to parse firmware version.")
                })?;

                for char in val.chars() {
                    if !char.is_ascii_digit() {
                        return Err(TD0Error::FileParse(
                            "unable to parse firmware version.".to_string(),
                        ));
                    }
                }
                let mut val = val.to_string();
                val.insert(1, '.');

                Ok(val
                    .get(0..4)
                    .ok_or_else(|| TD0Error::file_parse_error("unable to parse firmware version."))?
                    .to_string())
            } else {
                Err(TD0Error::FileParse(
                    "unable to parse firmware version.".to_string(),
                ))
            }
        } else {
            Ok(declared)
        }
    }

    /// Return the size of this file in bytes.
    pub fn size(&self) -> usize {
        self.collect_buffers()
            .iter()
            .fold(0usize, |acc, buf| acc + buf.len())
    }

    fn add_tag(&mut self, chunk_name: &ChunkTagString, chunk_size: usize) -> TD0Result<()> {
        if self.tags.contains_key(chunk_name) {
            return Err(TD0Error::DuplicateChunk(chunk_name.to_string()));
        }

        // Update the offsets in the manifest tags. Because the chunk is pushed onto the back
        // of all the chunks, each chunk is offset by +16 bytes (the size of a TD0ManifestTag.)
        let tag_order = { self.tag_order.clone() };
        let chunks_offset = i64::try_from(size_of::<TD0ManifestTag>()).unwrap();
        for tag_name in &tag_order {
            self.update_chunk_offset(tag_name, chunks_offset)?;
        }

        let next_chunk_pos: usize = tag_order
            .last()
            .map(|tag_name| {
                let tag = self.tag(tag_name).unwrap();
                tag.chunk_pos() + tag.chunk_size()
            })
            .expect("there is already at least one tag.");

        let mut manifest_tag: TD0ManifestTag = TD0ManifestTag::default();
        manifest_tag.set_chunk_pos(next_chunk_pos)?;
        manifest_tag.set_chunk_size(chunk_size)?;
        manifest_tag.set_model(self.model().as_str())?;
        manifest_tag.set_tag(chunk_name)?;

        self.tags
            .insert(chunk_name.clone(), BytesMut::from(manifest_tag.as_bytes()));
        self.tag_order.push(chunk_name.clone());
        let id_tag = TD0IdChunk::mut_from_bytes(&mut self.id_chunk).unwrap();
        id_tag.set_bytes_remaining(
            u16::try_from(id_tag.bytes_remaining() + size_of::<TD0ManifestTag>())
                .map_err(|_| TD0Error::FileTooLarge)?,
        );

        Ok(())
    }

    fn backup_manifest(&self) -> &HDRaItem {
        use zerocopy::FromBytes;

        HDRaItem::ref_from_bytes(self.chunks["HDRa"].item_at_raw(0).unwrap()).unwrap()
    }

    fn backup_manifest_mut(&mut self) -> &mut HDRaItem {
        use zerocopy::FromBytes;

        HDRaItem::mut_from_bytes(
            self.chunk_mut("HDRa")
                .expect("no file gets created without HDR.")
                .item_at_raw_mut(0)
                .expect("HDR has at least one item."),
        )
        .expect("can convert existing HDRaItem bytes to HDRaItem.")
    }

    fn calc_remove_chunk_offsets(
        &self,
        remove_chunk_tag: &TD0ManifestTag,
    ) -> HashMap<ChunkTagString, i64> {
        let (remove_chunk_pos, remove_chunk_size) =
            (remove_chunk_tag.chunk_pos(), remove_chunk_tag.chunk_size());

        // The chunks before remove_tag_pos positions are decremented by the size of a TD0ManifestTag.
        // The chunks afterware are decremented by the size of the chunk in addition to the manfiest tag.
        let before_offset: i64 = -i64::try_from(size_of::<TD0ManifestTag>()).unwrap();
        let after_offset: i64 =
            before_offset - i64::try_from(remove_chunk_size).expect("chunk size must be valid.");

        self.tag_order
            .iter()
            .map(|tag_name| {
                let tag = self.tag(tag_name).expect("tag_order and tags are in sync.");
                if tag.chunk_pos() <= remove_chunk_pos {
                    (tag_name.clone(), before_offset)
                } else {
                    (tag_name.clone(), after_offset)
                }
                // Intentionally excluding chunk_pos == remove_tag_pos.
            })
            .collect()
    }

    fn calc_size(&self) -> usize {
        // Return what the file size _should_ be based on the manifest data.
        // NOTE: To get the _actual_ size in bytes, call self.size() instead.
        let last_tag_name = self
            .tag_order
            .iter()
            .last()
            .expect("there is at least one chunk");
        let last_tag = self
            .tag(last_tag_name)
            .expect("tag_order and tags are in sync.");

        last_tag.chunk_pos() + last_tag.chunk_size() + SZ_MD5_DIGEST
    }

    fn collect_buffers(&self) -> Vec<&BytesMut> {
        // Precalculate the number of buffers that will be processed.
        let num_bufs: usize =
            self.chunks.len() * 3   // Count one for each chunk, its header, and its manifest tag.
            + 1                     // Count the ID chunk.
            + 1                     // Count the checksum.
        ;

        let mut all_bufs: Vec<&BytesMut> = Vec::with_capacity(num_bufs);

        // In order add:
        //  - ID chunk
        //  - Manifest
        //  - Chunk data
        //  - File Checksum
        all_bufs.push(&self.id_chunk);
        for tag_name in &self.tag_order {
            all_bufs.push(&self.tags[tag_name]);
        }

        for tag_name in &self.tag_order {
            let chunk = self
                .chunks
                .get(&tag_name.to_string())
                .expect("chunks and tag_order are consistent.");
            all_bufs.extend(chunk.collect_buffers());
        }

        all_bufs.push(&self.checksum);

        all_bufs
    }

    fn parse_chunks(
        bytes: &mut BytesMut,
        tags: &HashMap<ChunkTagString, BytesMut>,
        tag_order: &[ChunkTagString],
    ) -> TD0Result<HashMap<String, TD0Chunk>> {
        use zerocopy::FromBytes;

        let mut chunks: HashMap<String, TD0Chunk> = HashMap::new();

        for tag_name in tag_order {
            let tag = TD0ManifestTag::ref_from_bytes(
                tags.get(tag_name).expect("tag_order and tags are in sync."),
            )
            .unwrap();
            if tag.chunk_size() > bytes.len() {
                return Err(TD0Error::file_parse_error(ERR_CHUNK_OOB));
            }

            chunks.insert(
                tag_name.to_string(),
                TD0Chunk::try_from(bytes.split_to(tag.chunk_size()))?,
            );
        }

        Ok(chunks)
    }

    fn parse_manifest(
        buf: &mut BytesMut,
        id_chunk: &TD0IdChunk,
        file_size: usize,
    ) -> TD0Result<(ChunkTagMap, Vec<ChunkTagString>)> {
        use zerocopy::TryFromBytes;

        let mut manifest: ChunkTagMap = ChunkTagMap::new();
        let mut seen: HashSet<ChunkTagString> = HashSet::new();
        let mut tag_order: Vec<ChunkTagString> = Vec::new();

        let sz_manifest: usize = OFFSET_BYTES_REMAINING + id_chunk.bytes_remaining();
        let mut pos = size_of::<TD0IdChunk>();

        // Validate that manifest size as declared by the ID chunk is evenly
        // divisible into TD0ManifestTags.
        if !sz_manifest.is_multiple_of(size_of::<TD0ManifestTag>()) {
            return Err(TD0Error::file_parse_error(ERR_MANIFEST_SIZE));
        }

        let mut expected_next_chunk_pos = sz_manifest;

        // Parse manifest tags until the calculated size of manifest is reached.
        while pos < sz_manifest {
            if buf.len() < size_of::<TD0ManifestTag>() {
                return Err(TD0Error::file_parse_error(ERR_MANIFEST_TAG_READ));
            }
            let tag_bytes = buf.split_to(size_of::<TD0ManifestTag>());
            let tag = TD0ManifestTag::try_ref_from_bytes(&tag_bytes)
                .map_err(|_| TD0Error::file_parse_error(ERR_MANIFEST_TAG_PARSE))?;
            tag.validate(file_size)
                .map_err(|_| TD0Error::file_parse_error(ERR_MANIFEST_TAG_INVALID))?;
            // Ensure that each chunk in the manifest is in order and contiguous in the file.
            if tag.chunk_pos() == expected_next_chunk_pos {
                expected_next_chunk_pos = tag.chunk_pos() + tag.chunk_size();
            } else {
                return Err(TD0Error::file_parse_error(ERR_MANIFEST_CHUNK_OUT_OF_ORDER));
            }

            // Fill manifest and tag_order containers.
            let tag_name: ChunkTagString = ChunkTagString::try_from(tag.tag())?;
            if !seen.insert(tag_name.clone()) {
                return Err(TD0Error::file_parse_error(ERR_MANIFEST_TAG_DUPLICATE));
            }
            manifest.insert(tag_name.clone(), tag_bytes);
            tag_order.push(tag_name);
            pos += size_of::<TD0ManifestTag>();
        }

        Ok((manifest, tag_order))
    }

    /// Parse the model and firmware version from `raw` bytes.
    ///
    /// Returns a tuple (`TD0DeviceModel`, `firmware_version`).
    ///
    /// This should be used as the first step of the file parsing process, so that
    /// the appropriate `TD0Chunk` callbacks may be chosen based on model and firmware.
    ///
    /// # Errors
    /// Expected errors include:
    /// - [`TD0Error::FileParse`] if
    ///     - The file is not a TD0 file.
    ///     - A manifest tag is invalid.
    ///     - The backup header is missing or not parsable (this header contains the
    ///       firmware version.)
    ///
    /// # Example
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{TD0DeviceModel, TD0File};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_v1_10.TD0");
    /// let (model, firmware) : (TD0DeviceModel, String) = TD0File::parse_model_and_firmware(TD0_BYTES)?;
    ///
    /// assert_eq!(model, TD0DeviceModel::SPDSXPro);
    /// assert_eq!(firmware, "1.10");
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    pub fn parse_model_and_firmware(raw: &[u8]) -> TD0Result<(TD0DeviceModel, String)> {
        let (id_chunk, mut rem) = TD0IdChunk::ref_from_prefix(raw)
            .map_err(|_| TD0Error::file_parse_error(ERR_TD0_CHUNK_READ))?;

        let sz_header = OFFSET_BYTES_REMAINING + id_chunk.bytes_remaining();

        let mut pos = size_of::<TD0IdChunk>();
        let mut backup_tag: Option<&TD0ManifestTag> = None;

        while pos < sz_header {
            let (tag, new_rem) = TD0ManifestTag::ref_from_prefix(rem)
                .map_err(|_| TD0Error::file_parse_error(ERR_MANIFEST_TAG_PARSE))?;
            rem = new_rem;
            if tag.tag() == Self::DEFAULT_HDR_TAG {
                backup_tag = Some(tag);
                break;
            }
            pos += size_of::<TD0ManifestTag>();
        }

        let backup_tag: &TD0ManifestTag =
            backup_tag.ok_or_else(|| TD0Error::file_parse_error(ERR_BACKUP_HEADER_MISSING))?;
        let mut backup_item_range: std::ops::Range<usize> = backup_tag.chunk_range();
        backup_item_range.start += size_of::<ChunkHeader>();
        let backup_item_bytes: &[u8] = bytes_or_error(raw, backup_item_range, || {
            TD0Error::file_parse_error(ERR_BACKUP_HEADER_MISSING)
        })?;
        let backup_item = HDRaItem::ref_from_bytes(backup_item_bytes)
            .map_err(|_| TD0Error::invalid_chunk_error("HDRa", "unable to parse chunk."))?;
        Ok((
            TD0DeviceModel::try_from(backup_tag.model())?,
            Self::parse_firmware_version(backup_item).unwrap(),
        ))
    }

    /// Parse bytes into a `TD0File` with a specific chunk item callbacks.
    ///
    /// This is the correct way to parse a file while getting useful information from the chunk items.
    ///
    /// # Errors
    /// Expected errors include:
    /// - [`TD0Error::FileParse`] if
    ///     - The file is not a TD0 file.
    ///     - A manifest tag is invalid.
    ///     - The file backup chunk is missing or invalid.
    ///     - The firmware version is not parsable.
    ///     - The file is missing a checksum.
    ///     - The file contains unexpected data not described in the file manifest.
    /// - [`TD0Error::InvalidChunk`](crate::result::TD0Error::InvalidChunk) if a data chunk or its header is invalid.
    ///
    pub fn parse_with(
        bytes: &[u8],
        callbacks_factory: fn(
            chunk_name: &str,
            firmware_version: &str,
        ) -> Option<ChunkItemAccessorCallbacks>,
        valid_chunks_callback: fn(firmware_version: &str) -> Option<&'static [&'static str]>,
    ) -> TD0Result<Self> {
        let mut fileobj = Self::try_from(bytes)?;
        fileobj.chunk_callbacks_factory = callbacks_factory;
        fileobj.valid_chunk_names_callback = valid_chunks_callback;
        let firmware = &fileobj.firmware_version().to_string();

        for (chunk_name, chunk) in &mut fileobj.chunks {
            chunk.set_item_callbacks(
                (fileobj.chunk_callbacks_factory)(chunk_name, firmware).unwrap_or_default(),
            );
        }
        Ok(fileobj)
    }

    /// Read the checksum from the file.
    pub fn read_checksum(&self) -> Vec<u8> {
        self.checksum.to_vec()
    }

    fn read_or_calc_checksum(&self) -> Vec<u8> {
        if self.is_dirty {
            self.calc_checksum()
        } else {
            Vec::from(self.checksum.clone())
        }
    }

    /// Remove the chunk `chunk_name` from the file.
    ///
    /// # Errors
    /// Expected errors include:
    /// - [`TD0Error::InvalidChunk`](crate::result::TD0Error::InvalidChunk)
    ///   - if `chunk_name` is not a valid chunk name.
    ///   - if the chunk size in the file metadata is invalid.
    ///
    /// # Example
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0File};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_two_chunks.TD0");
    /// let mut td0file : TD0File = parse_td0_file(TD0_BYTES)?;
    ///
    /// // Remove the`TGLa` chunk, and verify the chunk was removed.
    /// td0file.remove_chunk("TGLa")?;
    /// assert_eq!(td0file.list_chunks(), vec!["HDRa"]);
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    pub fn remove_chunk(&mut self, chunk_name: &str) -> TD0Result<()> {
        let chunk_name: &ChunkTagStr = chunk_name
            .try_into()
            .map_err(|_| TD0Error::unknown_chunk_error(chunk_name))?;

        let Some(tag) = self.tag(chunk_name) else {
            return Err(TD0Error::unknown_chunk_error(chunk_name));
        };

        if chunk_name == "HDRa" {
            // We may not remove the HDRa chunk from a file.
            return Err(TD0Error::OperationNotSupported);
        }

        // Before removing any items, calculate how much each item shifts in position and apply
        // those offsets to the existing chunks.
        let chunk_offset_changes = self.calc_remove_chunk_offsets(tag);
        for (modify_chunk_name, offset) in &chunk_offset_changes {
            self.update_chunk_offset(modify_chunk_name, *offset)?;
        }

        // Remove the tag and chunk.
        self.remove_tag(chunk_name)?;
        self.chunks
            .remove(chunk_name.as_ref())
            .expect("chunk tag has a corresponding chunk.");

        self.is_dirty = true;
        Ok(())
    }

    /// Replace the chunk `chunk_name` with the new chunk `new_chunk`.
    ///
    /// # Errors
    /// Expected errors include:
    /// - [`TD0Error::InvalidChunk`](crate::result::TD0Error::InvalidChunk)
    ///   - if `chunk_name` is not a valid chunk name.
    /// - [`TD0Error::InvalidInput`](crate::result::TD0Error::InvalidChunk)
    ///   - if `new_chunk` is a different size in bytes from the existing chunk `chunk_name`.
    ///   - if `new_chunk` does not consist of the same item type as chunk `chunk_name`.
    ///
    pub fn replace_chunk(&mut self, chunk_name: &str, new_chunk: TD0Chunk) -> TD0Result<()> {
        if !self.has_chunk(chunk_name) {
            return Err(TD0Error::unknown_chunk_error(chunk_name));
        }

        if chunk_name == "HDRa" {
            // We may not remove the HDRa chunk from a file.
            return Err(TD0Error::OperationNotSupported);
        }

        let dest_chunk = self.chunk(chunk_name).unwrap();
        if !dest_chunk.size() == new_chunk.size() {
            return Err(TD0Error::InvalidInput(
                "new_chunk size does not match destination chunk.".to_string(),
            ));
        }

        let dest_item = dest_chunk.item_at(0)?;
        let src_item = new_chunk.item_at(0)?;
        if !TD0Chunk::chunk_item_types_comparable(&*dest_item, &*src_item) {
            return Err(TD0Error::InvalidInput(
                "new_chunk item type does not match destination chunk.".to_string(),
            ));
        }

        self.chunks.remove(chunk_name);
        self.chunks.insert(chunk_name.to_string(), new_chunk);
        Ok(())
    }

    fn tag_index(&self, chunk_name: &ChunkTagStr) -> Option<usize> {
        self.tag_order.iter().position(|val| val == chunk_name)
    }

    fn remove_tag(&mut self, chunk_name: &ChunkTagStr) -> TD0Result<()> {
        let remove_tag_idx: usize = self
            .tag_index(chunk_name)
            .expect("tag_order and tags are in sync.");

        self.tags
            .remove(chunk_name)
            .ok_or_else(|| TD0Error::unknown_chunk_error(chunk_name))?;
        self.tag_order.remove(remove_tag_idx);
        let id_tag = TD0IdChunk::mut_from_bytes(&mut self.id_chunk).unwrap();
        id_tag.set_bytes_remaining(
            u16::try_from(id_tag.bytes_remaining() - size_of::<TD0ManifestTag>()).unwrap(),
        );
        Ok(())
    }

    /// Return a reference to the [`TD0ManifestTag`] for `chunk_name` or None if that
    /// chunk is not present.
    pub fn tag(&self, chunk_name: &ChunkTagStr) -> Option<&TD0ManifestTag> {
        self.tags
            .get(chunk_name)
            .map(|tag_bytes| TD0ManifestTag::ref_from_bytes(tag_bytes).unwrap())
    }

    /// Return a mutable reference to the [`TD0ManifestTag`] for `chunk_name` or None
    /// if that chunk is not present.
    pub fn tag_mut(&mut self, chunk_name: &ChunkTagStr) -> Option<&mut TD0ManifestTag> {
        self.tags
            .get_mut(chunk_name)
            .map(|tag_bytes| TD0ManifestTag::mut_from_bytes(tag_bytes).unwrap())
    }

    fn update_chunk_offset(&mut self, chunk_name: &ChunkTagStr, offset: i64) -> TD0Result<()> {
        // Update the position in the manifest.
        let tag = self
            .tag_mut(chunk_name)
            .ok_or_else(|| TD0Error::unknown_chunk_error(chunk_name))?;
        let chunk_pos: i64 = i64::try_from(tag.chunk_pos())
            .map_err(|_| TD0Error::invalid_chunk_error(tag.tag(), "chunk offset invalid."))?;

        tag.set_chunk_pos(
            usize::try_from(chunk_pos + offset).map_err(|_| TD0Error::out_of_range_error_u32())?,
        )?;

        Ok(())
    }

    /// Check that the suffix of `bytes` contains a checksum of the rest of `bytes`.
    ///
    /// # Errors
    /// - [`TD0Error::ChecksumMismatch`] if the included file checksum is incorrect.
    /// - [`TD0Error::FileParse`] if the bytes isn't large enough to contain a checksum.
    pub fn verify_checksum(bytes: &[u8]) -> TD0Result<()> {
        if bytes.len() <= SZ_MD5_DIGEST {
            return Err(TD0Error::file_parse_error(
                "not enough bytes for a checksum.",
            ));
        }

        let (source, checksum) = bytes.split_at(bytes.len() - SZ_MD5_DIGEST);
        if calc_md5_checksum(&[source]) == checksum {
            Ok(())
        } else {
            Err(TD0Error::ChecksumMismatch)
        }
    }
}

impl TryFrom<Vec<u8>> for TD0File {
    type Error = TD0Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        use zerocopy::{FromBytes, TryFromBytes};

        let file_size: usize = value.len();
        Self::verify_checksum(&value)?;
        let mut bytes: BytesMut = BytesMut::from_iter(value);

        // Parse and validate the id chunk.
        if bytes.len() < size_of::<TD0IdChunk>() {
            return Err(TD0Error::file_parse_error(ERR_TD0_CHUNK_READ));
        }
        let id_chunk_bytes: BytesMut = bytes.split_to(size_of::<TD0IdChunk>());
        let id_chunk: &TD0IdChunk = TD0IdChunk::ref_from_bytes(&id_chunk_bytes).unwrap();
        id_chunk.validate()?;

        // Parse and validate the manifest.
        let (tags, tag_order) = Self::parse_manifest(&mut bytes, id_chunk, file_size)?;

        if !tags.contains_key(&ChunkTagString::from_string_unchecked("HDRa")) {
            return Err(TD0Error::file_parse_error(ERR_BACKUP_HEADER_MISSING));
        }

        // Parse and validate the chunks.
        let chunks: HashMap<String, TD0Chunk> = Self::parse_chunks(&mut bytes, &tags, &tag_order)?;

        // Get the firmware version.
        let Some(Ok(backup_header)) = chunks
            .get("HDRa")
            .expect("parse_chunks verifies existence.")
            .item_at_raw(0)
            .map(HDRaItem::try_ref_from_bytes)
        else {
            return Err(TD0Error::file_parse_error("unable to read backup header."));
        };
        backup_header.validate()?;

        let firmware_version = Self::parse_firmware_version(backup_header)?;
        if bytes.len() < SZ_MD5_DIGEST {
            return Err(TD0Error::file_parse_error("file missing checksum."));
        } else if bytes.len() > SZ_MD5_DIGEST {
            return Err(TD0Error::file_parse_error(
                "unparsed file data not specified in manifest.",
            ));
        }

        Ok(Self {
            firmware_version,
            id_chunk: id_chunk_bytes,
            tags,
            tag_order,
            chunks,
            checksum: bytes,

            valid_chunk_names_callback: default_valid_chunks_callback,
            chunk_callbacks_factory: default_chunk_callbacks_factory,
            is_dirty: false,
        })
    }
}

impl From<TD0File> for Vec<u8> {
    fn from(value: TD0File) -> Self {
        <&TD0File>::into(&value)
    }
}

impl From<&TD0File> for Vec<u8> {
    fn from(value: &TD0File) -> Self {
        let mut dest: BytesMut = BytesMut::new();

        // Get all buffers, substituting a known good checksum for a possibly stale one.
        let checksum_buf = BytesMut::from_iter(value.read_or_calc_checksum());
        let mut bufs = value.collect_buffers();
        bufs.pop();
        bufs.push(&checksum_buf);

        for buf in bufs {
            dest.extend(buf);
        }
        Self::from(dest)
    }
}

impl<'a> TryFrom<&'a [u8]> for TD0File {
    type Error = TD0Error;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "this matches an existing callback function."
)]
fn default_chunk_callbacks_factory(
    _chunk_name: &str,
    _version: &str,
) -> Option<ChunkItemAccessorCallbacks> {
    Some(ChunkItemAccessorCallbacks::default())
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "this matches an existing callback function."
)]
const fn default_valid_chunks_callback(_version: &str) -> Option<&'static [&'static str]> {
    Some(&DEFAULT_VALID_CHUNKS)
}
