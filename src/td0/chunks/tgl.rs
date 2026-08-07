use super::common::{ChunkItem, ChunkItemValue, ChunkItemValueRaw};
use libtd0_derive::TD0ChunkItem;
use zerocopy_derive::{FromBytes, IntoBytes, KnownLayout};

#[derive(Clone, Copy, Debug, FromBytes, IntoBytes, KnownLayout, TD0ChunkItem)]
#[repr(C, packed)]
pub struct TGLaItem {
    /* Represents a tag in the tag library. */
    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    name: [u8; 16],
}
