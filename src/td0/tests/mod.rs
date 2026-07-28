#![allow(unused)]
use zerocopy_derive::{FromBytes, IntoBytes, KnownLayout};
use zerocopy::{U16, I16, LittleEndian};

use libtd0_derive::TD0ChunkItem;
use libtd0_derive::repeat_fields;
use super::chunk::{ChunkItem,ChunkItemValue,ChunkItemValueRaw, IntEncodedDecimal, Volume};
use super::TD0Result;

pub const SYS_KIT_SWITCH : [&'static str ; 2] = [
    "SYSTEM",
    "KIT",
];

#[repeat_fields]
#[derive(Clone, Copy, Debug, FromBytes, IntoBytes, TD0ChunkItem)]
#[repr(C, packed)]
pub struct TestStruct {
    #[td0_field(field_type = "Text", pad_byte = 0x0)]
    name: [u8; 16],
    #[td0_field(field_type = "Text", pad_byte = 0x0)]
    memo: [u8; 64],
    #[td0_field(field_type = "Volume")]
    volume: I16<LittleEndian>,
    #[td0_field(field_type = "EnumStr", collection = "SYS_KIT_SWITCH")]
    click_setting: u8,
}

#[cfg(test)]

#[test]
fn test_create_struct() { 
    let ts = TestStruct::default();
    assert_eq!(ts.name, [0; 16]);
    assert_eq!(ts.get_value("name"), Some(ChunkItemValue::Text("".to_string())));
    assert_eq!(ts.get_value("click_setting"), Some(ChunkItemValue::EnumStr("SYSTEM")));
}


#[test]
fn test_enumstr_get_value() {
    let val = *SYS_KIT_SWITCH.get(0).unwrap_or_else(|| &"INVALID");

    //println!("\n\n\n\n Val is: `{val}`");
    assert_eq!(val, "SYSTEM");
}