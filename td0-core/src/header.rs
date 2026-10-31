// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use zerocopy::{LittleEndian, U16, U32};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

use super::helpers::{
    copy_ascii_str_to_u8_slice, text_from_u8_array, try_u32_le_from_usize, usize_from_u32,
};

use super::ascii_string::AsciiPrintableString;
use super::result::{TD0Error, TD0Result};
use td0_derive::TD0ChunkItemDerive;

pub const OFFSET_BYTES_REMAINING: usize = 2;
pub const TD0_MAGIC: [u8; 4] = *b"TD0a";

#[derive(
    Clone, Copy, PartialEq, Eq, Debug, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned,
)]
#[repr(C, packed)]
pub struct TD0IdChunk {
    bytes_remaining: U16<LittleEndian>,
    magic: [u8; 4],
    unknown: [u8; 10],
}

impl Default for TD0IdChunk {
    fn default() -> Self {
        let magic: [u8; 4] = *b"TD0a";
        Self {
            bytes_remaining: U16::from(14),
            magic,
            unknown: [0; 10],
        }
    }
}

impl core::fmt::Display for TD0IdChunk {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "TD0IdChunk: bytes_remaining: {} magic: \"{}\" unknown: {:?}",
            self.bytes_remaining,
            String::from_utf8_lossy(&self.magic),
            self.unknown
        )
    }
}

impl TD0IdChunk {
    pub fn bytes_remaining(&self) -> usize {
        usize_from_u32(self.bytes_remaining.get().into())
    }

    pub const fn magic_raw(&self) -> &[u8; 4] {
        &self.magic
    }

    pub fn magic(&self) -> String {
        String::from_utf8_lossy(&self.magic).into()
    }

    pub fn set_bytes_remaining(&mut self, remaining: u16) {
        self.bytes_remaining = U16::from(remaining);
    }

    pub fn set_magic(&mut self, magic: &[u8; 4]) {
        self.magic.copy_from_slice(magic);
    }

    pub const fn unknown(&self) -> &[u8; 10] {
        &self.unknown
    }

    pub fn try_ref_from_buf(buf: &[u8]) -> TD0Result<&Self> {
        use zerocopy::TryFromBytes;

        Self::try_ref_from_bytes(buf.get(..size_of::<Self>()).ok_or(TD0Error::FileParse(
            "unable to read TD0 ID chunk.".to_string(),
        ))?)
        .map_err(|_| TD0Error::FileParse("unable to parse TD0 ID chunk.".to_string()))
    }

    pub fn validate(&self) -> TD0Result<()> {
        if self.magic == TD0_MAGIC {
            Ok(())
        } else {
            Err(TD0Error::FileParse(
                "invalid or missing `TD0a` header chunk.".to_string(),
            ))
        }
    }
}

#[derive(
    Clone, Copy, Default, Debug, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq,
)]
#[repr(C, packed)]
pub struct TD0ManifestTag {
    tag: [u8; 4],
    model: [u8; 4],
    chunk_pos: U32<LittleEndian>,
    chunk_size: U32<LittleEndian>,
}

impl TD0ManifestTag {
    pub fn model(&self) -> String {
        String::from_utf8_lossy(&self.model).to_string()
    }

    pub fn tag(&self) -> String {
        String::from_utf8_lossy(&self.tag).to_string()
    }

    pub const fn tag_raw(&self) -> &[u8; 4] {
        &self.tag
    }

    pub fn chunk_pos(&self) -> usize {
        usize_from_u32(self.chunk_pos.get())
    }

    pub fn chunk_size(&self) -> usize {
        usize_from_u32(self.chunk_size.get())
    }

    pub fn chunk_range(&self) -> ::core::ops::Range<usize> {
        self.chunk_pos()..self.chunk_pos() + self.chunk_size()
    }

    pub fn set_model(&mut self, model: &str) -> TD0Result<()> {
        if !model.is_ascii() {
            return Err(TD0Error::InvalidInput(
                "model must be ASCII only.".to_string(),
            ));
        }

        if model.len() != 4 {
            return Err(TD0Error::InvalidInput(
                "model must be exactly 4 ASCII characters long.".to_string(),
            ));
        }

        self.model.copy_from_slice(model.as_bytes());
        Ok(())
    }

    pub fn set_tag(&mut self, tag: &str) -> TD0Result<()> {
        if !tag.is_ascii() {
            return Err(TD0Error::InvalidInput(
                "tag must be ASCII only.".to_string(),
            ));
        }

        if tag.len() != 4 {
            return Err(TD0Error::InvalidInput(
                "model must be exactly 4 ASCII characters long.".to_string(),
            ));
        }

        self.tag.copy_from_slice(tag.as_bytes());
        Ok(())
    }

    pub fn set_chunk_pos(&mut self, pos: usize) -> TD0Result<()> {
        self.chunk_pos = try_u32_le_from_usize(pos)?;
        Ok(())
    }

    pub fn set_chunk_size(&mut self, size: usize) -> TD0Result<()> {
        self.chunk_size = try_u32_le_from_usize(size)?;
        Ok(())
    }

    pub fn try_read_from_buf_at(buf: &[u8], pos: usize) -> TD0Result<Self> {
        use zerocopy::TryFromBytes;

        let bytes = buf
            .get(pos..pos + size_of::<TD0ManifestTag>())
            .ok_or(TD0Error::FileParse(
                "unable to read manifest tag.".to_string(),
            ))?;

        TD0ManifestTag::try_read_from_bytes(bytes)
            .map_err(|_| TD0Error::FileParse("invalid manifest tag.".to_string()))
    }

    pub fn try_mut_from_buf_at(buf: &mut [u8], pos: usize) -> TD0Result<&mut Self> {
        use zerocopy::TryFromBytes;

        let bytes =
            buf.get_mut(pos..pos + size_of::<TD0ManifestTag>())
                .ok_or(TD0Error::FileParse(
                    "unable to read manifest tag.".to_string(),
                ))?;

        TD0ManifestTag::try_mut_from_bytes(bytes)
            .map_err(|_| TD0Error::FileParse("invalid manifest tag.".to_string()))
    }

    pub fn validate(&self, file_size: usize) -> TD0Result<()> {
        if self.chunk_pos() + self.chunk_size() > file_size {
            return Err(TD0Error::validation_error(
                "tag describes a chunk that extends beyond the file buffer.",
            ));
        }

        for byte in &self.tag[..3] {
            if !byte.is_ascii_uppercase() {
                return Err(TD0Error::validation_error(
                    "improper tag format (bytes [0..3] must be ASCII uppercase.)",
                ));
            }
        }
        if !self.tag[3].is_ascii_lowercase() {
            return Err(TD0Error::validation_error(
                "improper tag format (byte[3] must be ASCII lowercase.)",
            ));
        }
        Ok(())
    }
}

#[doc(hidden)]
impl core::fmt::Display for TD0ManifestTag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} : model: {}  pos: {}  length: {}",
            self.tag(),
            self.model(),
            self.chunk_pos,
            self.chunk_size,
        )
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Unaligned,
    PartialEq,
    TD0ChunkItemDerive,
)]
#[repr(C, packed)]
#[td0_struct(td0_core_crate = "crate")]
pub struct HDRaItem {
    // e.g. "SSXPROBK"
    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    tag: [u8; 8],

    #[td0_field(field_type = "Slice")]
    data: [u8; 8],

    // NOTE: SSXPROBK (whole system) backups use 0x0 as a pad byte.
    //       SSXPROKT (single kit) backups use 0x20, likely copied
    //       verbatim from the kit name.
    #[td0_field(field_type = "Text", pad_byte = 0)]
    name: [u8; 16],

    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    firmware: [u8; 4],

    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    build: [u8; 4],

    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    device_serial: [u8; 8],

    #[td0_field(field_type = "Slice")]
    suffix: [u8; 16],
}

impl HDRaItem {
    const DEFAULT_BACKUP_NAME: [u8; 16] = *b"USER KIT        ";
    const DEFAULT_BUILD: [u8; 4] = *b"0000";
    const DEFAULT_DEVICE_SERIAL: [u8; 8] = *b"XXXXXXX ";
    const DEFAULT_FIRMWARE: [u8; 4] = *b"2.00";
    const DEFAULT_HEADER_DATA: [u8; 8] = [1u8, 1u8, 0, 0, 0, 0, 0, 0];
    const DEFAULT_SUFFIX: [u8; 16] = [0xFFu8; 16];
    const DEFAULT_TAG: [u8; 8] = *b"SSXPROKT";

    pub fn tag(&self) -> String {
        text_from_u8_array(&self.tag, 0)
    }

    pub fn set_tag(&mut self, tag: &str) -> TD0Result<()> {
        copy_ascii_str_to_u8_slice(tag, &mut self.tag, 0)?;
        Ok(())
    }

    pub fn name(&self) -> String {
        text_from_u8_array(&self.name, 0)
    }

    pub fn set_name(&mut self, name: &str) -> TD0Result<()> {
        copy_ascii_str_to_u8_slice(name, &mut self.name, 0)?;
        Ok(())
    }

    pub fn firmware(&self) -> String {
        text_from_u8_array(&self.firmware, b' ')
    }

    pub fn set_firmware(&mut self, firmware: &str) -> TD0Result<()> {
        copy_ascii_str_to_u8_slice(firmware, &mut self.firmware, b' ')?;
        Ok(())
    }

    pub fn build(&self) -> String {
        text_from_u8_array(&self.build, b' ')
    }

    pub fn set_build(&mut self, build: &str) -> TD0Result<()> {
        copy_ascii_str_to_u8_slice(build, &mut self.build, b' ')?;
        Ok(())
    }

    pub fn device_serial(&self) -> String {
        text_from_u8_array(&self.device_serial, b' ')
    }

    pub fn set_device_serial(&mut self, device_serial: &str) -> TD0Result<()> {
        copy_ascii_str_to_u8_slice(device_serial, &mut self.device_serial, b' ')?;
        Ok(())
    }

    pub fn default_usable() -> Self {
        Self {
            tag: Self::DEFAULT_TAG,
            data: Self::DEFAULT_HEADER_DATA,
            name: Self::DEFAULT_BACKUP_NAME,
            firmware: Self::DEFAULT_FIRMWARE,
            build: Self::DEFAULT_BUILD,
            device_serial: Self::DEFAULT_DEVICE_SERIAL,
            suffix: Self::DEFAULT_SUFFIX,
        }
    }

    pub fn validate(&self) -> TD0Result<()> {
        let tag = self.tag();
        if AsciiPrintableString::try_from(tag.clone()).is_err() {
            return Err(TD0Error::validation_error("invalid backup header tag."));
        }
        for char in tag.chars() {
            if !(char.is_ascii_uppercase() || char == ' ' || char == '\0') {
                return Err(TD0Error::validation_error("invalid backup header tag."));
            }
        }

        let name = self.name();
        if AsciiPrintableString::try_from(name.clone()).is_err() {
            return Err(TD0Error::validation_error("invalid backup header name."));
        }

        let firmware = self.firmware();
        if AsciiPrintableString::try_from(firmware.clone()).is_err() {
            return Err(TD0Error::validation_error(
                "invalid backup header firmware.",
            ));
        }
        for char in firmware.chars() {
            if !(char.is_ascii_digit() || char == '.' || char == ' ') {
                return Err(TD0Error::validation_error(
                    "invalid backup header firmware.",
                ));
            }
        }

        let build = self.build();
        if AsciiPrintableString::try_from(build.clone()).is_err() {
            return Err(TD0Error::validation_error("invalid backup header build."));
        }
        for char in build.chars() {
            if !(char.is_ascii_digit()) {
                return Err(TD0Error::validation_error("invalid backup header build."));
            }
        }

        let device_serial = self.device_serial();
        if AsciiPrintableString::try_from(device_serial.clone()).is_err() {
            return Err(TD0Error::validation_error(
                "invalid backup header device_serial.",
            ));
        }
        for char in device_serial.chars() {
            if !(char.is_ascii_digit() || char.is_ascii_uppercase()) {
                return Err(TD0Error::validation_error(
                    "invalid backup header device_serial.",
                ));
            }
        }

        Ok(())
    }
}
