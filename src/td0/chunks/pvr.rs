// Data types for TD0 PVRa data (unknown what this represents).
use super::common::{ChunkItem, ChunkItemValue, ChunkItemValueRaw};
use libtd0_derive::TD0ChunkItem;
use zerocopy_derive::{FromBytes, IntoBytes, KnownLayout};

#[derive(Clone, Copy, Debug, FromBytes, IntoBytes, KnownLayout, TD0ChunkItem)]
#[repr(C, packed)]
pub struct PVRaItem {
    #[td0_field(field_type = "Slice")]
    unknown: [u8; 64],
}
