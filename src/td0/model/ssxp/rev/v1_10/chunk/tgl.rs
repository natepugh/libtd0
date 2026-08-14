use libtd0_derive::TD0ChunkItemDerive;
use zerocopy_derive::{FromBytes, IntoBytes, KnownLayout};

#[derive(Clone, Copy, Debug, FromBytes, IntoBytes, KnownLayout, TD0ChunkItemDerive)]
#[repr(C, packed)]
pub struct TGLaItem {
    /* Represents a tag in the tag library. */
    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    name: [u8; 16],
}
