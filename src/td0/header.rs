use libtd0_core::usize_from_u32;
use zerocopy::{FromBytes, LittleEndian, U16, U32};
use zerocopy_derive::{Immutable, IntoBytes, KnownLayout};

pub const SZ_HDR_CHUNK: usize = 16;
pub const OFFSET_BYTES_REMAINING: usize = 2;
pub const TD0_MAGIC: [u8; 4] = *b"TD0a";

#[derive(Clone, Copy, Debug, Default, FromBytes, Immutable, IntoBytes, KnownLayout)]
#[repr(C, packed)]
pub struct TD0IdChunk {
    bytes_remaining: U16<LittleEndian>,
    magic: [u8; 4],
    unknown: [u8; 10],
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

    #[expect(dead_code)]
    pub fn magic(&self) -> String {
        String::from_utf8_lossy(&self.magic).into()
    }

    #[expect(dead_code)]
    pub const fn unknown(&self) -> &[u8; 10] {
        &self.unknown
    }
}

#[derive(Clone, Copy, Debug, FromBytes, Immutable, IntoBytes, KnownLayout)]
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
        usize::try_from(self.chunk_pos.get()).expect("platform usize is >= 32 bits")
    }

    pub fn chunk_size(&self) -> usize {
        usize::try_from(self.chunk_size.get()).expect("platform usize is >= 32 bits")
    }

    #[expect(dead_code)]
    pub fn chunk_range(&self) -> ::core::ops::Range<usize> {
        self.chunk_pos()..self.chunk_pos() + self.chunk_size()
    }
}

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
