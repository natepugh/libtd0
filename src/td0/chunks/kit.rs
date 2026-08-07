use super::common::{ChunkItem, ChunkItemValue, ChunkItemValueRaw, IntEncodedDecimal, Volume};
use libtd0_derive::{TD0ChunkItem, repeat_fields};
use zerocopy::{I16, LittleEndian, U16};
use zerocopy_derive::{FromBytes, IntoBytes, KnownLayout};

use crate::td0::strings::{
    CLICK_MODES, CLICK_SOUNDS, DELAYSYNC_VALUES, DIRECT_ROUTES, EQ_HI_FREQS, EQ_LOW_FREQS,
    EQ_MID_FREQS, EQ_Q_VALS, EXP_PARAMS, EXP_PEDAL_MIDI_CHANNELS, EXP_PEDAL_MODES, LED_COLORS,
    LED_MODES, LOOP_MODES, MFX, MFX_ROUTES, MIDI_CHANNELS, OFF_ON, OUTPUT_ROUTES,
    PAD_DYNAMICS_CURVES, PAD_EDIT_KNOB_GROUPS, PAD_EXT_CTRL_SETTINGS, PAD_LAYER_TYPES,
    PAD_MFX_ROUTES, PAD_OUTPUT_ROUTES, PAD_TRIGGER_TYPES, PADS, POLY_MODES, SEQUENCE_OPTS,
    SIDECHAIN_ROUTES, SYS_KIT_SWITCH, TRANS_SENS, TRIGGER_TYPES,
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

    /* General Kit Settings */
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
    #[td0_field(field_type = "EnumStr", collection = "MFX_ROUTES")]
    mfx_1_routing: u8,

    #[repeat(count = 4, format = "mfx_{}_direct_out_routing")]
    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
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
    pad_sequence_1: u8,

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

    /* Click Settings */
    #[td0_field(field_type = "EnumStr", collection = "CLICK_MODES")]
    click_mode: u8, // Click Mode

    #[td0_field(field_type = "EnumStr", collection = "CLICK_SOUNDS")]
    click_internal_sound: u8, // Click Sound

    #[td0_field(field_type = "Slice")]
    click_unknown_1: [u8; 2], // Unknown 10

    #[td0_field(field_type = "U16", max = 20000)]
    click_sample_slot: U16<LittleEndian>, // Click wav slot select

    #[td0_field(field_type = "Volume")]
    click_volume: I16<LittleEndian>, // Click master volume

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    click_pan: i8, // Click pan: -15 = 100% L, 15 = 100% R.  Only int vals, -15 .. 0 .. 15

    #[td0_field(field_type = "U8")]
    click_unknown_2: u8, // Unknown 11

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    click_led_reference_enable: u8, // Click LED reference: 00 = off, 01 = on

    #[td0_field(field_type = "EnumStr", collection = "PADS")]
    click_pad_range_start: u8, // Click pad range start: 0 - 9 pads, 0xA - 0x11 Trig1 - 8,  0x12 - 0x13 = ft switch 1, 2

    #[td0_field(field_type = "EnumStr", collection = "PAD_TRIGGER_TYPES")]
    click_pad_trigger_type: u8, // Click pad trigger type: 0 == One-Time, 1 == Retrigger, 2 == Alternate

    #[td0_field(field_type = "U8", min = 1, max = 9)]
    click_beats: u8, // Click beats

    // Click volumes are 0 - 127
    #[td0_field(field_type = "U8", max = 127)]
    click_volume_accent: u8, // Click Accent volume

    #[td0_field(field_type = "U8", max = 127)]
    click_volume_quarter_note: u8, // Click quarter note volume

    #[td0_field(field_type = "U8", max = 127)]
    click_volume_8th_note: u8, // Click 8th note volume

    #[td0_field(field_type = "U8", max = 127)]
    click_volume_triplet: u8, // Click triplet volume

    #[td0_field(field_type = "U8", max = 127)]
    click_volume_16th_note: u8, // Click 16th note volume

    #[td0_field(field_type = "EnumStr", collection = "PADS")]
    click_pad_range_end: u8, // Click pad range end

    #[td0_field(field_type = "Slice")]
    click_unknown_3: [u8; 2], // Unknown 12

    #[td0_field(field_type = "EnumStr", collection = "OUTPUT_ROUTES")]
    click_output_routing: u8, // Click output_routing: 0 = Master + Phones, 1 = Phones only

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    click_direct_out: u8, // Click direct out routing: 0 - 9 Off, Direct 1, 2, 1+2, 3, 4, 3+4, Master DOUT L, R, L+R

    #[td0_field(field_type = "Slice")]
    misc_unknown_1: [u8; 4],

    #[repeat(count = 9, format = "pad_{}_midi_note")]
    #[td0_field(field_type = "U16", max = 128)]
    pad_1_midi_note: U16<LittleEndian>,

    #[repeat(count = 8, format = "trig_{}_midi_note")]
    #[td0_field(field_type = "U16", max = 128)]
    trig_1_midi_note: U16<LittleEndian>,

    #[repeat(count = 2, format = "foot_sw_{}_midi_note")]
    #[td0_field(field_type = "U16", max = 128)]
    foot_sw_1_midi_note: U16<LittleEndian>,

    #[repeat(count = 9, format = "pad_{}_midi_gate_time")]
    #[td0_field(field_type = "TD0Decimal", min = 0.1, max = 8.0)]
    pad_1_midi_gate_time: u8,

    #[repeat(count = 8, format = "trig_{}_midi_gate_time")]
    #[td0_field(field_type = "TD0Decimal", min = 0.1, max = 8.0)]
    trig_1_midi_gate_time: u8,

    #[repeat(count = 2, format = "foot_sw_{}_midi_gate_time")]
    #[td0_field(field_type = "TD0Decimal", min = 0.1, max = 8.0)]
    foot_sw_1_midi_gate_time: u8,

    #[repeat(count = 9, format = "pad_{}_midi_channel")]
    #[td0_field(field_type = "EnumStr", collection = "MIDI_CHANNELS")]
    pad_1_midi_channel: u8,

    #[repeat(count = 8, format = "trig_{}_midi_channel")]
    #[td0_field(field_type = "EnumStr", collection = "MIDI_CHANNELS")]
    trig_1_midi_channel: u8,

    #[repeat(count = 2, format = "foot_sw_{}_midi_channel")]
    #[td0_field(field_type = "EnumStr", collection = "MIDI_CHANNELS")]
    foot_sw_1_midi_channel: u8,

    #[td0_field(field_type = "Slice")]
    misc_unknown_2: [u8; 2],

    #[td0_field(field_type = "EnumStr", collection = "EXP_PEDAL_MIDI_CHANNELS")]
    exp_pedal_midi_channel: u8,

    #[td0_field(field_type = "U8")]
    exp_pedal_unknown_1: u8,

    #[td0_field(field_type = "U16", max = 128)]
    closed_pedal_midi_note: U16<LittleEndian>,

    #[td0_field(field_type = "Slice")]
    misc_unknown_3: [u8; 18],

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    sidechain_enable: u8,

    #[td0_field(field_type = "Slice")]
    misc_unknown_4: [u8; 23],

    /* MFX Settings */
    #[td0_field(field_type = "EnumStr", collection = "MFX")]
    mfx_1_effect_select: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    mfx_1_enable: u8,

    #[td0_field(field_type = "Volume")]
    mfx_1_volume: I16<LittleEndian>,

    #[td0_field(field_type = "Slice")]
    mfx_1_unknown: [u8; 4],

    #[repeat(count = 32, format = "mfx_1_param_{}")]
    #[td0_field(field_type = "U16")]
    mfx_1_param_1: U16<LittleEndian>,

    #[td0_field(field_type = "EnumStr", collection = "MFX")]
    mfx_2_effect_select: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    mfx_2_enable: u8,

    #[td0_field(field_type = "Volume")]
    mfx_2_volume: I16<LittleEndian>,

    #[td0_field(field_type = "Slice")]
    mfx_2_unknown: [u8; 4],

    #[repeat(count = 32, format = "mfx_2_param_{}")]
    #[td0_field(field_type = "U16")]
    mfx_2_param_1: U16<LittleEndian>,

    #[td0_field(field_type = "EnumStr", collection = "MFX")]
    mfx_3_effect_select: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    mfx_3_enable: u8,

    #[td0_field(field_type = "Volume")]
    mfx_3_volume: I16<LittleEndian>,

    #[td0_field(field_type = "Slice")]
    mfx_3_unknown: [u8; 4],

    #[repeat(count = 32, format = "mfx_3_param_{}")]
    #[td0_field(field_type = "U16")]
    mfx_3_param_1: U16<LittleEndian>,

    #[td0_field(field_type = "EnumStr", collection = "MFX")]
    mfx_4_effect_select: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    mfx_4_enable: u8,

    #[td0_field(field_type = "Volume")]
    mfx_4_volume: I16<LittleEndian>,

    #[td0_field(field_type = "Slice")]
    mfx_4_unknown: [u8; 4],

    #[repeat(count = 32, format = "mfx_4_param_{}")]
    #[td0_field(field_type = "U16")]
    mfx_4_param_1: U16<LittleEndian>,

    /* Pad Settings */
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    pad_1_layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_1_fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_1_fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_1_dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    pad_1_dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_1_fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    pad_1_ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_1_hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_1_hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_1_hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_1_mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_1_mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_1_link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_1_link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    pad_1_output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    pad_1_direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    pad_1_mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_1_sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_1_exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_1_trigger_reserve: u8,

    #[td0_field(field_type = "Slice")]
    pad_1_unknown: [u8; 8],

    //
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    pad_2_layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_2_fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_2_fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_2_dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    pad_2_dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_2_fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    pad_2_ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_2_hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_2_hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_2_hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_2_mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_2_mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_2_link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_2_link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    pad_2_output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    pad_2_direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    pad_2_mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_2_sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_2_exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_2_trigger_reserve: u8,

    #[td0_field(field_type = "Slice")]
    pad_2_unknown: [u8; 8],

    //
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    pad_3_layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_3_fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_3_fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_3_dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    pad_3_dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_3_fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    pad_3_ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_3_hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_3_hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_3_hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_3_mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_3_mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_3_link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_3_link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    pad_3_output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    pad_3_direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    pad_3_mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_3_sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_3_exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_3_trigger_reserve: u8,

    #[td0_field(field_type = "Slice")]
    pad_3_unknown: [u8; 8],

    //
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    pad_4_layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_4_fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_4_fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_4_dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    pad_4_dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_4_fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    pad_4_ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_4_hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_4_hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_4_hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_4_mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_4_mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_4_link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_4_link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    pad_4_output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    pad_4_direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    pad_4_mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_4_sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_4_exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_4_trigger_reserve: u8,

    #[td0_field(field_type = "Slice")]
    pad_4_unknown: [u8; 8],

    //
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    pad_5_layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_5_fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_5_fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_5_dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    pad_5_dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_5_fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    pad_5_ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_5_hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_5_hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_5_hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_5_mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_5_mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_5_link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_5_link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    pad_5_output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    pad_5_direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    pad_5_mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_5_sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_5_exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_5_trigger_reserve: u8,

    #[td0_field(field_type = "Slice")]
    pad_5_unknown: [u8; 8],

    //
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    pad_6_layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_6_fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_6_fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_6_dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    pad_6_dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_6_fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    pad_6_ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_6_hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_6_hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_6_hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_6_mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_6_mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_6_link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_6_link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    pad_6_output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    pad_6_direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    pad_6_mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_6_sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_6_exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_6_trigger_reserve: u8,

    #[td0_field(field_type = "Slice")]
    pad_6_unknown: [u8; 8],

    //
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    pad_7_layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_7_fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_7_fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_7_dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    pad_7_dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_7_fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    pad_7_ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_7_hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_7_hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_7_hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_7_mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_7_mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_7_link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_7_link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    pad_7_output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    pad_7_direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    pad_7_mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_7_sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_7_exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_7_trigger_reserve: u8,

    #[td0_field(field_type = "Slice")]
    pad_7_unknown: [u8; 8],

    //
    //
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    pad_8_layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_8_fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_8_fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_8_dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    pad_8_dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_8_fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    pad_8_ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_8_hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_8_hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_8_hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_8_mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_8_mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_8_link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_8_link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    pad_8_output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    pad_8_direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    pad_8_mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_8_sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_8_exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_8_trigger_reserve: u8,

    #[td0_field(field_type = "Slice")]
    pad_8_unknown: [u8; 8],

    //
    //
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    pad_9_layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_9_fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_9_fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_9_dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    pad_9_dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_9_fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    pad_9_ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_9_hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_9_hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    pad_9_hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_9_mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_9_mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_9_link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    pad_9_link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    pad_9_output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    pad_9_direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    pad_9_mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_9_sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_9_exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_9_trigger_reserve: u8,

    #[td0_field(field_type = "Slice")]
    pad_9_unknown: [u8; 8],

    //
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    trig_1_layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_1_fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_1_fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_1_dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    trig_1_dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_1_fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    trig_1_ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_1_hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_1_hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_1_hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_1_mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_1_mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_1_link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_1_link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    trig_1_output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    trig_1_direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    trig_1_mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_1_sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_1_exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_1_trigger_reserve: u8,

    #[td0_field(field_type = "Slice")]
    trig_1_unknown: [u8; 8],

    //
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    trig_2_layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_2_fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_2_fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_2_dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    trig_2_dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_2_fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    trig_2_ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_2_hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_2_hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_2_hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_2_mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_2_mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_2_link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_2_link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    trig_2_output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    trig_2_direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    trig_2_mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_2_sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_2_exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_2_trigger_reserve: u8,

    #[td0_field(field_type = "Slice")]
    trig_2_unknown: [u8; 8],

    //
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    trig_3_layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_3_fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_3_fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_3_dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    trig_3_dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_3_fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    trig_3_ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_3_hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_3_hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_3_hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_3_mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_3_mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_3_link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_3_link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    trig_3_output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    trig_3_direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    trig_3_mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_3_sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_3_exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_3_trigger_reserve: u8,

    #[td0_field(field_type = "Slice")]
    trig_3_unknown: [u8; 8],

    //
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    trig_4_layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_4_fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_4_fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_4_dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    trig_4_dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_4_fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    trig_4_ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_4_hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_4_hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_4_hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_4_mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_4_mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_4_link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_4_link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    trig_4_output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    trig_4_direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    trig_4_mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_4_sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_4_exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_4_trigger_reserve: u8,

    #[td0_field(field_type = "Slice")]
    trig_4_unknown: [u8; 8],

    //
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    trig_5_layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_5_fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_5_fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_5_dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    trig_5_dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_5_fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    trig_5_ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_5_hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_5_hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_5_hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_5_mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_5_mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_5_link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_5_link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    trig_5_output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    trig_5_direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    trig_5_mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_5_sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_5_exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_5_trigger_reserve: u8,

    #[td0_field(field_type = "Slice")]
    trig_5_unknown: [u8; 8],

    //
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    trig_6_layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_6_fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_6_fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_6_dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    trig_6_dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_6_fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    trig_6_ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_6_hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_6_hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_6_hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_6_mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_6_mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_6_link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_6_link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    trig_6_output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    trig_6_direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    trig_6_mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_6_sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_6_exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_6_trigger_reserve: u8,

    #[td0_field(field_type = "Slice")]
    trig_6_unknown: [u8; 8],

    //
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    trig_7_layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_7_fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_7_fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_7_dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    trig_7_dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_7_fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    trig_7_ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_7_hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_7_hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_7_hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_7_mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_7_mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_7_link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_7_link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    trig_7_output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    trig_7_direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    trig_7_mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_7_sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_7_exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_7_trigger_reserve: u8,

    #[td0_field(field_type = "Slice")]
    trig_7_unknown: [u8; 8],

    //
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    trig_8_layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_8_fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_8_fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_8_dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    trig_8_dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_8_fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    trig_8_ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_8_hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_8_hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    trig_8_hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_8_mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_8_mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_8_link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    trig_8_link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    trig_8_output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    trig_8_direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    trig_8_mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_8_sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_8_exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_8_trigger_reserve: u8,

    #[td0_field(field_type = "Slice")]
    trig_8_unknown: [u8; 8],

    //
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    foot_sw_1_layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    foot_sw_1_fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    foot_sw_1_fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_1_dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    foot_sw_1_dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    foot_sw_1_fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    foot_sw_1_ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    foot_sw_1_hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    foot_sw_1_hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    foot_sw_1_hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    foot_sw_1_mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    foot_sw_1_mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    foot_sw_1_link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    foot_sw_1_link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    foot_sw_1_output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    foot_sw_1_direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    foot_sw_1_mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_1_sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_1_exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_1_trigger_reserve: u8,

    #[td0_field(field_type = "Slice")]
    foot_sw_1_unknown: [u8; 8],

    //
    //
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    foot_sw_2_layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    foot_sw_2_fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    foot_sw_2_fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_2_dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    foot_sw_2_dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    foot_sw_2_fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    foot_sw_2_ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    foot_sw_2_hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    foot_sw_2_hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    foot_sw_2_hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    foot_sw_2_mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    foot_sw_2_mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    foot_sw_2_link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    foot_sw_2_link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    foot_sw_2_output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    foot_sw_2_direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    foot_sw_2_mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_2_sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_2_exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_2_trigger_reserve: u8,

    #[td0_field(field_type = "Slice")]
    foot_sw_2_unknown: [u8; 8],

    /* Pad Layer Settings */
    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_1_layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_1_layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    pad_1_layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    pad_1_layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    pad_1_layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    pad_1_layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_1_layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    pad_1_layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_1_layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    pad_1_layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    pad_1_layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    pad_1_layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    pad_1_layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    pad_1_layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    pad_1_layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    pad_1_layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    pad_1_layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    pad_1_layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_1_layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_1_layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    pad_1_layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    pad_1_layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_1_layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_1_layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_1_layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    pad_1_layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_1_layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_1_layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_1_layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    pad_1_layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_1_layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_1_layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    pad_1_layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    pad_1_layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_1_layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_1_layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_1_layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_1_layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_1_layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    pad_1_layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    pad_1_layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    pad_1_layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_1_layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_1_layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    pad_1_layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    pad_1_layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    pad_1_layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    pad_1_layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_1_layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    pad_1_layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_1_layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    pad_1_layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    pad_1_layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    pad_1_layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    pad_1_layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    pad_1_layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    pad_1_layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    pad_1_layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    pad_1_layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    pad_1_layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_1_layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_1_layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    pad_1_layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    pad_1_layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_1_layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_1_layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_1_layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    pad_1_layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_1_layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_1_layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_1_layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    pad_1_layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_1_layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_1_layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    pad_1_layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    pad_1_layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_1_layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_1_layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_1_layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_1_layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_1_layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    pad_1_layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    pad_1_layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    pad_1_layer_2_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_2_layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_2_layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    pad_2_layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    pad_2_layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    pad_2_layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    pad_2_layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_2_layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    pad_2_layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_2_layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    pad_2_layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    pad_2_layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    pad_2_layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    pad_2_layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    pad_2_layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    pad_2_layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    pad_2_layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    pad_2_layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    pad_2_layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_2_layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_2_layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    pad_2_layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    pad_2_layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_2_layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_2_layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_2_layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    pad_2_layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_2_layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_2_layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_2_layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    pad_2_layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_2_layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_2_layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    pad_2_layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    pad_2_layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_2_layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_2_layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_2_layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_2_layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_2_layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    pad_2_layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    pad_2_layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    pad_2_layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_2_layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_2_layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    pad_2_layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    pad_2_layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    pad_2_layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    pad_2_layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_2_layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    pad_2_layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_2_layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    pad_2_layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    pad_2_layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    pad_2_layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    pad_2_layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    pad_2_layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    pad_2_layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    pad_2_layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    pad_2_layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    pad_2_layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_2_layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_2_layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    pad_2_layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    pad_2_layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_2_layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_2_layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_2_layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    pad_2_layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_2_layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_2_layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_2_layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    pad_2_layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_2_layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_2_layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    pad_2_layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    pad_2_layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_2_layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_2_layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_2_layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_2_layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_2_layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    pad_2_layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    pad_2_layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    pad_2_layer_2_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_3_layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_3_layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    pad_3_layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    pad_3_layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    pad_3_layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    pad_3_layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_3_layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    pad_3_layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_3_layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    pad_3_layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    pad_3_layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    pad_3_layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    pad_3_layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    pad_3_layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    pad_3_layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    pad_3_layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    pad_3_layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    pad_3_layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_3_layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_3_layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    pad_3_layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    pad_3_layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_3_layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_3_layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_3_layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    pad_3_layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_3_layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_3_layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_3_layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    pad_3_layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_3_layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_3_layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    pad_3_layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    pad_3_layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_3_layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_3_layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_3_layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_3_layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_3_layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    pad_3_layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    pad_3_layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    pad_3_layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_3_layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_3_layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    pad_3_layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    pad_3_layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    pad_3_layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    pad_3_layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_3_layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    pad_3_layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_3_layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    pad_3_layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    pad_3_layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    pad_3_layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    pad_3_layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    pad_3_layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    pad_3_layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    pad_3_layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    pad_3_layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    pad_3_layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_3_layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_3_layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    pad_3_layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    pad_3_layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_3_layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_3_layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_3_layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    pad_3_layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_3_layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_3_layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_3_layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    pad_3_layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_3_layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_3_layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    pad_3_layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    pad_3_layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_3_layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_3_layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_3_layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_3_layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_3_layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    pad_3_layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    pad_3_layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    pad_3_layer_2_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_4_layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_4_layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    pad_4_layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    pad_4_layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    pad_4_layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    pad_4_layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_4_layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    pad_4_layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_4_layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    pad_4_layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    pad_4_layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    pad_4_layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    pad_4_layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    pad_4_layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    pad_4_layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    pad_4_layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    pad_4_layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    pad_4_layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_4_layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_4_layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    pad_4_layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    pad_4_layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_4_layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_4_layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_4_layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    pad_4_layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_4_layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_4_layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_4_layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    pad_4_layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_4_layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_4_layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    pad_4_layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    pad_4_layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_4_layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_4_layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_4_layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_4_layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_4_layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    pad_4_layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    pad_4_layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    pad_4_layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_4_layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_4_layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    pad_4_layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    pad_4_layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    pad_4_layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    pad_4_layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_4_layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    pad_4_layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_4_layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    pad_4_layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    pad_4_layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    pad_4_layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    pad_4_layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    pad_4_layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    pad_4_layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    pad_4_layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    pad_4_layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    pad_4_layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_4_layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_4_layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    pad_4_layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    pad_4_layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_4_layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_4_layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_4_layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    pad_4_layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_4_layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_4_layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_4_layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    pad_4_layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_4_layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_4_layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    pad_4_layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    pad_4_layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_4_layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_4_layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_4_layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_4_layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_4_layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    pad_4_layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    pad_4_layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    pad_4_layer_2_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_5_layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_5_layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    pad_5_layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    pad_5_layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    pad_5_layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    pad_5_layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_5_layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    pad_5_layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_5_layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    pad_5_layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    pad_5_layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    pad_5_layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    pad_5_layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    pad_5_layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    pad_5_layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    pad_5_layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    pad_5_layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    pad_5_layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_5_layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_5_layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    pad_5_layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    pad_5_layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_5_layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_5_layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_5_layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    pad_5_layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_5_layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_5_layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_5_layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    pad_5_layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_5_layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_5_layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    pad_5_layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    pad_5_layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_5_layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_5_layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_5_layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_5_layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_5_layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    pad_5_layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    pad_5_layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    pad_5_layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_5_layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_5_layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    pad_5_layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    pad_5_layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    pad_5_layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    pad_5_layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_5_layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    pad_5_layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_5_layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    pad_5_layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    pad_5_layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    pad_5_layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    pad_5_layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    pad_5_layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    pad_5_layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    pad_5_layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    pad_5_layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    pad_5_layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_5_layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_5_layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    pad_5_layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    pad_5_layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_5_layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_5_layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_5_layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    pad_5_layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_5_layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_5_layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_5_layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    pad_5_layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_5_layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_5_layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    pad_5_layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    pad_5_layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_5_layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_5_layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_5_layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_5_layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_5_layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    pad_5_layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    pad_5_layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    pad_5_layer_2_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_6_layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_6_layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    pad_6_layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    pad_6_layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    pad_6_layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    pad_6_layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_6_layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    pad_6_layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_6_layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    pad_6_layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    pad_6_layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    pad_6_layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    pad_6_layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    pad_6_layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    pad_6_layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    pad_6_layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    pad_6_layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    pad_6_layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_6_layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_6_layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    pad_6_layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    pad_6_layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_6_layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_6_layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_6_layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    pad_6_layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_6_layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_6_layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_6_layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    pad_6_layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_6_layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_6_layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    pad_6_layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    pad_6_layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_6_layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_6_layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_6_layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_6_layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_6_layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    pad_6_layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    pad_6_layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    pad_6_layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_6_layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_6_layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    pad_6_layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    pad_6_layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    pad_6_layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    pad_6_layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_6_layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    pad_6_layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_6_layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    pad_6_layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    pad_6_layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    pad_6_layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    pad_6_layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    pad_6_layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    pad_6_layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    pad_6_layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    pad_6_layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    pad_6_layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_6_layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_6_layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    pad_6_layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    pad_6_layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_6_layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_6_layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_6_layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    pad_6_layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_6_layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_6_layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_6_layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    pad_6_layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_6_layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_6_layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    pad_6_layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    pad_6_layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_6_layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_6_layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_6_layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_6_layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_6_layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    pad_6_layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    pad_6_layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    pad_6_layer_2_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_7_layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_7_layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    pad_7_layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    pad_7_layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    pad_7_layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    pad_7_layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_7_layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    pad_7_layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_7_layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    pad_7_layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    pad_7_layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    pad_7_layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    pad_7_layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    pad_7_layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    pad_7_layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    pad_7_layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    pad_7_layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    pad_7_layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_7_layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_7_layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    pad_7_layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    pad_7_layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_7_layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_7_layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_7_layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    pad_7_layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_7_layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_7_layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_7_layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    pad_7_layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_7_layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_7_layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    pad_7_layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    pad_7_layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_7_layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_7_layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_7_layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_7_layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_7_layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    pad_7_layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    pad_7_layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    pad_7_layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_7_layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_7_layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    pad_7_layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    pad_7_layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    pad_7_layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    pad_7_layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_7_layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    pad_7_layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_7_layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    pad_7_layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    pad_7_layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    pad_7_layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    pad_7_layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    pad_7_layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    pad_7_layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    pad_7_layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    pad_7_layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    pad_7_layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_7_layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_7_layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    pad_7_layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    pad_7_layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_7_layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_7_layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_7_layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    pad_7_layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_7_layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_7_layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_7_layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    pad_7_layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_7_layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_7_layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    pad_7_layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    pad_7_layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_7_layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_7_layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_7_layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_7_layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_7_layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    pad_7_layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    pad_7_layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    pad_7_layer_2_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_8_layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_8_layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    pad_8_layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    pad_8_layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    pad_8_layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    pad_8_layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_8_layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    pad_8_layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_8_layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    pad_8_layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    pad_8_layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    pad_8_layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    pad_8_layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    pad_8_layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    pad_8_layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    pad_8_layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    pad_8_layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    pad_8_layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_8_layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_8_layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    pad_8_layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    pad_8_layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_8_layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_8_layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_8_layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    pad_8_layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_8_layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_8_layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_8_layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    pad_8_layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_8_layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_8_layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    pad_8_layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    pad_8_layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_8_layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_8_layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_8_layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_8_layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_8_layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    pad_8_layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    pad_8_layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    pad_8_layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_8_layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_8_layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    pad_8_layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    pad_8_layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    pad_8_layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    pad_8_layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_8_layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    pad_8_layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_8_layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    pad_8_layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    pad_8_layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    pad_8_layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    pad_8_layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    pad_8_layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    pad_8_layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    pad_8_layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    pad_8_layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    pad_8_layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_8_layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_8_layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    pad_8_layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    pad_8_layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_8_layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_8_layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_8_layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    pad_8_layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_8_layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_8_layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_8_layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    pad_8_layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_8_layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_8_layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    pad_8_layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    pad_8_layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_8_layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_8_layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_8_layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_8_layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_8_layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    pad_8_layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    pad_8_layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    pad_8_layer_2_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_9_layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_9_layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    pad_9_layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    pad_9_layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    pad_9_layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    pad_9_layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_9_layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    pad_9_layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_9_layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    pad_9_layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    pad_9_layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    pad_9_layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    pad_9_layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    pad_9_layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    pad_9_layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    pad_9_layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    pad_9_layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    pad_9_layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_9_layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_9_layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    pad_9_layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    pad_9_layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_9_layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_9_layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_9_layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    pad_9_layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_9_layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_9_layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_9_layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    pad_9_layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_9_layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_9_layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    pad_9_layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    pad_9_layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_9_layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_9_layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_9_layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_9_layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_9_layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    pad_9_layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    pad_9_layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    pad_9_layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_9_layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_9_layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    pad_9_layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    pad_9_layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    pad_9_layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    pad_9_layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_9_layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    pad_9_layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_9_layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    pad_9_layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    pad_9_layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    pad_9_layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    pad_9_layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    pad_9_layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    pad_9_layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    pad_9_layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    pad_9_layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    pad_9_layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_9_layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    pad_9_layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    pad_9_layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    pad_9_layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_9_layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_9_layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_9_layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    pad_9_layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_9_layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_9_layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    pad_9_layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    pad_9_layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    pad_9_layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_9_layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    pad_9_layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    pad_9_layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    pad_9_layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    pad_9_layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    pad_9_layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_9_layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    pad_9_layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    pad_9_layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    pad_9_layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    pad_9_layer_2_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_1_layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_1_layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    trig_1_layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    trig_1_layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    trig_1_layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    trig_1_layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_1_layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    trig_1_layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_1_layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    trig_1_layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    trig_1_layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    trig_1_layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    trig_1_layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    trig_1_layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    trig_1_layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    trig_1_layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    trig_1_layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    trig_1_layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_1_layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_1_layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    trig_1_layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    trig_1_layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_1_layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_1_layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_1_layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    trig_1_layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_1_layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_1_layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_1_layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    trig_1_layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_1_layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_1_layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    trig_1_layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    trig_1_layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_1_layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_1_layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_1_layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_1_layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_1_layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    trig_1_layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    trig_1_layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    trig_1_layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_1_layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_1_layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    trig_1_layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    trig_1_layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    trig_1_layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    trig_1_layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_1_layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    trig_1_layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_1_layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    trig_1_layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    trig_1_layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    trig_1_layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    trig_1_layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    trig_1_layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    trig_1_layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    trig_1_layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    trig_1_layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    trig_1_layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_1_layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_1_layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    trig_1_layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    trig_1_layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_1_layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_1_layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_1_layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    trig_1_layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_1_layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_1_layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_1_layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    trig_1_layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_1_layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_1_layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    trig_1_layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    trig_1_layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_1_layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_1_layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_1_layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_1_layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_1_layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    trig_1_layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    trig_1_layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    trig_1_layer_2_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_2_layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_2_layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    trig_2_layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    trig_2_layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    trig_2_layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    trig_2_layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_2_layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    trig_2_layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_2_layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    trig_2_layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    trig_2_layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    trig_2_layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    trig_2_layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    trig_2_layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    trig_2_layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    trig_2_layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    trig_2_layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    trig_2_layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_2_layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_2_layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    trig_2_layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    trig_2_layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_2_layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_2_layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_2_layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    trig_2_layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_2_layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_2_layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_2_layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    trig_2_layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_2_layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_2_layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    trig_2_layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    trig_2_layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_2_layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_2_layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_2_layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_2_layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_2_layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    trig_2_layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    trig_2_layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    trig_2_layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_2_layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_2_layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    trig_2_layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    trig_2_layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    trig_2_layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    trig_2_layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_2_layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    trig_2_layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_2_layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    trig_2_layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    trig_2_layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    trig_2_layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    trig_2_layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    trig_2_layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    trig_2_layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    trig_2_layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    trig_2_layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    trig_2_layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_2_layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_2_layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    trig_2_layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    trig_2_layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_2_layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_2_layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_2_layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    trig_2_layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_2_layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_2_layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_2_layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    trig_2_layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_2_layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_2_layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    trig_2_layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    trig_2_layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_2_layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_2_layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_2_layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_2_layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_2_layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    trig_2_layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    trig_2_layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    trig_2_layer_2_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_3_layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_3_layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    trig_3_layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    trig_3_layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    trig_3_layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    trig_3_layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_3_layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    trig_3_layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_3_layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    trig_3_layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    trig_3_layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    trig_3_layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    trig_3_layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    trig_3_layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    trig_3_layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    trig_3_layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    trig_3_layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    trig_3_layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_3_layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_3_layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    trig_3_layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    trig_3_layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_3_layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_3_layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_3_layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    trig_3_layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_3_layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_3_layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_3_layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    trig_3_layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_3_layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_3_layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    trig_3_layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    trig_3_layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_3_layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_3_layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_3_layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_3_layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_3_layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    trig_3_layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    trig_3_layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    trig_3_layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_3_layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_3_layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    trig_3_layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    trig_3_layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    trig_3_layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    trig_3_layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_3_layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    trig_3_layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_3_layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    trig_3_layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    trig_3_layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    trig_3_layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    trig_3_layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    trig_3_layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    trig_3_layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    trig_3_layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    trig_3_layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    trig_3_layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_3_layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_3_layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    trig_3_layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    trig_3_layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_3_layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_3_layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_3_layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    trig_3_layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_3_layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_3_layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_3_layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    trig_3_layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_3_layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_3_layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    trig_3_layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    trig_3_layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_3_layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_3_layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_3_layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_3_layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_3_layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    trig_3_layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    trig_3_layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    trig_3_layer_2_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_4_layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_4_layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    trig_4_layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    trig_4_layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    trig_4_layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    trig_4_layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_4_layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    trig_4_layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_4_layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    trig_4_layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    trig_4_layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    trig_4_layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    trig_4_layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    trig_4_layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    trig_4_layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    trig_4_layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    trig_4_layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    trig_4_layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_4_layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_4_layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    trig_4_layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    trig_4_layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_4_layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_4_layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_4_layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    trig_4_layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_4_layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_4_layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_4_layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    trig_4_layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_4_layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_4_layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    trig_4_layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    trig_4_layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_4_layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_4_layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_4_layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_4_layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_4_layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    trig_4_layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    trig_4_layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    trig_4_layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_4_layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_4_layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    trig_4_layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    trig_4_layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    trig_4_layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    trig_4_layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_4_layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    trig_4_layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_4_layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    trig_4_layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    trig_4_layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    trig_4_layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    trig_4_layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    trig_4_layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    trig_4_layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    trig_4_layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    trig_4_layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    trig_4_layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_4_layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_4_layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    trig_4_layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    trig_4_layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_4_layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_4_layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_4_layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    trig_4_layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_4_layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_4_layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_4_layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    trig_4_layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_4_layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_4_layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    trig_4_layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    trig_4_layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_4_layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_4_layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_4_layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_4_layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_4_layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    trig_4_layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    trig_4_layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    trig_4_layer_2_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_5_layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_5_layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    trig_5_layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    trig_5_layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    trig_5_layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    trig_5_layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_5_layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    trig_5_layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_5_layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    trig_5_layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    trig_5_layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    trig_5_layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    trig_5_layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    trig_5_layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    trig_5_layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    trig_5_layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    trig_5_layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    trig_5_layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_5_layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_5_layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    trig_5_layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    trig_5_layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_5_layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_5_layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_5_layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    trig_5_layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_5_layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_5_layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_5_layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    trig_5_layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_5_layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_5_layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    trig_5_layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    trig_5_layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_5_layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_5_layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_5_layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_5_layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_5_layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    trig_5_layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    trig_5_layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    trig_5_layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_5_layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_5_layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    trig_5_layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    trig_5_layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    trig_5_layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    trig_5_layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_5_layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    trig_5_layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_5_layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    trig_5_layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    trig_5_layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    trig_5_layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    trig_5_layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    trig_5_layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    trig_5_layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    trig_5_layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    trig_5_layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    trig_5_layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_5_layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_5_layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    trig_5_layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    trig_5_layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_5_layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_5_layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_5_layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    trig_5_layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_5_layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_5_layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_5_layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    trig_5_layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_5_layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_5_layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    trig_5_layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    trig_5_layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_5_layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_5_layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_5_layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_5_layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_5_layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    trig_5_layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    trig_5_layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    trig_5_layer_2_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_6_layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_6_layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    trig_6_layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    trig_6_layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    trig_6_layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    trig_6_layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_6_layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    trig_6_layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_6_layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    trig_6_layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    trig_6_layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    trig_6_layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    trig_6_layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    trig_6_layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    trig_6_layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    trig_6_layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    trig_6_layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    trig_6_layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_6_layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_6_layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    trig_6_layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    trig_6_layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_6_layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_6_layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_6_layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    trig_6_layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_6_layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_6_layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_6_layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    trig_6_layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_6_layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_6_layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    trig_6_layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    trig_6_layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_6_layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_6_layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_6_layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_6_layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_6_layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    trig_6_layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    trig_6_layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    trig_6_layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_6_layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_6_layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    trig_6_layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    trig_6_layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    trig_6_layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    trig_6_layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_6_layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    trig_6_layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_6_layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    trig_6_layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    trig_6_layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    trig_6_layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    trig_6_layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    trig_6_layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    trig_6_layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    trig_6_layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    trig_6_layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    trig_6_layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_6_layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_6_layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    trig_6_layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    trig_6_layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_6_layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_6_layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_6_layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    trig_6_layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_6_layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_6_layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_6_layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    trig_6_layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_6_layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_6_layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    trig_6_layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    trig_6_layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_6_layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_6_layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_6_layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_6_layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_6_layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    trig_6_layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    trig_6_layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    trig_6_layer_2_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_7_layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_7_layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    trig_7_layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    trig_7_layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    trig_7_layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    trig_7_layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_7_layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    trig_7_layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_7_layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    trig_7_layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    trig_7_layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    trig_7_layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    trig_7_layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    trig_7_layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    trig_7_layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    trig_7_layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    trig_7_layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    trig_7_layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_7_layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_7_layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    trig_7_layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    trig_7_layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_7_layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_7_layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_7_layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    trig_7_layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_7_layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_7_layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_7_layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    trig_7_layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_7_layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_7_layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    trig_7_layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    trig_7_layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_7_layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_7_layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_7_layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_7_layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_7_layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    trig_7_layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    trig_7_layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    trig_7_layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_7_layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_7_layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    trig_7_layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    trig_7_layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    trig_7_layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    trig_7_layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_7_layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    trig_7_layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_7_layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    trig_7_layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    trig_7_layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    trig_7_layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    trig_7_layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    trig_7_layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    trig_7_layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    trig_7_layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    trig_7_layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    trig_7_layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_7_layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_7_layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    trig_7_layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    trig_7_layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_7_layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_7_layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_7_layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    trig_7_layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_7_layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_7_layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_7_layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    trig_7_layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_7_layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_7_layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    trig_7_layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    trig_7_layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_7_layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_7_layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_7_layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_7_layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_7_layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    trig_7_layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    trig_7_layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    trig_7_layer_2_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_8_layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_8_layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    trig_8_layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    trig_8_layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    trig_8_layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    trig_8_layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_8_layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    trig_8_layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_8_layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    trig_8_layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    trig_8_layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    trig_8_layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    trig_8_layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    trig_8_layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    trig_8_layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    trig_8_layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    trig_8_layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    trig_8_layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_8_layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_8_layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    trig_8_layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    trig_8_layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_8_layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_8_layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_8_layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    trig_8_layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_8_layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_8_layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_8_layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    trig_8_layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_8_layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_8_layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    trig_8_layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    trig_8_layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_8_layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_8_layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_8_layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_8_layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_8_layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    trig_8_layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    trig_8_layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    trig_8_layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_8_layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_8_layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    trig_8_layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    trig_8_layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    trig_8_layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    trig_8_layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_8_layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    trig_8_layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_8_layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    trig_8_layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    trig_8_layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    trig_8_layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    trig_8_layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    trig_8_layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    trig_8_layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    trig_8_layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    trig_8_layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    trig_8_layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_8_layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    trig_8_layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    trig_8_layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    trig_8_layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_8_layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_8_layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_8_layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    trig_8_layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_8_layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_8_layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    trig_8_layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    trig_8_layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    trig_8_layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_8_layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    trig_8_layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    trig_8_layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    trig_8_layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    trig_8_layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trig_8_layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_8_layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    trig_8_layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    trig_8_layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    trig_8_layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    trig_8_layer_2_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_1_layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    foot_sw_1_layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    foot_sw_1_layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    foot_sw_1_layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    foot_sw_1_layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    foot_sw_1_layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    foot_sw_1_layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    foot_sw_1_layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_1_layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    foot_sw_1_layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    foot_sw_1_layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    foot_sw_1_layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    foot_sw_1_layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    foot_sw_1_layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    foot_sw_1_layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    foot_sw_1_layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    foot_sw_1_layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    foot_sw_1_layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_1_layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    foot_sw_1_layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    foot_sw_1_layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    foot_sw_1_layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    foot_sw_1_layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    foot_sw_1_layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    foot_sw_1_layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    foot_sw_1_layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    foot_sw_1_layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    foot_sw_1_layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    foot_sw_1_layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    foot_sw_1_layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    foot_sw_1_layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    foot_sw_1_layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    foot_sw_1_layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    foot_sw_1_layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    foot_sw_1_layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    foot_sw_1_layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_1_layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    foot_sw_1_layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    foot_sw_1_layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    foot_sw_1_layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    foot_sw_1_layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    foot_sw_1_layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_1_layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    foot_sw_1_layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    foot_sw_1_layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    foot_sw_1_layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    foot_sw_1_layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    foot_sw_1_layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    foot_sw_1_layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    foot_sw_1_layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_1_layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    foot_sw_1_layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    foot_sw_1_layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    foot_sw_1_layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    foot_sw_1_layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    foot_sw_1_layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    foot_sw_1_layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    foot_sw_1_layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    foot_sw_1_layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    foot_sw_1_layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_1_layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    foot_sw_1_layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    foot_sw_1_layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    foot_sw_1_layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    foot_sw_1_layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    foot_sw_1_layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    foot_sw_1_layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    foot_sw_1_layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    foot_sw_1_layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    foot_sw_1_layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    foot_sw_1_layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    foot_sw_1_layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    foot_sw_1_layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    foot_sw_1_layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    foot_sw_1_layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    foot_sw_1_layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    foot_sw_1_layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    foot_sw_1_layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_1_layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    foot_sw_1_layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    foot_sw_1_layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    foot_sw_1_layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    foot_sw_1_layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    foot_sw_1_layer_2_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_2_layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    foot_sw_2_layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    foot_sw_2_layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    foot_sw_2_layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    foot_sw_2_layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    foot_sw_2_layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    foot_sw_2_layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    foot_sw_2_layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_2_layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    foot_sw_2_layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    foot_sw_2_layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    foot_sw_2_layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    foot_sw_2_layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    foot_sw_2_layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    foot_sw_2_layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    foot_sw_2_layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    foot_sw_2_layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    foot_sw_2_layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_2_layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    foot_sw_2_layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    foot_sw_2_layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    foot_sw_2_layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    foot_sw_2_layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    foot_sw_2_layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    foot_sw_2_layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    foot_sw_2_layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    foot_sw_2_layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    foot_sw_2_layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    foot_sw_2_layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    foot_sw_2_layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    foot_sw_2_layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    foot_sw_2_layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    foot_sw_2_layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    foot_sw_2_layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    foot_sw_2_layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    foot_sw_2_layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_2_layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    foot_sw_2_layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    foot_sw_2_layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    foot_sw_2_layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    foot_sw_2_layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    foot_sw_2_layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_2_layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    foot_sw_2_layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    foot_sw_2_layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    foot_sw_2_layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    foot_sw_2_layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    foot_sw_2_layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    foot_sw_2_layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    foot_sw_2_layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_2_layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    foot_sw_2_layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    foot_sw_2_layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    foot_sw_2_layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    foot_sw_2_layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    foot_sw_2_layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    foot_sw_2_layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    foot_sw_2_layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    foot_sw_2_layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    foot_sw_2_layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_2_layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    foot_sw_2_layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    foot_sw_2_layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    foot_sw_2_layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    foot_sw_2_layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    foot_sw_2_layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    foot_sw_2_layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    foot_sw_2_layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    foot_sw_2_layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    foot_sw_2_layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    foot_sw_2_layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    foot_sw_2_layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    foot_sw_2_layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    foot_sw_2_layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    foot_sw_2_layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    foot_sw_2_layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    foot_sw_2_layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    foot_sw_2_layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    foot_sw_2_layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    foot_sw_2_layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    foot_sw_2_layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    foot_sw_2_layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    foot_sw_2_layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    foot_sw_2_layer_2_unknown_12: [u8; 12], //  Padding?
}
// To test: field count: 2295
