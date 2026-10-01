// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use td0_derive::TD0ChunkItemDerive;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Clone, Copy, Debug, FromBytes, Immutable, IntoBytes, KnownLayout, TD0ChunkItemDerive)]
#[repr(C, packed)]
pub struct TGLaItem {
    /* Represents a tag in the tag library. */
    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    name: [u8; 16],
}
