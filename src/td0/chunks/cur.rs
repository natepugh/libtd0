// Data types for TD0 CURa data (unknown what this represents).
use zerocopy_derive::{FromBytes, IntoBytes, KnownLayout};
use super::common::{ChunkItem, ChunkItemValue, ChunkItemValueRaw};
use libtd0_derive::TD0ChunkItem;


#[derive(Clone, Copy, Debug, FromBytes, IntoBytes, KnownLayout, TD0ChunkItem)]
#[repr(C, packed)]
pub struct CURaItem {
    #[td0_field(field_type = "Slice")]
    unknown: [u8; 8],
}