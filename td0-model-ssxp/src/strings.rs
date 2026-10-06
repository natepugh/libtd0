// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

#![cfg_attr(
    faster_incomplete_compile,
    expect(dead_code, reason = "faster incomplete compilation.")
)]

pub const CLICK_MODES: [&str; 3] = [
    "PLAY INTERNAL CLICK",
    "PLAY WAVE as CLICK",
    "PLAY WAVE as CLICK-TRACK",
];

pub const CLICK_SOUNDS: [&str; 10] = [
    "METRONOME",
    "BEEP",
    "WOOD BLOCK",
    "STICKS",
    "CLAVES",
    "AGOGO",
    "TRIANGLE",
    "TAMBOURINE",
    "BELL",
    "CABASA",
];

pub const DELAYSYNC_VALUES: [&str; 22] = [
    "1/64T", "1/64", "1/32T", "1/32", "1/16T", "1/32.", "1/16", "1/8T", "1/16.", "1/8", "1/4T",
    "1/8.", "1/4", "1/2T", "1/4.", "1/2", "1/1T", "1/2.", "1/1", "2/1T", "1/1.", "2/1",
];

pub const DIRECT_ROUTES: [&str; 10] = [
    "OFF",
    "DIRECT 1",
    "DIRECT 2",
    "DIRECT 1+2",
    "DIRECT 3",
    "DIRECT 4",
    "DIRECT 3+4",
    "MASTER DIRECT L",
    "MASTER DIRECT R",
    "MASTER DIRECT L+R",
];

pub const EXP_PARAMS: [&str; 98] = [
    "OFF",
    "CC1: ",
    "CC2: ",
    "CC3: ",
    "CC4: ",
    "CC5: ",
    "CC6: ",
    "CC7: ",
    "CC8: ",
    "CC9: ",
    "CC10: ",
    "CC11: ",
    "CC12: ",
    "CC13: ",
    "CC14: ",
    "CC15: ",
    "CC16: ",
    "CC17: ",
    "CC18: ",
    "CC19: ",
    "CC20: ",
    "CC21: ",
    "CC22: ",
    "CC23: ",
    "CC24: ",
    "CC25: ",
    "CC26: ",
    "CC27: ",
    "CC28: ",
    "CC29: ",
    "CC30: ",
    "CC31: ",
    "CC32: ",
    "CC33: ",
    "CC34: ",
    "CC35: ",
    "CC36: ",
    "CC37: ",
    "CC38: ",
    "CC39: ",
    "CC40: ",
    "CC41: ",
    "CC42: ",
    "CC43: ",
    "CC44: ",
    "CC45: ",
    "CC46: ",
    "CC47: ",
    "CC48: ",
    "CC49: ",
    "CC50: ",
    "CC51: ",
    "CC52: ",
    "CC53: ",
    "CC54: ",
    "CC55: ",
    "CC56: ",
    "CC57: ",
    "CC58: ",
    "CC59: ",
    "CC60: ",
    "CC61: ",
    "CC62: ",
    "CC63: ",
    "CC64: ",
    "CC65: ",
    "CC66: ",
    "CC67: ",
    "CC68: ",
    "CC69: ",
    "CC70: ",
    "CC71: ",
    "CC72: ",
    "CC73: ",
    "CC74: ",
    "CC75: ",
    "CC76: ",
    "CC77: ",
    "CC78: ",
    "CC79: ",
    "CC80: ",
    "CC81: ",
    "CC82: ",
    "CC83: ",
    "CC84: ",
    "CC85: ",
    "CC86: ",
    "CC87: ",
    "CC88: ",
    "CC89: ",
    "CC90: ",
    "CC91: ",
    "CC92: ",
    "CC93: ",
    "CC94: ",
    "CC95: ",
    "MASTER EFFECT CTRL",
    "EXPRESSION",
];

pub const EXP_PEDAL_MODES: [&str; 2] = ["EXP-CTRL", "HH-CTRL"];

pub const EQ_LOW_FREQS: [&str; 35] = [
    "20 Hz", "22.5 Hz", "25 Hz", "28.25 Hz", "31.5 Hz", "35.75 Hz", "40 Hz", "45 Hz", "50 Hz",
    "56.5 Hz", "63 Hz", "71.5 Hz", "80 Hz", "90 Hz", "100 Hz", "112.5 Hz", "125 Hz", "142.5 Hz",
    "160 Hz", "180 Hz", "200 Hz", "225 Hz", "250 Hz", "282.5 Hz", "315 Hz", "357.5 Hz", "400 Hz",
    "450 Hz", "500 Hz", "565 Hz", "630 Hz", "715 Hz", "800 Hz", "900 Hz", "1 kHz",
];

pub const EQ_HI_FREQS: [&str; 25] = [
    "1 kHz",
    "1.125 kHz",
    "1.25 kHz",
    "1.425 kHz",
    "1.6 kHz",
    "1.8 kHz",
    "2 kHz",
    "2.25 kHz",
    "2.5 kHz",
    "2.825 kHz",
    "3.15 kHz",
    "3.575 kHz",
    "4 kHz",
    "4.5 kHz",
    "5 kHz",
    "5.65 kHz",
    "6.3 kHz",
    "7.15 kHz",
    "8 kHz",
    "9 kHz",
    "10 kHz",
    "11.25 kHz",
    "12.5 kHz",
    "14.25 kHz",
    "16 kHz",
];

// NOTE: LOW and High overlap by one value, "1 kHz". The values stored in the
//   binary data are NOT continuous, as in HIGH starts at 0x00 : [&str; 2] = 1 kHz,
//   while low starts at 0x00 : [&str; 2] = 20 hZ and ENDS at 0x22 = 1 kHz. MID has all
//   values, in order.
//
//   Create the MID tuple by combining LOW + HIGH, but removing the last element,
//   "1 kHz", from LOW to avoid repeating the value.
//pub const EQ_MID_FREQS : [&str; 2] = EQ_LOW_FREQS[:-1] + EQ_HI_FREQS
pub const EQ_MID_FREQS: [&str; 59] = [
    "20 Hz",
    "22.5 Hz",
    "25 Hz",
    "28.25 Hz",
    "31.5 Hz",
    "35.75 Hz",
    "40 Hz",
    "45 Hz",
    "50 Hz",
    "56.5 Hz",
    "63 Hz",
    "71.5 Hz",
    "80 Hz",
    "90 Hz",
    "100 Hz",
    "112.5 Hz",
    "125 Hz",
    "142.5 Hz",
    "160 Hz",
    "180 Hz",
    "200 Hz",
    "225 Hz",
    "250 Hz",
    "282.5 Hz",
    "315 Hz",
    "357.5 Hz",
    "400 Hz",
    "450 Hz",
    "500 Hz",
    "565 Hz",
    "630 Hz",
    "715 Hz",
    "800 Hz",
    "900 Hz",
    "1 kHz",
    "1.125 kHz",
    "1.25 kHz",
    "1.425 kHz",
    "1.6 kHz",
    "1.8 kHz",
    "2 kHz",
    "2.25 kHz",
    "2.5 kHz",
    "2.825 kHz",
    "3.15 kHz",
    "3.575 kHz",
    "4 kHz",
    "4.5 kHz",
    "5 kHz",
    "5.65 kHz",
    "6.3 kHz",
    "7.15 kHz",
    "8 kHz",
    "9 kHz",
    "10 kHz",
    "11.25 kHz",
    "12.5 kHz",
    "14.25 kHz",
    "16 kHz",
];

pub const EQ_Q_VALS: [&str; 7] = ["0.5", "1.0", "2.0", "3.0", "4.0", "8.0", "16.0"];

pub const LED_COLORS: [&str; 17] = [
    "COLOR1", "COLOR2", "COLOR3", "COLOR4", "COLOR5", "COLOR6", "COLOR7", "COLOR8", "COLOR9",
    "COLOR10", "COLOR11", "COLOR12", "COLOR13", "COLOR14", "COLOR14", "COLOR15", "COLOR16",
];

pub const LED_MODES: [&str; 3] = ["STATIC", "STATE", "DYNAMIC"];

pub const LOOP_MODES: [&str; 5] = ["OFF", "ON", "X2", "X4", "X8"];

pub const MFX: [&str; 53] = [
    "TAPE ECHO",
    "DELAY",
    "TIME CTRL DELAY",
    "REVERSE DELAY",
    "2TAP PAN DELAY",
    "3TAP PAN DELAY",
    "MID-SIDE DELAY",
    "REVERB",
    "LONG REVERB",
    "ISOLATOR",
    "LOW BOOST",
    "SUPER FILTER",
    "MULTI MODE FILTER",
    "ENHANCER",
    "AUTO WAH",
    "HUMANIZER",
    "MID-SIDE EQ",
    "PHASER",
    "SMALL PHASER",
    "SCRIPT 100",
    "STEP PHASER",
    "INFINITE PHASER",
    "RING MODULATOR",
    "TREMOLO",
    "AUTO PAN",
    "SLICER",
    "FLANGER",
    "SBF-325",
    "STEP FLANGER",
    "CHORUS",
    "SPACE-D",
    "CE-1",
    "SDD-320",
    "JUNO-106 CHORUS",
    "OVERDRIVE",
    "DISTORTION",
    "T-SCREAM",
    "FUZZ",
    "TONE FATTENER",
    "HMS DISTORTION ",
    "SATURATOR ",
    "WARM SATURATOR",
    "SPEAKER SIMULATOR",
    "GUITAR AMP SIMULATOR",
    "COMPRESSOR",
    "MID-SIDE COMPRESSOR",
    "LIMITER",
    "GATE",
    "LOFI COMPRESS",
    "BIT CRUSHER",
    "PITCH SHIFTER",
    "DJFX LOOPER",
    "BPM LOOPER",
];

pub const EXP_PEDAL_MIDI_CHANNELS: [&str; 16] = [
    "CH 1", "CH 2", "CH 3", "CH 4", "CH 5", "CH 6", "CH 7", "CH 8", "CH 9", "CH 10", "CH 11",
    "CH 12", "CH 13", "CH 14", "CH 15", "CH 16",
];

pub const MIDI_CHANNELS: [&str; 17] = [
    "CH 1", "CH 2", "CH 3", "CH 4", "CH 5", "CH 6", "CH 7", "CH 8", "CH 9", "CH 10", "CH 11",
    "CH 12", "CH 13", "CH 14", "CH 15", "CH 16", "GLOBAL",
];

pub const OUTPUT_ROUTES: [&str; 2] = ["PHONES+MASTER", "PHONES ONLY"];
pub const SIDECHAIN_ROUTES: [&str; 2] = ["MASTER+PHONES", "PHONES-ONLY"];
pub const MFX_ROUTES: [&str; 3] = ["MASTER+PHONES", "PHONES-ONLY", "SIDE CHAIN"];

pub const PAD_EDIT_KNOB_GROUPS: [&str; 8] = [
    "MFX1",
    "MFX2",
    "MFX3",
    "MFX4",
    "SIDE CHAIN",
    "SYSTEM LED",
    "MASTER EFFECT",
    "PAD EDIT KNOB CC",
];

pub const PAD_DYNAMICS_CURVES: [&str; 4] = ["LINEAR", "LOUD1", "LOUD2", "LOUD3"];

pub const PAD_EXT_CTRL_SETTINGS: [&str; 3] = ["OFF", "ON", "ON-ALT"];

pub const PAD_LAYER_TYPES: [&str; 8] = [
    "MIX",
    "FADE1",
    "FADE2",
    "XFADE",
    "SWITCH",
    "SW[MONO];",
    "ALTERNATE",
    "HI-HAT",
];

pub const PAD_MFX_ROUTES: [&str; 4] = ["MFX 1", "MFX 2", "MFX 3", "MFX 4"];

pub const PAD_OUTPUT_ROUTES: [&str; 4] = ["MASTER+PHONES", "PHONES-ONLY", "MFX", "SIDECHAIN"];

pub const PAD_TRIGGER_TYPES: [&str; 3] = ["ONE-TIME", "RETRIGGER", "ALTERNATE"];

pub const PADS: [&str; 20] = [
    "OFF",
    "PAD 1",
    "PAD 2",
    "PAD 3",
    "PAD 4",
    "PAD 5",
    "PAD 6",
    "PAD 7",
    "PAD 8",
    "PAD 9",
    "TRIG 1",
    "TRIG 2",
    "TRIG 3",
    "TRIG 4",
    "TRIG 5",
    "TRIG 6",
    "TRIG 7",
    "TRIG 8",
    "FOOT SW 1",
    "FOOT SW 2",
];

pub const POLY_MODES: [&str; 2] = ["MONO", "POLY"];

pub const SEQUENCE_OPTS: [&str; 21] = [
    "OFF",
    "PAD 1",
    "PAD 2",
    "PAD 3",
    "PAD 4",
    "PAD 5",
    "PAD 6",
    "PAD 7",
    "PAD 8",
    "PAD 9",
    "TRIG 1",
    "TRIG 2",
    "TRIG 3",
    "TRIG 4",
    "TRIG 5",
    "TRIG 6",
    "TRIG 7",
    "TRIG 8",
    "FOOT SW 1",
    "FOOT SW 2",
    "SKIP",
];

pub const OFF_ON: [&str; 2] = ["Off", "On"];

pub const SYS_KIT_SWITCH: [&str; 2] = ["SYSTEM", "KIT"];

pub const TRIGGER_TYPES: [&str; 2] = ["ONESHOT", "ALTERNATE"];

pub const TRANS_SENS: [&str; 4] = ["ULOW", "LOW", "MID", "HIGH"];
