// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use td0_derive::{TD0ChunkItemDerive, repeat_fields};
use zerocopy::{LittleEndian, U16};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[repeat_fields]
#[derive(Clone, Copy, Debug, FromBytes, Immutable, IntoBytes, KnownLayout, TD0ChunkItemDerive)]
#[repr(C, packed)]
pub struct STPbItem {
    #[td0_field(field_type = "Slice")]
    unknown_1: [u8; 248],

    /*  This is ugly: The chunk header describes this chunk as containing
        only one item- some (as yet unknown in purpose) data followed by
        sixteen color data definition blocks comprising of four fields apiece.
        (R, G, B, and something)

        This library will follow the file format as well as it is understood,
        and won't attempt to fix usability issues in this way. Ideally, any
        client program(s) will solve these shortcomings.
    */
    #[repeat_section(count = 16, prefix_format = "color_{}")]
    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    name: [u8; 16],

    #[td0_field(field_type = "U16")]
    red: U16<LittleEndian>,

    #[td0_field(field_type = "U16")]
    green: U16<LittleEndian>,

    #[td0_field(field_type = "U16")]
    blue: U16<LittleEndian>,

    #[repeat_section_last]
    #[td0_field(field_type = "U16")]
    unknown: U16<LittleEndian>,

    // Extra 128 bytes of something

    // "Backup" on the sample I've seen.
    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    unknown_name: [u8; 16],

    #[td0_field(field_type = "Slice")]
    unknown_2: [u8; 112],
}
