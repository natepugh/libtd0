// Data types for TD0 SSXP Setlist data.
use super::common::{ChunkItem, ChunkItemValue, ChunkItemValueRaw};
use libtd0_derive::TD0ChunkItem;
use zerocopy::{LittleEndian, U32};
use zerocopy_derive::{FromBytes, IntoBytes, KnownLayout};

#[derive(Clone, Copy, Debug, FromBytes, IntoBytes, KnownLayout, TD0ChunkItem)]
#[repr(C, packed)]
pub struct WVPaItem {
    #[td0_field(field_type = "U32")]
    start_point: U32<LittleEndian>, // start point

    #[td0_field(field_type = "U32")]
    end_point: U32<LittleEndian>, // End point

    #[td0_field(field_type = "U32")]
    loop_point: U32<LittleEndian>, // loop point

    #[td0_field(field_type = "U8", max = 127)]
    volume: u8, // Sample Volume (0 - 127)

    #[td0_field(field_type = "Slice")]
    unknown4: [u8; 3], // unknown data

    #[td0_field(field_type = "Slice")]
    unknown5: [u8; 4], // unknown data

    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    name: [u8; 16], // Name

    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    filename: [u8; 100], // Filename

    #[td0_field(field_type = "Slice")]
    unknown6: [u8; 16], // Unknown data

    #[td0_field(field_type = "Text", pad_byte = 0)]
    device_serial_number: [u8; 8], // device_serial_number

    #[td0_field(field_type = "Slice")]
    unknown7: [u8; 4], // Unknown data
}
