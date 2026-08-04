// Data types for TD0 SSXP Setlist data.
use super::common::{ChunkItem, ChunkItemValue, ChunkItemValueRaw};
use libtd0_derive::TD0ChunkItem;
use zerocopy::{LittleEndian, U16};
use zerocopy_derive::{FromBytes, IntoBytes, KnownLayout};

#[derive(Clone, Copy, Debug, FromBytes, IntoBytes, KnownLayout, TD0ChunkItem)]
#[repr(C, packed)]
pub struct STLaItem {
    /* Setlist:
     *
     * 16-byte name
     * 32 x 8-byte Kit Selection
     */
    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    name: [u8; 16],

    #[td0_field(field_type = "U16", max = 200)]
    step_1_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_1_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_2_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_2_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_3_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_3_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_4_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_4_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_5_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_5_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_6_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_6_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_7_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_7_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_8_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_8_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_9_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_9_unknown: [u8; 6],

    #[td0_field(field_type = "U16", max = 200)]
    step_10_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_10_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_11_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_11_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_12_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_12_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_13_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_13_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_14_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_14_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_15_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_15_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_16_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_16_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_17_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_17_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_18_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_18_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_19_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_19_unknown: [u8; 6],

    #[td0_field(field_type = "U16", max = 200)]
    step_20_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_20_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_21_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_21_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_22_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_22_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_23_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_23_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_24_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_24_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_25_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_25_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_26_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_26_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_27_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_27_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_28_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_28_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_29_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_29_unknown: [u8; 6],

    #[td0_field(field_type = "U16", max = 200)]
    step_30_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_30_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_31_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_31_unknown: [u8; 6],
    #[td0_field(field_type = "U16", max = 200)]
    step_32_kit: U16<LittleEndian>,
    #[td0_field(field_type = "Slice")]
    step_32_unknown: [u8; 6],
}
