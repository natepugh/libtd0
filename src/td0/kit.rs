use crate::td0::chunk::{ChunkItem, ChunkItemValue, ChunkItemValueRaw, IntEncodedDecimal, Volume};
use crate::td0::result::{TD0Error, TD0Result};
use libtd0_derive::{TD0ChunkItem, repeat_fields};
use zerocopy::{I16, LittleEndian, U16};
use zerocopy_derive::{FromBytes, IntoBytes, KnownLayout};

use super::strings::{
    DIRECT_ROUTES, EXP_PARAMS, EXP_PEDAL_MODES, LED_COLORS, LED_MODES, LOOP_MODES,
    PAD_EDIT_KNOB_GROUPS, PADS, SEQUENCE_OPTS, SIDECHAIN_ROUTES, SYS_KIT_SWITCH,
};

//  Kit LED order in backup files:
//      1 - 9) Row divider LEDs, L to R, top to bottom
//      10 - 15) Column divider LEDs, L to R, top to bottom
//
//     +--------+--------+--------+
//     |        |        |        |
//     |        1        1        |
//     |        0        1        |
//     |        |        |        |
//     |-led_01-+-led_02-+-led_03-+
//     |        |        |        |
//     |        1        1        |
//     |        2        3        |
//     |        |        |        |
//     |-led_04-+-led_05-+-led_06-+
//     |        |        |        |
//     |        1        1        |
//     |        4        5        |
//     |        |        |        |
//     +-led_07-+-led_08-+-led_09-+
//

#[repeat_fields]
#[derive(Clone, Copy, Debug, FromBytes, IntoBytes, KnownLayout, TD0ChunkItem)]
#[repr(C, packed)]
pub struct KITaItem {
    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    name: [u8; 16],

    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    memo: [u8; 64],

    #[td0_field(field_type = "Volume")]
    volume: I16<LittleEndian>,

    #[td0_field(field_type = "EnumStr", collection = "SYS_KIT_SWITCH")]
    click_setting: u8,

    #[td0_field(field_type = "U8")]
    unknown_1: u8,

    #[td0_field(field_type = "TD0Decimal", min = 20.0, max = 260.0)]
    tempo: U16<LittleEndian>,

    #[td0_field(field_type = "Slice")]
    unknown_2: [u8; 3],

    #[repeat(count = 4, format = "mfx_{}_routing")]
    mfx_1_routing: u8,

    #[repeat(count = 4, format = "mfx_{}_direct_out_routing")]
    mfx_1_direct_out_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "SIDECHAIN_ROUTES")]
    sidechain_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    sidechain_direct_out_routing: u8,

    #[td0_field(field_type = "Slice")]
    unknown_3: [u8; 3],

    #[td0_field(field_type = "EnumStr", collection = "PADS")]
    pad_sequence_pad: u8,

    // Pad sequence steps (1 - 16): 0 - 19 = pads 1 - 9, triggers 1 - 8, ftswitches 1 - 2, 20 = SKIP
    #[repeat(count = 16, format = "pad_sequence_{}")]
    #[td0_field(field_type = "EnumStr", collection = "SEQUENCE_OPTS")]
    pad_sequence_: u8,

    #[td0_field(field_type = "Slice")]
    unknown_4: [u8; 21],

    // Pad 1 - 9 LED Mode
    #[repeat(count = 9, format = "pad_{}_led_mode")]
    #[td0_field(field_type = "EnumStr", collection = "LED_MODES")]
    pad_1_led_mode: u8,

    #[td0_field(field_type = "Slice")]
    unknown_5: [u8; 9],

    // LED 1 - 15
    #[repeat(count = 15, format = "led_{}")]
    #[td0_field(field_type = "EnumStr", collection = "LED_COLORS")]
    led_1: u8,

    #[td0_field(field_type = "Slice")]
    unknown_6: [u8; 3],

    #[td0_field(field_type = "EnumStr", collection = "SYS_KIT_SWITCH")]
    pad_edit_knob_mode: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EDIT_KNOB_GROUPS")]
    pad_edit_knob_1_group: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EDIT_KNOB_GROUPS")]
    pad_edit_knob_2_group: u8,

    #[td0_field(field_type = "Slice")]
    unknown_7: [u8; 2],

    #[td0_field(field_type = "EnumStr", collection = "EXP_PARAMS")]
    pad_edit_knob_1_param: u8,

    #[td0_field(field_type = "EnumStr", collection = "EXP_PARAMS")]
    pad_edit_knob_2_param: u8,

    #[td0_field(field_type = "Slice")]
    unknown_8: [u8; 6],

    #[td0_field(field_type = "EnumStr", collection = "EXP_PARAMS")]
    exp_pedal_assign: u8,

    #[td0_field(field_type = "EnumStr", collection = "EXP_PEDAL_MODES")]
    exp_pedal_mode: u8,

    #[td0_field(field_type = "Slice")]
    unknown_9: [u8; 13],
}
