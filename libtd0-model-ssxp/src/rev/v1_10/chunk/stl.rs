// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

// Data types for TD0 SSXP Setlist data.
use td0_derive::{TD0ChunkItemDerive, repeat_fields};
use zerocopy::{LittleEndian, U16};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[repeat_fields]
#[derive(Clone, Copy, Debug, FromBytes, Immutable, IntoBytes, KnownLayout, TD0ChunkItemDerive)]
#[repr(C, packed)]
pub struct STLaItem {
    /* Setlist:
     *
     * 16-byte name
     * 32 x 8-byte Kit Selection
     */
    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    name: [u8; 16],

    #[repeat_section(count = 32, prefix_format = "step_{}")]
    #[td0_field(field_type = "U16", max = 200)]
    kit: U16<LittleEndian>,

    #[repeat_section_last]
    #[td0_field(field_type = "Slice")]
    unknown: [u8; 6],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stl_a_item_expected_num_fields() {
        assert_eq!(STLA_ITEM_FIELDS.len(), 65);
    }
}
