// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

#![cfg(test)]
#![allow(clippy::panic_in_result_fn, reason = "More concise tests.")]

use std::assert_matches;

use td0_core::{TD0ChunkItem, TD0Error, TD0Result, TD0Value, TD0ValueRaw, Volume};
use zerocopy::{I16, LittleEndian, U16, U32};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes};

use td0_derive::TD0ChunkItemDerive;
use td0_derive::repeat_fields;

const ENUM_STR_VALUES: [&str; 5] = [
    "Test_0_EnumStr",
    "Test_1_EnumStr",
    "Test_2_EnumStr",
    "Test_3_EnumStr",
    "Test_4_EnumStr",
];

const TEST_BROKEN: &str = "Test broken if failed.";

#[repeat_fields]
#[derive(Clone, Copy, Debug, FromBytes, Immutable, IntoBytes, TD0ChunkItemDerive)]
#[repr(C, packed)]
pub struct TestStruct {
    #[td0_field(field_type = "EnumStr", collection = "ENUM_STR_VALUES")]
    enumstr_field: u8,

    #[td0_field(field_type = "I16", min = -2048, max = 50)]
    i16_bounded: I16<LittleEndian>,

    #[td0_field(field_type = "I16")]
    i16_unbounded: I16<LittleEndian>,

    #[td0_field(field_type = "I8")]
    i8_unbounded: i8,

    #[td0_field(field_type = "I8", min = -45)]
    i8_bounded_min_only: i8,

    #[td0_field(field_type = "I8", max = -45)]
    i8_bounded_max_only: i8,

    #[td0_field(field_type = "Slice")]
    slice: [u8; 3],

    #[td0_field(field_type = "TD0Decimal", min = 20.0, max = 260.0)]
    td0_decimal_bounded: U16<LittleEndian>,

    #[td0_field(field_type = "TD0Decimal", min = 0.1, max = 8.0)]
    td0_decimal_u8: u8,

    #[td0_field(field_type = "Text", pad_byte = 0x20)]
    space_padded_text: [u8; 16],

    #[td0_field(field_type = "Text", pad_byte = 0)]
    zero_padded_text: [u8; 16],

    #[td0_field(field_type = "U16")]
    u16_unbounded: U16<LittleEndian>,

    #[td0_field(field_type = "U16", min = 257, max = 65500)]
    u16_bounded: U16<LittleEndian>,

    #[td0_field(field_type = "U32")]
    u32_unbounded: U32<LittleEndian>,

    #[td0_field(field_type = "U32", min = 65537, max = 655_377)]
    u32_bounded: U32<LittleEndian>,

    #[td0_field(field_type = "U8")]
    u8_unbounded: u8,

    #[td0_field(field_type = "U8", min = 27)]
    u8_bounded_min_only: u8,

    #[td0_field(field_type = "U8", max = 142)]
    u8_bounded_max_only: u8,

    #[td0_field(field_type = "Volume")]
    volume_field: I16<LittleEndian>,

    #[repeat(count = 3, format = "repeat_field_{}_val")]
    #[td0_field(field_type = "U8")]
    repeat_field_1_val: u8,

    #[repeat_section(count = 2, prefix_format = "item_{}")]
    #[td0_field(field_type = "U8")]
    val_1: u8,

    #[repeat(count = 3, format = "val_rpt_{}")]
    #[td0_field(field_type = "U8")]
    val_rpt_1: u8,

    #[repeat_section_last]
    #[td0_field(field_type = "U8")]
    val_3: u8,
}

#[test]
fn test_creates_expected_fields() {
    let expected: [&str; 32] = [
        "enumstr_field",
        "i16_bounded",
        "i16_unbounded",
        "i8_unbounded",
        "i8_bounded_min_only",
        "i8_bounded_max_only",
        "slice",
        "td0_decimal_bounded",
        "td0_decimal_u8",
        "space_padded_text",
        "zero_padded_text",
        "u16_unbounded",
        "u16_bounded",
        "u32_unbounded",
        "u32_bounded",
        "u8_unbounded",
        "u8_bounded_min_only",
        "u8_bounded_max_only",
        "volume_field",
        "repeat_field_1_val",
        "repeat_field_2_val",
        "repeat_field_3_val",
        "item_1_val_1",
        "item_1_val_rpt_1",
        "item_1_val_rpt_2",
        "item_1_val_rpt_3",
        "item_1_val_3",
        "item_2_val_1",
        "item_2_val_rpt_1",
        "item_2_val_rpt_2",
        "item_2_val_rpt_3",
        "item_2_val_3",
    ];

    let ts = TestStruct::default();
    assert_eq!(ts.list_fields(), expected);
}

#[test]
fn test_expected_defaults() {
    let ts = TestStruct::default();
    assert_eq!(
        ts.field_value("enumstr_field"),
        Some(td0_core::TD0Value::Text(ENUM_STR_VALUES[0].to_string())),
        "Should be zeroth element."
    );
    assert_eq!(
        ts.field_value("i16_bounded"),
        Some(td0_core::TD0Value::I16(0i16)),
        "Default 0 shouldn't be clamped."
    );
    assert_eq!(
        ts.field_value("i8_bounded_min_only"),
        Some(td0_core::TD0Value::I8(0i8)),
        "Default 0 shouldn't be clamped."
    );
    assert_eq!(
        ts.field_value("i8_bounded_max_only"),
        Some(td0_core::TD0Value::I8(-45i8)),
        "Default 0 clamped to max."
    );
    assert_eq!(
        ts.field_value("slice"),
        Some(td0_core::TD0Value::Slice(Box::new([0u8; 3]))),
        "Default slice is 0-filled."
    );
    assert_eq!(
        ts.field_value("td0_decimal_bounded"),
        Some(td0_core::TD0Value::Decimal(20.0)),
        "Decimal should clamp default 0 to field min."
    );
    assert_eq!(
        ts.field_value("space_padded_text"),
        Some(td0_core::TD0Value::Text(String::new())),
        "Default is pad byte (' '), which should be trimmed."
    );
    assert_eq!(
        ts.field_value_raw("space_padded_text"),
        Some(td0_core::TD0ValueRaw::Slice(Box::new([0x20; 16])))
    );
    assert_eq!(
        ts.field_value("zero_padded_text"),
        Some(td0_core::TD0Value::Text(String::new())),
        "Default is pad byte ('\\0'), which should be trimmed."
    );
    assert_eq!(
        ts.field_value_raw("zero_padded_text"),
        Some(td0_core::TD0ValueRaw::Slice(Box::new([0; 16])))
    );
    assert_eq!(
        ts.field_value("u16_unbounded"),
        Some(td0_core::TD0Value::U16(0)),
        "Default 0 shouldn't be clamped."
    );
    assert_eq!(
        ts.field_value("u16_bounded"),
        Some(td0_core::TD0Value::U16(257)),
        "Default 0 should be clamped to field min."
    );
    assert_eq!(
        ts.field_value("u32_unbounded"),
        Some(td0_core::TD0Value::U32(0)),
        "Default 0 shouldn't be clamped."
    );
    assert_eq!(
        ts.field_value("u32_bounded"),
        Some(td0_core::TD0Value::U32(65537)),
        "Default 0 should be clamped to field min."
    );
    assert_eq!(
        ts.field_value("u8_unbounded"),
        Some(td0_core::TD0Value::U8(0)),
        "Default 0 shouldn't be clamped."
    );
    assert_eq!(
        ts.field_value("u8_bounded_min_only"),
        Some(td0_core::TD0Value::U8(27)),
        "Default 0 should be clamped to field min."
    );
    assert_eq!(
        ts.field_value("volume_field"),
        Some(td0_core::TD0Value::Decimal(0f32)),
        "Default 0 shouldn't be clamped."
    );
}

#[test]
fn test_repeat_fields() {
    let ts = TestStruct::default();
    assert_eq!(
        ts.field_value("repeat_field_1_val"),
        Some(td0_core::TD0Value::U8(0)),
        "Repeat field `repeat_field_1_val` should be retrievable."
    );
    assert_eq!(
        ts.field_value("repeat_field_2_val"),
        Some(td0_core::TD0Value::U8(0)),
        "Repeat field `repeat_field_2_val` should be retrievable."
    );
    assert_eq!(
        ts.field_value("repeat_field_3_val"),
        Some(td0_core::TD0Value::U8(0)),
        "Repeat field `repeat_field_3_val` should be retrievable."
    );
    assert_eq!(
        ts.field_value("item_1_val_1"),
        Some(td0_core::TD0Value::U8(0)),
        "Repeat section field `item_1_val_1` should be retrievable."
    );
    assert_eq!(
        ts.field_value("item_1_val_rpt_1"),
        Some(td0_core::TD0Value::U8(0)),
        "Repeat section field `item_1_val_rpt_1` should be retrievable."
    );
    assert_eq!(
        ts.field_value("item_1_val_rpt_2"),
        Some(td0_core::TD0Value::U8(0)),
        "Repeat section field `item_1_val_rpt_2` should be retrievable."
    );
    assert_eq!(
        ts.field_value("item_1_val_rpt_3"),
        Some(td0_core::TD0Value::U8(0)),
        "Repeat section field `item_1_val_rpt_3` should be retrievable."
    );
    assert_eq!(
        ts.field_value("item_1_val_3"),
        Some(td0_core::TD0Value::U8(0)),
        "Repeat section field `item_1_val_3` should be retrievable."
    );
    assert_eq!(
        ts.field_value("item_2_val_1"),
        Some(td0_core::TD0Value::U8(0)),
        "Repeat section field `item_2_val_1` should be retrievable."
    );
    assert_eq!(
        ts.field_value("item_2_val_rpt_1"),
        Some(td0_core::TD0Value::U8(0)),
        "Repeat section field `item_2_val_rpt_1` should be retrievable."
    );
    assert_eq!(
        ts.field_value("item_2_val_rpt_2"),
        Some(td0_core::TD0Value::U8(0)),
        "Repeat section field `item_2_val_rpt_2` should be retrievable."
    );
    assert_eq!(
        ts.field_value("item_2_val_rpt_3"),
        Some(td0_core::TD0Value::U8(0)),
        "Repeat section field `item_2_val_rpt_3` should be retrievable."
    );
    assert_eq!(
        ts.field_value("item_2_val_3"),
        Some(td0_core::TD0Value::U8(0)),
        "Repeat section field `item_2_val_3` should be retrievable."
    );
}

#[test]
#[expect(clippy::too_many_lines)]
fn test_setters() {
    let mut ts = TestStruct::default();
    // enumstr_field: #[td0_field(field_type = "EnumStr", ...)]
    ts.set_field_value(
        "enumstr_field",
        td0_core::TD0Value::Text("Test_3_EnumStr".to_string()),
    )
    .expect("Can't set `enumstr_field`");
    assert_eq!(
        ts.field_value("enumstr_field"),
        Some(td0_core::TD0Value::Text("Test_3_EnumStr".to_string())),
        "`enumstr_field` Setter should have set the correct value."
    );
    assert_eq!(
        ts.field_value_raw("enumstr_field"),
        Some(td0_core::TD0ValueRaw::U8(3)),
        "`enumstr_field` Setter should have set the correct raw value."
    );
    let raw_before = ts.field_value_raw("enumstr_field");
    assert_matches!(
        ts.set_field_value(
            "enumstr_field",
            td0_core::TD0Value::Text("INVALID".to_string())
        ),
        Err(TD0Error::InvalidInput(..)),
        "`enumstr_field` Setter should error on invalid value."
    );
    assert_eq!(
        raw_before,
        ts.field_value_raw("enumstr_field"),
        "`enumstr_field` Setter should not change value on Error."
    );

    // i16_bounded #[td0_field(field_type = "I16", min = -2048, max = 50)]
    assert_matches!(
        ts.set_field_value("i16_bounded", td0_core::TD0Value::I16(-2048)),
        Ok { .. },
        "`i16_bounded` Bounded field setter should accept min value."
    );
    assert_matches!(
        ts.set_field_value("i16_bounded", td0_core::TD0Value::I16(50)),
        Ok { .. },
        "`i16_bounded` Bounded field setter should accept max value."
    );
    let val_before = ts.field_value("i16_bounded");
    assert_matches!(
        ts.set_field_value("i16_bounded", td0_core::TD0Value::I16(-2049)),
        Err(TD0Error::OutOfRange(..)),
        "`i16_bounded` Bounded field setter should Error if less than min."
    );
    assert_matches!(
        ts.set_field_value("i16_bounded", td0_core::TD0Value::I16(51)),
        Err(TD0Error::OutOfRange(..)),
        "`i16_bounded` Bounded field setter should Error if greater than max."
    );
    assert_eq!(
        val_before,
        ts.field_value("i16_bounded"),
        "`i16_bounded` Setter should not change value on Error."
    );

    // i8_bounded_min_only: #[td0_field(field_type = "I8", min = -45)]
    assert_matches!(
        ts.set_field_value("i8_bounded_min_only", td0_core::TD0Value::I8(-45)),
        Ok { .. },
        "`i8_bounded_min_only` Bounded_min_only field setter should accept min value."
    );
    assert_matches!(
        ts.set_field_value("i8_bounded_min_only", td0_core::TD0Value::I8(-46)),
        Err(TD0Error::OutOfRange(..)),
        "`i8_bounded_min_only` Bounded_min_only field setter should Error if less than min."
    );
    let val_before = ts.field_value("i8_bounded_min_only");
    assert_eq!(
        val_before,
        ts.field_value("i8_bounded_min_only"),
        "`i8_bounded_min_only` Setter should not change value on Error."
    );

    // i8_bounded_max_only: #[td0_field(field_type = "I8", max = -45)]
    assert_matches!(
        ts.set_field_value("i8_bounded_max_only", td0_core::TD0Value::I8(-45)),
        Ok { .. },
        "`i8_bounded_max_only` Bounded_max_only field setter should accept max value."
    );
    assert_matches!(
        ts.set_field_value("i8_bounded_max_only", td0_core::TD0Value::I8(-44)),
        Err(TD0Error::OutOfRange(..)),
        "`i8_bounded_max_only` Bounded_max_only field setter should Error if greater than max."
    );

    // slice: [u8; 3]: #[td0_field(field_type = "Slice")]
    assert_matches!(
        ts.set_field_value("slice", td0_core::TD0Value::Slice(Box::new([1u8; 3]))),
        Err(TD0Error::ReadOnlyField(..)),
        "Slice shouldn't be settable with set_field_value()."
    );

    // td0_decimal_bounded: #[td0_field(field_type = "TD0Decimal", min = 20.0, max = 260.0)]
    assert_matches!(
        ts.set_field_value("td0_decimal_bounded", td0_core::TD0Value::Decimal(20.0)),
        Ok { .. },
        "`td0_decimal_bounded` Bounded field setter should accept min value."
    );
    assert_eq!(
        ts.field_value("td0_decimal_bounded"),
        Some(td0_core::TD0Value::Decimal(20.0)),
        "`td0_decimal_bounded` Bounded field setter should set value."
    );
    assert_matches!(
        ts.set_field_value("td0_decimal_bounded", td0_core::TD0Value::Decimal(260.0)),
        Ok { .. },
        "`td0_decimal_bounded` Bounded field setter should accept max value."
    );
    assert_eq!(
        ts.field_value("td0_decimal_bounded"),
        Some(td0_core::TD0Value::Decimal(260.0)),
        "`td0_decimal_bounded` Bounded field setter should set value."
    );
    let val_before = ts.field_value("td0_decimal_bounded");
    assert_matches!(
        ts.set_field_value("td0_decimal_bounded", td0_core::TD0Value::Decimal(19.9999)),
        Err(TD0Error::OutOfRangeDecimal(..)),
        "`td0_decimal_bounded` Bounded field setter should Error if less than min."
    );
    assert_matches!(
        ts.set_field_value("td0_decimal_bounded", td0_core::TD0Value::Decimal(260.1)),
        Err(TD0Error::OutOfRangeDecimal(..)),
        "`td0_decimal_bounded` Bounded field setter should Error if greater than max."
    );
    assert_eq!(
        val_before,
        ts.field_value("td0_decimal_bounded"),
        "`td0_decimal_bounded` Setter should not change value on Error."
    );

    // td0_decimal_u8: #[td0_field(field_type = "TD0Decimal", min = 0.1, max = 8.0)]
    assert_matches!(
        ts.set_field_value("td0_decimal_u8", td0_core::TD0Value::Decimal(0.1)),
        Ok { .. },
        "`td0_decimal_u8` Bounded field setter should accept min value."
    );
    assert_eq!(
        ts.field_value("td0_decimal_u8"),
        Some(td0_core::TD0Value::Decimal(0.1)),
        "`td0_decimal_u8` Bounded field setter should set value."
    );
    assert_matches!(
        ts.set_field_value("td0_decimal_u8", td0_core::TD0Value::Decimal(8.0)),
        Ok { .. },
        "`td0_decimal_u8` Bounded field setter should accept max value."
    );
    assert_eq!(
        ts.field_value("td0_decimal_u8"),
        Some(td0_core::TD0Value::Decimal(8.0)),
        "`td0_decimal_u8` Bounded field setter should set value."
    );
    let val_before = ts.field_value("td0_decimal_u8");
    assert_matches!(
        ts.set_field_value("td0_decimal_u8", td0_core::TD0Value::Decimal(0.08)),
        Err(TD0Error::OutOfRangeDecimal(..)),
        "`td0_decimal_u8` Bounded field setter should Error if less than min."
    );
    assert_matches!(
        ts.set_field_value("td0_decimal_u8", td0_core::TD0Value::Decimal(8.01)),
        Err(TD0Error::OutOfRangeDecimal(..)),
        "`td0_decimal_u8` Bounded field setter should Error if greater than max."
    );
    assert_eq!(
        val_before,
        ts.field_value("td0_decimal_u8"),
        "`td0_decimal_u8` Setter should not change value on Error."
    );

    // space_padded_text: [u8; 16] : #[td0_field(field_type = "Text", pad_byte = 0x20)]
    assert_matches!(
        ts.set_field_value(
            "space_padded_text",
            td0_core::TD0Value::Text("Testing: 1..2..3".to_string())
        ),
        Ok { .. },
        "`space_padded_text` Setter should accept maximum length string."
    );
    assert_eq!(
        ts.field_value("space_padded_text"),
        Some(td0_core::TD0Value::Text("Testing: 1..2..3".to_string())),
        "`space_padded_text` Setter should set value."
    );
    assert_matches!(
        ts.set_field_value(
            "space_padded_text",
            td0_core::TD0Value::Text("One char too long".to_string())
        ),
        Err(TD0Error::OutOfRange(..)),
        "`space_padded_text` Setter should Error if string too long."
    );
    assert_eq!(
        ts.field_value("space_padded_text"),
        Some(td0_core::TD0Value::Text("Testing: 1..2..3".to_string())),
        "`space_padded_text` Setter should not change value on Error."
    );
    assert_matches!(
        ts.set_field_value(
            "space_padded_text",
            td0_core::TD0Value::Text("Short test".to_string())
        ),
        Ok { .. },
        "`space_padded_text` Setter should accept less than maximum length string."
    );
    assert_eq!(
        ts.field_value("space_padded_text"),
        Some(td0_core::TD0Value::Text("Short test".to_string())),
        "`space_padded_text` Setter should strip padding from a short string."
    );
    assert_eq!(
        ts.field_value_raw("space_padded_text"),
        Some(td0_core::TD0ValueRaw::Slice(Box::new(
            b"Short test      ".to_owned()
        ))),
        "`space_padded_text` Setter should pad a short string."
    );
    assert_matches!(
        ts.set_field_value("space_padded_text", td0_core::TD0Value::Text(String::new())),
        Ok { .. },
        "`space_padded_text` Setter should set value on an empty string."
    );
    assert_eq!(
        ts.field_value("space_padded_text"),
        Some(td0_core::TD0Value::Text(String::new())),
        "`space_padded_text` Getter should return empty string."
    );
    assert_eq!(
        ts.field_value_raw("space_padded_text"),
        Some(td0_core::TD0ValueRaw::Slice(Box::new(
            b"                ".to_owned()
        ))),
        "`space_padded_text` Setter should set pad an empty string."
    );

    // zero_padded_text: [u8; 16]: #[td0_field(field_type = "Text", pad_byte = 0)]
    assert_matches!(
        ts.set_field_value(
            "zero_padded_text",
            td0_core::TD0Value::Text("Short test".to_string())
        ),
        Ok { .. },
        "`space_padded_text` Setter should pad a short string."
    );
    assert_eq!(
        ts.field_value("zero_padded_text"),
        Some(td0_core::TD0Value::Text("Short test".to_string())),
        "`zero_padded_text` Getter should strip padding from a padded string."
    );

    assert_eq!(
        ts.field_value_raw("zero_padded_text"),
        Some(td0_core::TD0ValueRaw::Slice(Box::new(
            b"Short test\0\0\0\0\0\0".to_owned()
        ))),
        "`zero_padded_text` Setter should set pad a short string."
    );

    // u16_bounded: #[td0_field(field_type = "U16", min = 257, max = 65500)]
    assert_matches!(
        ts.set_field_value("u16_bounded", td0_core::TD0Value::U16(257)),
        Ok { .. },
        "`u16_bounded` Bounded field setter should accept min value."
    );
    assert_matches!(
        ts.set_field_value("u16_bounded", td0_core::TD0Value::U16(65500)),
        Ok { .. },
        "`u16_bounded` Bounded field setter should accept max value."
    );
    assert_eq!(
        ts.field_value("u16_bounded"),
        Some(td0_core::TD0Value::U16(65500)),
        "`u16_bounded` Setter/Getter round-trip should return the same value."
    );
    let val_before = ts.field_value("u16_bounded");
    assert_matches!(
        ts.set_field_value("u16_bounded", td0_core::TD0Value::U16(256)),
        Err(TD0Error::OutOfRange(..)),
        "`u16_bounded` Bounded field setter should Error if less than min."
    );
    assert_matches!(
        ts.set_field_value("u16_bounded", td0_core::TD0Value::U16(65501)),
        Err(TD0Error::OutOfRange(..)),
        "`u16_bounded` Bounded field setter should Error if greater than max."
    );
    assert_eq!(
        val_before,
        ts.field_value("u16_bounded"),
        "`u16_bounded` Setter should not change value on Error."
    );

    // u32_bounded: #[td0_field(field_type = "U16", min = 65537, max = 655_377)]
    assert_matches!(
        ts.set_field_value("u32_bounded", td0_core::TD0Value::U32(65537)),
        Ok { .. },
        "`u32_bounded` Bounded field setter should accept min value."
    );
    assert_matches!(
        ts.set_field_value("u32_bounded", td0_core::TD0Value::U32(655_377)),
        Ok { .. },
        "`u32_bounded` Bounded field setter should accept max value."
    );
    assert_eq!(
        ts.field_value("u32_bounded"),
        Some(td0_core::TD0Value::U32(655_377)),
        "`u32_bounded` Setter/Getter round-trip should return the same value."
    );
    let val_before = ts.field_value("u32_bounded");
    assert_matches!(
        ts.set_field_value("u32_bounded", td0_core::TD0Value::U32(65536)),
        Err(TD0Error::OutOfRange(..)),
        "`u32_bounded` Bounded field setter should Error if less than min."
    );
    assert_matches!(
        ts.set_field_value("u32_bounded", td0_core::TD0Value::U32(655_378)),
        Err(TD0Error::OutOfRange(..)),
        "`u32_bounded` Bounded field setter should Error if greater than max."
    );
    assert_eq!(
        val_before,
        ts.field_value("u32_bounded"),
        "`u32_bounded` Setter should not change value on Error."
    );

    //u8_unbounded: #[td0_field(field_type = "U8")]
    assert_matches!(
        ts.set_field_value("u8_unbounded", td0_core::TD0Value::U8(u8::MIN)),
        Ok { .. },
        "`u8_unbounded` Bounded field setter should accept min value."
    );
    assert_matches!(
        ts.set_field_value("u8_unbounded", td0_core::TD0Value::U8(u8::MAX)),
        Ok { .. },
        "`u8_unbounded` Bounded field setter should accept max value."
    );
    assert_eq!(
        ts.field_value("u8_unbounded"),
        Some(td0_core::TD0Value::U8(u8::MAX)),
        "`u8_unbounded` Setter/Getter round-trip should return the same value."
    );

    // u8_bounded_min_only: #[td0_field(field_type = "U8", min = 27)]
    assert_matches!(
        ts.set_field_value("u8_bounded_min_only", td0_core::TD0Value::U8(27)),
        Ok { .. },
        "`u8_bounded_min_only` Bounded_min_only field setter should accept min value."
    );
    assert_matches!(
        ts.set_field_value("u8_bounded_min_only", td0_core::TD0Value::U8(26)),
        Err(TD0Error::OutOfRange(..)),
        "`u8_bounded_min_only` Bounded_min_only field setter should Error if less than min."
    );
    let val_before = ts.field_value("u8_bounded_min_only");
    assert_eq!(
        val_before,
        ts.field_value("u8_bounded_min_only"),
        "`u8_bounded_min_only` Setter should not change value on Error."
    );
    assert_matches!(
        ts.set_field_value("u8_bounded_min_only", td0_core::TD0Value::U8(u8::MAX)),
        Ok { .. },
        "`u8_bounded_min_only` Bounded_min_only field setter should accept u8 max value."
    );

    // u8_bounded_max_only: u8 #[td0_field(field_type = "U8", max = 142)]
    assert_matches!(
        ts.set_field_value("u8_bounded_max_only", td0_core::TD0Value::U8(142)),
        Ok { .. },
        "`u8_bounded_max_only` Bounded_max_only field setter should accept max value."
    );
    assert_matches!(
        ts.set_field_value("u8_bounded_max_only", td0_core::TD0Value::U8(143)),
        Err(TD0Error::OutOfRange(..)),
        "`u8_bounded_max_only` Bounded_max_only field setter should Error if greater than max."
    );
    assert_matches!(
        ts.set_field_value("u8_bounded_max_only", td0_core::TD0Value::U8(u8::MIN)),
        Ok { .. },
        "`u8_bounded_max_only` Bounded_max_only field setter should accept u8 min value."
    );

    // volume_field: #[td0_field(field_type = "Volume")]
    assert_matches!(
        ts.set_field_value("volume_field", td0_core::TD0Value::Decimal(Volume::MIN)),
        Ok { .. },
        "`volume_field` Bounded field setter should accept min value."
    );
    assert_eq!(
        ts.field_value("volume_field"),
        Some(td0_core::TD0Value::Decimal(Volume::MIN)),
        "`volume_field` Bounded field setter should set value."
    );

    assert_matches!(
        ts.set_field_value("volume_field", td0_core::TD0Value::Decimal(Volume::MAX)),
        Ok { .. },
        "`volume_field` Bounded field setter should accept max value."
    );
    assert_eq!(
        ts.field_value("volume_field"),
        Some(td0_core::TD0Value::Decimal(Volume::MAX)),
        "`volume_field` Bounded field setter should set value."
    );

    let val_before = ts.field_value("volume_field");
    assert_matches!(
        ts.set_field_value(
            "volume_field",
            td0_core::TD0Value::Decimal(Volume::MIN - 0.01)
        ),
        Err(TD0Error::OutOfRangeDecimal(..)),
        "`volume_field` Bounded field setter should Error if less than min."
    );
    assert_matches!(
        ts.set_field_value(
            "volume_field",
            td0_core::TD0Value::Decimal(Volume::MAX + 0.01)
        ),
        Err(TD0Error::OutOfRangeDecimal(..)),
        "`volume_field` Bounded field setter should Error if greater than max."
    );
    assert_eq!(
        val_before,
        ts.field_value("volume_field"),
        "`volume_field` Setter should not change value on Error."
    );
    assert_matches!(
        ts.set_field_value(
            "volume_field",
            td0_core::TD0Value::Decimal(
                Volume::try_from(Volume::MINUS_INF_I16)
                    .expect(TEST_BROKEN)
                    .into()
            )
        ),
        Ok { .. },
        "`volume_field` Setter should accept Volume::MINUS_INF_I16."
    );
    assert_eq!(
        ts.field_value("volume_field"),
        Some(td0_core::TD0Value::Decimal(Volume::MINUS_INF_FLOAT)),
        "`volume_field` Getter should return Volume::MINUS_INF."
    );

    assert_eq!(
        ts.field_value("volume_field")
            .expect(TEST_BROKEN)
            .to_string(),
        Volume::MINUS_INF_DISPLAY,
        "`volume_field` to_string() should handle Volume::MINUS_INF."
    );
    assert_matches!(
        ts.set_field_value("volume_field", td0_core::TD0Value::Decimal(-12.5)),
        Ok { .. },
        "`volume_field` Setter should accept in range value."
    );
    assert_eq!(
        ts.field_value("volume_field"),
        Some(td0_core::TD0Value::Decimal(-12.5)),
        "`volume_field` Setter/getter handle value in range."
    );
    assert_eq!(
        ts.field_value_raw("volume_field"),
        Some(td0_core::TD0ValueRaw::I16(-125)),
        "`volume_field` native storage value should be correct."
    );

    // repeat_field_1_val, _2_val, _3_val #[td0_field(field_type = "U8")]
    assert_matches!(
        ts.set_field_value("repeat_field_1_val", td0_core::TD0Value::U8(23)),
        Ok { .. },
        "`repeat_field_1_val` Setter works on repeat field."
    );
    assert_matches!(
        ts.set_field_value("repeat_field_2_val", td0_core::TD0Value::U8(24)),
        Ok { .. },
        "`repeat_field_2_val` Setter works on repeat field."
    );
    assert_matches!(
        ts.set_field_value("repeat_field_3_val", td0_core::TD0Value::U8(25)),
        Ok { .. },
        "`repeat_field_3_val` Setter works on repeat field."
    );
    assert_eq!(
        ts.field_value("repeat_field_1_val"),
        Some(td0_core::TD0Value::U8(23)),
        "`repeat_field_1_val` Getter works on repeat field and set/getters are independent."
    );
    assert_eq!(
        ts.field_value("repeat_field_2_val"),
        Some(td0_core::TD0Value::U8(24)),
        "`repeat_field_2_val` Getter works on repeat field and set/getters are independent."
    );
    assert_eq!(
        ts.field_value("repeat_field_3_val"),
        Some(td0_core::TD0Value::U8(25)),
        "`repeat_field_3_val` Getter works on repeat field and set/getters are independent."
    );

    // item_1_val_1, item_2_val_1 #[repeat_section(count=2, prefix_format = "item_{}")]
    assert_matches!(
        ts.set_field_value("item_1_val_1", td0_core::TD0Value::U8(23)),
        Ok { .. },
        "`item_1_val_1` Setter works on repeat section field."
    );
    assert_matches!(
        ts.set_field_value("item_2_val_1", td0_core::TD0Value::U8(24)),
        Ok { .. },
        "`item_2_val_1` Setter works on repeat section field."
    );
    assert_eq!(
        ts.field_value("item_1_val_1"),
        Some(td0_core::TD0Value::U8(23)),
        "`item_1_val_1` Getter works on repeat section field and set/getters are independent."
    );
    assert_eq!(
        ts.field_value("item_2_val_1"),
        Some(td0_core::TD0Value::U8(24)),
        "`item_2_val_1` Getter works on repeat section field and set/getters are independent."
    );
}

#[test]
fn test_text_getter_into_setter() {
    let mut source = TestStruct::default();
    let mut dest = TestStruct::default();

    let val = TD0Value::new_text_value("Test 3 Text");
    source
        .set_field_value("space_padded_text", val.clone())
        .expect("can set Text field");
    dest.set_field_value(
        "space_padded_text",
        source
            .field_value("space_padded_text")
            .expect("must be able to retrieve value."),
    )
    .expect("setter from getter works.");
    assert_eq!(
        dest.field_value("space_padded_text"),
        Some(val),
        "Setting `space_padded_text` from a getter should work."
    );
}

#[test]
fn test_u8_getter_into_setter_raw() {
    let mut source = TestStruct::default();
    let mut dest = TestStruct::default();

    let val_raw: TD0ValueRaw = TD0ValueRaw::U8(95);
    source
        .set_field_value_raw("u8_unbounded", val_raw.clone())
        .unwrap();
    dest.set_field_value_raw(
        "u8_unbounded",
        source.field_value_raw("u8_unbounded").unwrap(),
    )
    .unwrap();
    assert_eq!(
        dest.field_value_raw("u8_unbounded"),
        Some(val_raw),
        "Setting raw `u8_unbounded` from a raw getter should work."
    );
}

#[test]
fn test_u16_getter_into_setter_raw() {
    let mut source = TestStruct::default();
    let mut dest = TestStruct::default();

    let val_raw: TD0ValueRaw = TD0ValueRaw::U16(95);
    source
        .set_field_value_raw("u16_unbounded", val_raw.clone())
        .unwrap();
    dest.set_field_value_raw(
        "u16_unbounded",
        source.field_value_raw("u16_unbounded").unwrap(),
    )
    .unwrap();
    assert_eq!(
        dest.field_value_raw("u16_unbounded"),
        Some(val_raw),
        "Setting raw `u16_unbounded` from a raw getter should work."
    );
}

#[test]
fn test_volume_getter_into_setter() {
    let mut source = TestStruct::default();
    let mut dest = TestStruct::default();

    let val: TD0Value = TD0Value::Decimal(-4.5);
    source.set_field_value("volume_field", val.clone()).unwrap();
    dest.set_field_value("volume_field", source.field_value("volume_field").unwrap())
        .unwrap();
    assert_eq!(
        dest.field_value("volume_field"),
        Some(val),
        "Setting raw `volume_field` from a raw getter should work."
    );
}

#[test]
fn test_text_getter_into_setter_raw() {
    let mut source = TestStruct::default();
    let mut dest = TestStruct::default();

    let val_raw: TD0ValueRaw = TD0ValueRaw::Slice(Box::new(*b"This is a test. "));
    source
        .set_field_value_raw("space_padded_text", val_raw.clone())
        .unwrap();
    dest.set_field_value_raw(
        "space_padded_text",
        source.field_value_raw("space_padded_text").unwrap(),
    )
    .unwrap();
    assert_eq!(
        dest.field_value_raw("space_padded_text"),
        Some(val_raw),
        "Setting raw `space_padded_text` from a raw getter should work."
    );
}

#[test]
fn test_enumstr_getter_into_setter() {
    let mut source = TestStruct::default();
    let mut dest = TestStruct::default();

    let val = TD0Value::new_text_value("Test_3_EnumStr");
    source
        .set_field_value("enumstr_field", val.clone())
        .expect("can set EnumStr field");
    dest.set_field_value(
        "enumstr_field",
        source
            .field_value("enumstr_field")
            .expect("must be able to retrieve value."),
    )
    .expect("setter from getter works.");
    assert_eq!(
        dest.field_value("enumstr_field"),
        Some(val),
        "Setting `enumstr_field` from a getter should work."
    );
}

#[test]
fn test_enumstr_getter_into_setter_raw() {
    let mut source = TestStruct::default();
    let mut dest = TestStruct::default();

    let val_raw: TD0ValueRaw = TD0ValueRaw::U8(0);
    source
        .set_field_value_raw("enumstr_field", val_raw.clone())
        .unwrap();
    dest.set_field_value_raw(
        "enumstr_field",
        source.field_value_raw("enumstr_field").unwrap(),
    )
    .unwrap();
    assert_eq!(
        dest.field_value_raw("enumstr_field"),
        Some(val_raw),
        "Setting raw `enumstr_field` from a raw getter should work."
    );
}

#[test]
fn test_setter_datatype_errors() {
    let mut ts = TestStruct::default();

    // EnumStr field:
    let result: TD0Result<()> = ts.set_field_value("space_padded_text", TD0Value::U8(23));
    assert_matches!(result, Err(TD0Error::DataType(_)));
    let err_text = result.err().unwrap().to_string();
    assert!(
        err_text.contains("Expected Text"),
        "Got wrong message: \"{err_text}\""
    );
}

#[test]
fn test_setter_raw_datatype_errors() {
    let mut ts = TestStruct::default();

    let result: TD0Result<()> = ts.set_field_value_raw("space_padded_text", TD0ValueRaw::U8(23));
    assert_matches!(result, Err(TD0Error::DataType(_)));
    let err_text = result.err().unwrap().to_string();
    assert!(
        err_text.contains("Expected Slice"),
        "Got wrong message: \"{err_text}\""
    );
}
