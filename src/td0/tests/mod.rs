#![allow(unused)]
use zerocopy::{I16, LittleEndian, U16};
use zerocopy_derive::{FromBytes, IntoBytes, KnownLayout};

use super::chunk::{ChunkItem, ChunkItemValue, ChunkItemValueRaw, IntEncodedDecimal, Volume};
use super::strings::{LED_COLORS, SYS_KIT_SWITCH};
use crate::td0::result::{TD0Error, TD0Result};
use libtd0_derive::TD0ChunkItem;
use libtd0_derive::repeat_fields;
use rust_decimal::Decimal;

#[repeat_fields]
#[derive(Clone, Copy, Debug, FromBytes, IntoBytes, TD0ChunkItem)]
#[repr(C, packed)]
pub struct TestStruct {
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

    #[repeat(count = 4, format = "mfx_{}_routing")]
    #[td0_field(field_type = "U8")]
    mfx_1_routing: u8,

    #[td0_field(field_type = "TD0Decimal", min = 20.0, max = 260.0)]
    tempo: U16<LittleEndian>,

    #[td0_field(field_type = "Slice")]
    unknown_2: [u8; 3],

    #[repeat(count = 15, format = "led_{}")]
    #[td0_field(field_type = "EnumStr", collection = "LED_COLORS")]
    led_1: u8,
}

#[cfg(test)]
#[test]
fn test_create_struct() {
    let ts = TestStruct::default();
    assert_eq!(ts.name, [32; 16]);
    assert_eq!(
        ts.get_value("name"),
        Some(ChunkItemValue::Text("".to_string()))
    );
    assert_eq!(
        ts.get_value("click_setting"),
        Some(ChunkItemValue::EnumStr("SYSTEM"))
    );
    assert_eq!(
        ts.get_value("volume"),
        Some(ChunkItemValue::Volume(
            Volume::try_from(0i16).expect("Test is broken.")
        ))
    );
    assert_eq!(
        ts.get_value("tempo"),
        Some(ChunkItemValue::TD0Decimal(
            IntEncodedDecimal::new_from_parts_raw(
                200u8,
                &Decimal::from_str_exact("20.0").unwrap(),
                &Decimal::from_str_exact("260.0").unwrap(),
            )
            .expect("Test broken")
        ))
    );
    assert_eq!(
        format!("{}", ts.get_value("unknown_2").expect("Test is broken")),
        "[0, 0, 0]"
    );
}

#[test]
fn test_enumstr_get_value() {
    let val = *SYS_KIT_SWITCH.get(0).unwrap_or_else(|| &"INVALID");
    assert_eq!(val, "SYSTEM");
}
