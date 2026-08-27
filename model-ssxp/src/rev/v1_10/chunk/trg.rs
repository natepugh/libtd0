// Data types for TD0 TRGa (trigger config?) data.
use libtd0_derive::TD0ChunkItemDerive;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Clone, Copy, Debug, FromBytes, Immutable, IntoBytes, KnownLayout, TD0ChunkItemDerive)]
#[repr(C, packed)]
pub struct TRGaItem {
    #[td0_field(field_type = "Slice")]
    unknown: [u8; 300],
}
