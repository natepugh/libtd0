use libtd0_derive::{TD0ChunkItemDerive, repeat_fields};
use zerocopy::{I16, LittleEndian, U16};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::td0::model::ssxp::strings::{
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
#[derive(Clone, Copy, Debug, FromBytes, Immutable, IntoBytes, KnownLayout, TD0ChunkItemDerive)]
#[repr(C, packed)]
pub struct KITbItem {
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
    #[repeat_section(count = 4, prefix_format = "mfx_{}")]
    #[td0_field(field_type = "EnumStr", collection = "MFX")]
    effect_select: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    enable: u8,

    #[td0_field(field_type = "Volume")]
    volume: I16<LittleEndian>,

    #[td0_field(field_type = "Slice")]
    unknown: [u8; 4],

    #[repeat_section_last]
    #[repeat(count = 32, format = "param_{}")]
    #[td0_field(field_type = "U16")]
    param_1: U16<LittleEndian>,

    /* Pad Settings */
    // pad_1 - 9
    #[repeat_section(count = 9, prefix_format = "pad_{}")]
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trigger_reserve: u8,

    #[repeat_section_last]
    #[td0_field(field_type = "Slice")]
    unknown: [u8; 8],

    // trig_1 - 8
    #[repeat_section(count = 8, prefix_format = "trig_{}")]
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trigger_reserve: u8,

    #[repeat_section_last]
    #[td0_field(field_type = "Slice")]
    unknown: [u8; 8],

    //
    // foot_sw_1 - 2
    #[repeat_section(count = 2, prefix_format = "foot_sw_{}")]
    #[td0_field(field_type = "EnumStr", collection = "PAD_LAYER_TYPES")]
    layer_type: u8,

    #[td0_field(field_type = "U8", max = 127)]
    fade_start: u8,

    #[td0_field(field_type = "U8", max = 127)]
    fade_end: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    dynamics_switch: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_DYNAMICS_CURVES")]
    dynamics_curve: u8,

    #[td0_field(field_type = "U8", max = 127)]
    fixed_velocity: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_EXT_CTRL_SETTINGS")]
    ext_ctrl: u8,

    #[td0_field(field_type = "U8", max = 127)]
    hihat_mode_volume: u8,

    #[td0_field(field_type = "U8", max = 127)]
    hihat_mode_fade_in: u8,

    #[td0_field(field_type = "U8", max = 127)]
    hihat_mode_decay: u8,

    #[td0_field(field_type = "U8", max = 17)]
    mute_grp_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    mute_grp_recv: u8,

    #[td0_field(field_type = "U8", max = 17)]
    link_send: u8,

    #[td0_field(field_type = "U8", max = 17)]
    link_recv: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_OUTPUT_ROUTES")]
    output_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "DIRECT_ROUTES")]
    direct_out: u8,

    #[td0_field(field_type = "EnumStr", collection = "PAD_MFX_ROUTES")]
    mfx_routing: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    sidechain_ctrl: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    exp_pedal_rx_ctrl_sw: u8,

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    trigger_reserve: u8,

    #[repeat_section_last]
    #[td0_field(field_type = "Slice")]
    unknown: [u8; 8],

    /* Pad Layer Settings */
    // pad_1 - 9
    #[repeat_section(count = 9, prefix_format = "pad_{}")]
    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[repeat_section_last]
    #[td0_field(field_type = "Slice")]
    layer_2_unknown_12: [u8; 12], //  Padding?

    // trig_1 - 8
    #[repeat_section(count = 8, prefix_format = "trig_{}")]
    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[repeat_section_last]
    #[td0_field(field_type = "Slice")]
    layer_2_unknown_12: [u8; 12], //  Padding?

    // foot_sw_1 - 2
    #[repeat_section(count = 2, prefix_format = "foot_sw_{}")]
    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_1_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    layer_1_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    layer_1_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    layer_1_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    layer_1_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    layer_1_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    layer_1_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    layer_1_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_1_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    layer_1_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    layer_1_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    layer_1_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    layer_1_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    layer_1_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    layer_1_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    layer_1_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    layer_1_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    layer_1_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_1_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    layer_1_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    layer_1_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    layer_1_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_1_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    layer_1_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    layer_1_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    layer_1_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    layer_1_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_1_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    layer_1_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    layer_1_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    layer_1_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_1_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    layer_1_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    layer_1_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_1_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    layer_1_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_1_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    layer_1_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    layer_1_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    layer_1_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    layer_1_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "Slice")]
    layer_1_unknown_12: [u8; 12], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_2_enable: u8, //  Layer Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    layer_2_unknown_1: u8, //  Unknown

    #[td0_field(field_type = "U16", max = 20000)]
    layer_2_sample_slot: U16<LittleEndian>, //  Sample Slot

    #[td0_field(field_type = "Volume")]
    layer_2_volume: I16<LittleEndian>, //  Volume -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[td0_field(field_type = "I8", min = -15, max = 15)]
    layer_2_pan: i8, //  Pan:          -15 ... 0 ... 15

    #[td0_field(field_type = "I8", min = -12, max = 12)]
    layer_2_pitch_coarse: i8, //  Pitch Coarse: -12 ... 0 ... 12

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    layer_2_pitch_fine: i8, //  Pitch Fine:   -50 ... 0 ... 50

    #[td0_field(field_type = "U8")]
    layer_2_unknown_2: u8, //  Unknown

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_2_delaysync_enable: u8, //  DelaySync Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "EnumStr", collection = "DELAYSYNC_VALUES")]
    layer_2_delaysync: u8, //  DelaySync Value:

    #[td0_field(field_type = "U16", max = 5000)]
    layer_2_sample_delay_ms: U16<LittleEndian>, //  Sample Delay ms: Unsigned 2-byte, 0 .. 5000

    #[td0_field(field_type = "EnumStr", collection = "POLY_MODES")]
    layer_2_poly_enable: u8, //  Poly Enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8", max = 127)]
    layer_2_fade_in: u8, //  Fade In: 0 - 127

    #[td0_field(field_type = "U8", max = 127)]
    layer_2_decay: u8, //  Decay  : 0 - 127

    #[td0_field(field_type = "U8")]
    layer_2_unknown_3: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "LOOP_MODES")]
    layer_2_loop_mode: u8, //  Loop mode: 0 - 5 = LOOP_MODES[n]

    #[td0_field(field_type = "EnumStr", collection = "TRIGGER_TYPES")]
    layer_2_trigger_type: u8, //  Trigger Type: 0 = ONESHOT, 1 = ALTERNATE

    #[td0_field(field_type = "Slice")]
    layer_2_unknown_4: [u8; 4], //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_2_eq_enable: u8, //  EQ enable: 0 = Off, 1 = On

    #[td0_field(field_type = "U8")]
    layer_2_unknown_5: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_LOW_FREQS")]
    layer_2_eq_low_freq: u8, //  EQ low Freq

    #[td0_field(field_type = "U8")]
    layer_2_unknown_6: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_2_eq_low_gain: i8, //  EQ low gain:      -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    layer_2_unknown_7: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    layer_2_eq_mid1_freq: u8, //  EQ Mid1 Freq

    #[td0_field(field_type = "U8")]
    layer_2_unknown_8: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    layer_2_eq_mid1_q: u8, //  EQ Mid1 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_2_eq_mid1_gain: i8, //  EQ Mid1 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_MID_FREQS")]
    layer_2_eq_mid2_freq: u8, //  EQ Mid2 Freq

    #[td0_field(field_type = "U8")]
    layer_2_unknown_9: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "EQ_Q_VALS")]
    layer_2_eq_mid2_q: u8, //  EQ Mid2 Q

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_2_eq_mid2_gain: i8, //  EQ Mid2 Gain:     -24 ... 0 ... 24

    #[td0_field(field_type = "EnumStr", collection = "EQ_HI_FREQS")]
    layer_2_eq_hi_freq: u8, //  EQ hi Freq

    #[td0_field(field_type = "U8")]
    layer_2_unknown_10: u8, //  Padding?

    #[td0_field(field_type = "I8", min = -24, max = 24)]
    layer_2_eq_hi_gain: i8, //  EQ hi gain:       -24 ... 0 ... 24

    #[td0_field(field_type = "U8")]
    layer_2_unknown_11: u8, //  Padding?

    #[td0_field(field_type = "EnumStr", collection = "OFF_ON")]
    layer_2_transient_enable: u8, //  Transient enable: 0 = Off, 1 = On

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    layer_2_transient_attack: i8, //  Transient attack: -50 ... 0 ... 50

    #[td0_field(field_type = "I8", min = -50, max = 50)]
    layer_2_transient_release: i8, //  Transient release:-50 ... 0 ... 50

    #[td0_field(field_type = "EnumStr", collection = "TRANS_SENS")]
    layer_2_transient_sensitivity: u8, //  Transient Sensitivity: 0 - 3 TRANS_SENS[n]

    #[td0_field(field_type = "Volume")]
    layer_2_transient_gain: I16<LittleEndian>, //  Trans Gain: -601 = -INF, -600 = -60.0 dB, -599 = -59.9 dB ... 0 = 0.0 dB(FS), 1 = 0.1 dB, 60 = 6.0 dB

    #[repeat_section_last]
    #[td0_field(field_type = "Slice")]
    layer_2_unknown_12: [u8; 12], //  Padding?

    /* NOTE: Possible data here?
     *   ( see https://www.roland.com/us/support/by_product/spd-sx_pro/updates_drivers/2e9e3491-dd14-4d5d-99e3-4674794d0cde/ )
     *
     *    - Per-Kit Master Effect Support: Master Effects can now be set individually for each kit.
     *    - Loop Stop Setting When Switching Kits:
     *      Previously, loop phrases continued playing when switching kits.
     *      A new setting allows loop playback to stop when changing kits.
     *      (Set to "Loop ON (Mute on KitChg)" on the PAD EDIT (1/5) screen.)
     *
     *   - Pad Output Mute for PHONES and MASTER OUT
     *       You can now set individual pads so that they do not output sound to PHONES or MASTER OUT.
     *       Previously, sound was always output to PHONES.
     *       This is useful when you want to output sound only to DIRECT OUT, or when using a kick pad as a control signal for sidechain processing and do not want its sound sent to PHONES or MASTER OUT.
     *       (Set OUTPUT to "OFF" on the OUTPUT/EFFECTS screen or the OUTPUT ASSIGN - PAD OUTPUT screen.)
     */
    #[td0_field(field_type = "Slice")]
    kit_b_unknown: [u8; 72],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kit_b_item_expected_num_fields() {
        assert_eq!(KITB_ITEM_FIELDS.len(), 2297);
    }
}
