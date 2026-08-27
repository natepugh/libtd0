// Data types for TD0 SSXP Setlist data.
use libtd0_derive::TD0ChunkItemDerive;
use zerocopy::{LittleEndian, U32};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Clone, Copy, Debug, FromBytes, Immutable, IntoBytes, KnownLayout, TD0ChunkItemDerive)]
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

    /* NOTE:
       The first two bytes of this field differ between firmware versions.
       The padding character used for PRELOAD files is different from user files.

       PRELOAD files: 0x0
       USER    files: 0x20 (ASCII space: " ")

       Observed patterns:
           - 1.10 (PRELOAD) [0x20, 0x20]
           - 2.0  (PRELOAD) [0xb0, 0x04]

           - 1.10 (USER) [0x0, 0x0]
           - 2.0  (USER) [0xb0, 0x04]
    */
    #[td0_field(field_type = "Slice")]
    unknown6: [u8; 16], // Unknown data

    #[td0_field(field_type = "Text", pad_byte = 0)]
    device_serial_number: [u8; 8], // device_serial_number

    // NOTE: This field differs per each file.
    #[td0_field(field_type = "Slice")]
    unknown7: [u8; 4], // Unknown data
}
