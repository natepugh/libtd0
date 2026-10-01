// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod header;
pub mod result;
pub mod type_impls;

use core::ops::Range;

use num_traits::Bounded;
use result::TD0Result;
use zerocopy::{LittleEndian, U32};

#[cfg(feature = "serde")]
use serde::Serializer;

pub const SZ_MD5_DIGEST: usize = 16;
pub const VOLUME_MIN: f32 = -60.0f32;
pub const VOLUME_MAX: f32 = 6.0f32;
pub const VOLUME_MINUS_INF_FLOAT: f32 = f32::NEG_INFINITY;
pub const VOLUME_MINUS_INF_I16: i16 = -601;
pub const VOLUME_MINUS_INF_DISPLAY: &str = "-inf";

pub trait TD0ChunkItem: Send + Sync + core::fmt::Debug {
    fn as_bytes(&self) -> &[u8];
    fn field_options(&self, field: &str) -> Option<&'static [&'static str]>;
    fn field_type(&self, field: &str) -> Option<&'static str>;
    fn field_value(&self, field: &str) -> Option<TD0Value>;
    fn field_value_raw(&self, field: &str) -> Option<TD0ValueRaw>;
    fn list_fields(&self) -> &'static [&'static str];
    fn set_field_value(&mut self, field: &str, value: &TD0Value) -> TD0Result<()>;
    fn set_field_value_raw(&mut self, field: &str, raw_value: &TD0ValueRaw) -> TD0Result<()>;
}
pub trait TD0File: Send + Sync + core::fmt::Debug {
    fn add_chunk(&mut self, chunk_name: &str, num_items: usize) -> TD0Result<()>;
    fn remove_chunk(&mut self, chunk_name: &str) -> TD0Result<()>;
    fn chunk_item_replace(
        &mut self,
        chunk_name: &str,
        item_index: usize,
        source: &dyn TD0ChunkItem,
    ) -> TD0Result<()>;
    fn chunk_items_copy(
        &mut self,
        chunk_name: &str,
        source_index: usize,
        dest_index: usize,
    ) -> TD0Result<()>;
    fn chunk_items_swap(
        &mut self,
        chunk_name: &str,
        index_1: usize,
        index_2: usize,
    ) -> TD0Result<()>;
    fn chunk_items_reorder(&mut self, chunk_name: &str, new_order: &[usize]) -> TD0Result<()>;
    fn finalize(&mut self) -> TD0Result<()>;
    fn try_from_bytes(bytes: &[u8]) -> TD0Result<Self>
    where
        Self: Sized;
    fn try_into_bytes(self) -> TD0Result<Vec<u8>>;
    fn chunk_item(&self, chunk_name: &str, item_index: usize) -> TD0Result<&dyn TD0ChunkItem>;
    fn chunk_item_mut(
        &mut self,
        chunk_name: &str,
        item_index: usize,
    ) -> TD0Result<&mut dyn TD0ChunkItem>;
    fn chunk_item_owned(
        &self,
        chunk_name: &str,
        item_index: usize,
    ) -> TD0Result<Box<dyn TD0ChunkItem>>;
    fn chunk_item_default(&self, chunk_name: &str) -> Option<Box<dyn TD0ChunkItem>>;
    fn chunk_item_raw(&self, chunk_name: &str, item_index: usize) -> TD0Result<&[u8]>;
    fn chunk_num_items(&self, chunk_name: &str) -> Option<usize>;
    fn chunk_pos(&self, chunk_name: &str) -> Option<usize>;
    fn chunk_raw(&self, chunk_name: &str) -> Option<&[u8]>;
    fn chunk_size(&self, chunk_name: &str) -> Option<usize>;
    fn list_chunks(&self) -> Vec<String>;
    fn manifest(&self) -> TD0Result<TD0Manifest>;
    fn new() -> Self
    where
        Self: Sized;
    fn to_bytes(&self) -> Vec<u8>;
    fn validate_load(&self) -> TD0Result<()>;
    fn validate_save(&self) -> TD0Result<()>;
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChunkManifest {
    name: String,
    pos: usize,
    size: usize,
    num_items: usize,
    item_size: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct IntEncodedDecimal(pub f32);

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum TD0BackupType {
    Kit,
    System,
    Unknown,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum TD0DeviceModel {
    SPDSXPro,
    Unknown,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Debug)]
pub struct TD0Manifest {
    backup_name: String,
    backup_type: TD0BackupType,

    #[cfg_attr(
        feature = "serde",
        serde(serialize_with = "serde_checksum_bytes_to_string")
    )]
    checksum_actual: [u8; 16],

    #[cfg_attr(
        feature = "serde",
        serde(serialize_with = "serde_checksum_bytes_to_string")
    )]
    checksum_calculated: [u8; 16],
    device_model: TD0DeviceModel,
    device_firmware_version: String,
    device_firmware_build: String,
    device_serial: String,
    size_actual: usize,
    size_calculated: usize,
    chunks: Vec<ChunkManifest>,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum TD0Value {
    /// This value is rounded to one decimal place (0.1) upon display and
    /// before setting the value of any field. To prevent unexpected results,
    /// ensure that your f32 value has already been rounded to one decimal
    /// place _before_ creating a TD0Value::Decimal.
    Decimal(f32),
    I16(i16),
    I8(i8),
    Slice(Box<[u8]>),
    Text(String),
    U16(u16),
    U32(u32),
    U8(u8),
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum TD0ValueRaw {
    I16(i16),
    I8(i8),
    Slice(Box<[u8]>),
    U16(u16),
    U32(u32),
    U8(u8),
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Volume(pub f32);
impl core::fmt::Display for Volume {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.0.eq(&VOLUME_MINUS_INF_FLOAT) {
            write!(f, "{}", VOLUME_MINUS_INF_DISPLAY)
        } else {
            write!(f, "{:.1}", self.0)
        }
    }
}

pub fn calc_range_overlap<T>(r1: Range<T>, r2: Range<T>) -> Option<Range<T>>
where
    T: Ord,
{
    let start = r1.start.max(r2.start);
    let end = r1.end.min(r2.end);

    if start > end { None } else { Some(start..end) }
}

pub fn checksum_bytes_to_string(val: &[u8]) -> String {
    val.iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>()
}

pub fn copy_ascii_str_to_u8_slice(src: &String, dest: &mut [u8], pad_byte: u8) -> TD0Result<()> {
    if src.len() > dest.len() {
        return Err(result::TD0Error::OutOfRange(result::OutOfRangeError::new(
            0,
            i64::try_from(dest.len()).expect("dest.len() is not larger than i64."),
        )));
    }
    if !src.is_ascii() {
        return Err(result::TD0Error::InvalidInput(
            "an ASCII string is required.".to_string(),
        ));
    }

    let src_bytes = src.as_bytes();
    dest[src_bytes.len()..].fill(pad_byte);
    dest[..src_bytes.len()].copy_from_slice(src_bytes);
    Ok(())
}

pub fn copy_slice_to_native(src: &[u8], dest: &mut [u8]) -> TD0Result<()> {
    if src.len() != dest.len() {
        return Err(result::TD0Error::InvalidInput(
            "source length != destination length.".to_string(),
        ));
    }
    dest.clone_from_slice(src);
    Ok(())
}

pub fn copy_slice_to_native_padded(src: &[u8], dest: &mut [u8], pad: u8) -> TD0Result<()> {
    if src.len() > dest.len() {
        return Err(result::TD0Error::InvalidInput(
            "source length != destination length.".to_string(),
        ));
    }
    dest[src.len()..].fill(pad);
    dest[..src.len()].clone_from_slice(src);
    Ok(())
}

pub fn in_range_inclusive<T>(val: T, min: Option<T>, max: Option<T>) -> bool
where
    T: PartialOrd + Bounded,
{
    (min.unwrap_or(T::min_value())..=max.unwrap_or(T::max_value())).contains(&val)
}

#[cfg(feature = "serde")]
fn serde_checksum_bytes_to_string<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(checksum_bytes_to_string(bytes).as_str())
}

pub fn try_u32_from_usize(val: usize) -> TD0Result<u32> {
    u32::try_from(val).map_err(|_| result::create_u32_oob_error())
}

pub fn try_u32_le_from_usize(val: usize) -> TD0Result<U32<LittleEndian>> {
    Ok(U32::from(
        u32::try_from(val).map_err(|_| result::create_u32_oob_error())?,
    ))
}

pub fn usize_from_u32(val: u32) -> usize {
    usize::try_from(val).expect("platform usize is >= 32 bits")
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::debug_assert_matches;
    use core::ops::Mul as _; // Required for f32.mul()
    use core::str::FromStr;

    #[test]
    fn test_chunk_item_val_from_u8_array() {
        let val = TD0Value::text_from_u8_array(" Test \0\0\0\0".as_bytes(), &0);
        assert_eq!(
            val,
            TD0Value::Text(" Test ".to_string()),
            "Should keep space chars before and after token."
        );

        let val = TD0Value::text_from_u8_array("Test With Spaces".as_bytes(), &0x20);
        assert_eq!(
            val,
            TD0Value::Text("Test With Spaces".to_string()),
            "Should keep all non-space chars af end of string."
        );
    }

    #[test]
    fn test_in_range_inclusive_u8() {
        assert!(in_range_inclusive(u8::MIN, None, None));
        assert!(in_range_inclusive(u8::MAX, None, None));
        assert!(in_range_inclusive(1u8, Some(1u8), Some(1u8)));
        assert!(!in_range_inclusive(2u8, Some(1u8), Some(1u8)));
        assert!(!in_range_inclusive(0u8, Some(1u8), Some(1u8)));
    }

    #[test]
    fn test_in_range_inclusive_decimal() {
        assert!(in_range_inclusive(f32::MIN, None, None));
        assert!(in_range_inclusive(f32::MAX, None, None));
        assert!(in_range_inclusive(23.0, Some(22.9), Some(23.1)));
        assert!(!in_range_inclusive(f32::NEG_INFINITY, None, None));
        assert!(!in_range_inclusive(f32::INFINITY, None, None));
    }

    #[test]
    fn test_int_encoded_decimal_from_u16() {
        let actual = IntEncodedDecimal::from(200u16);
        assert_eq!(actual.val(), 20.0f32, "Properly converts u16");
    }

    #[test]
    fn test_int_encoded_decimal_into_u16() {
        let actual = IntEncodedDecimal(20.0);
        assert_eq!(
            u16::try_from(actual).expect("Or test is broken."),
            200u16,
            "Properly converts to u16"
        );
    }

    #[test]
    fn test_volume_from_i16() {
        let actual = Volume::try_from(12i16).expect("Can't create Volume");
        let expected = Volume(1.2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_volume_max_from_i16() {
        let max_as_i16: i16 = (VOLUME_MAX.mul(10.0f32).round() as i32)
            .try_into()
            .expect("Can't convert VOLUME_MAX to i16.");
        let actual = Volume::try_from(max_as_i16).expect("Can't create Volume");
        let expected = Volume(VOLUME_MAX);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_volume_min_from_i16() {
        let min_as_i16: i16 = (VOLUME_MIN.mul(10.0f32).round() as i32)
            .try_into()
            .expect("Can't convert VOLUME_MIN to i16.");
        let actual = Volume::try_from(min_as_i16).expect("Can't create Volume");
        let expected = Volume(VOLUME_MIN);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_volume_minus_inf_from_i16() {
        let actual = Volume::try_from(VOLUME_MINUS_INF_I16).expect("Can't create Volume");
        let expected = Volume(VOLUME_MINUS_INF_FLOAT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_volume_to_string() {
        let v1 = Volume(VOLUME_MINUS_INF_FLOAT);
        assert_eq!(
            v1.to_string().as_str(),
            VOLUME_MINUS_INF_DISPLAY,
            "Test {} Volume to string.",
            VOLUME_MINUS_INF_DISPLAY
        );

        let v2 = Volume(-25.0f32);
        assert_eq!(
            v2.to_string().as_str(),
            "-25.0",
            "Test a negative Volume value."
        );

        let v3 = Volume(4.5f32);
        assert_eq!(
            v3.to_string().as_str(),
            "4.5",
            "Test a positive Volume value."
        );
    }

    #[test]
    fn test_volume_from_i16_out_of_range() {
        let actual = Volume::try_from(-602i16);
        assert_eq!(
            actual,
            Err(result::TD0Error::OutOfRangeDecimal(
                result::OutOfRangeErrorTD0Decimal::new(VOLUME_MIN, VOLUME_MAX)
            )),
            "Test proper Err from < VOLUME_MIN"
        );

        let actual = Volume::try_from(61i16);
        assert_eq!(
            actual,
            Err(result::TD0Error::OutOfRangeDecimal(
                result::OutOfRangeErrorTD0Decimal::new(VOLUME_MIN, VOLUME_MAX)
            )),
            "Test proper Err from > VOLUME_MAX"
        );
    }
    #[test]
    fn test_volume_from_str() {
        let actual = Volume::from_str("-60.2");
        assert_eq!(
            actual,
            Err(result::TD0Error::OutOfRangeDecimal(
                result::OutOfRangeErrorTD0Decimal::new(VOLUME_MIN, VOLUME_MAX)
            )),
            "Test proper Err from < VOLUME_MIN"
        );

        let actual = Volume::from_str(VOLUME_MIN.to_string().as_str()).expect("Test broken.");
        assert_eq!(
            actual,
            Volume(VOLUME_MIN),
            "Test proper conversion for VOLUME_MIN."
        );

        let actual = Volume::from_str("6.1");
        assert_eq!(
            actual,
            Err(result::TD0Error::OutOfRangeDecimal(
                result::OutOfRangeErrorTD0Decimal::new(VOLUME_MIN, VOLUME_MAX)
            )),
            "Test proper Err from > VOLUME_MAX"
        );

        let actual = Volume::from_str(VOLUME_MAX.to_string().as_str()).expect("Test broken.");
        assert_eq!(
            actual,
            Volume(VOLUME_MAX),
            "Test proper conversion for VOLUME_MAX."
        );

        let actual = Volume::from_str(VOLUME_MINUS_INF_DISPLAY).expect("Test broken.");
        assert_eq!(
            actual,
            Volume(VOLUME_MINUS_INF_FLOAT),
            "Test proper conversion for -Infinity"
        );
    }

    #[test]
    fn test_copy_ascii_str_to_u8_slice() {
        let mut dest = [0u8; 16];
        let src: String = "I'm 16chars long".to_string();
        let result = copy_ascii_str_to_u8_slice(&src, &mut dest, 0x0);
        assert_eq!(result, Ok(()), "Should not return an Err.");
        assert_eq!(
            String::from_utf8(dest.to_vec())
                .expect("Test broken")
                .as_str(),
            "I'm 16chars long",
            "Should properly copy to dest."
        );

        let src: String = "Needs padding".to_string();
        let result = copy_ascii_str_to_u8_slice(&src, &mut dest, b'.');
        assert_eq!(result, Ok(()), "Testing padding. Should not return an Err.");
        assert_eq!(
            String::from_utf8(dest.to_vec())
                .expect("Test broken")
                .as_str(),
            "Needs padding...",
            "Should properly pad dest bytes."
        );

        assert_eq!(
            copy_ascii_str_to_u8_slice(&"I'm 17 chars long".to_string(), &mut dest, 0u8),
            Err(result::TD0Error::OutOfRange(result::OutOfRangeError::new(
                0, 16
            ))),
            "Error if source string is too long."
        );

        debug_assert_matches!(
            copy_ascii_str_to_u8_slice(&"I'm not äscii".to_string(), &mut dest, 0u8),
            Err(result::TD0Error::InvalidInput(..)),
            "Error if non-ascii chars are in source.."
        );
    }

    #[test]
    fn test_copy_slice_to_native_padded() {
        let mut dest = [0u8; 16];
        let src: &[u8] = "I'm 16chars long".as_bytes();

        let result = copy_slice_to_native_padded(src, &mut dest, 0);
        assert_eq!(result, Ok(()), "Should not return an error.");
        assert_eq!(&dest, src);

        let result = copy_slice_to_native_padded("Needs padding".as_bytes(), &mut dest, b'.');
        assert_eq!(
            result,
            Ok(()),
            "Testing padding. Should not return an error."
        );
        assert_eq!(&dest, "Needs padding...".as_bytes());

        debug_assert_matches!(
            copy_slice_to_native_padded("I'm 17 chars long".as_bytes(), &mut dest, 0u8),
            Err(result::TD0Error::InvalidInput(..)),
            "Error if source slice is too long."
        );
    }

    #[test]
    fn test_td0_value_display_impl() {
        let val = TD0Value::Slice(Box::new([0u8, 1u8, 23u8, 27u8, 5u8]));
        assert_eq!(val.to_string().as_str(), "[0, 1, 23, 27, 5]");

        let val = TD0Value::Decimal(91.1);
        assert_eq!(val.to_string().as_str(), "91.1");

        let val = TD0Value::I16(-512);
        assert_eq!(val.to_string().as_str(), "-512");

        let val = TD0Value::I8(-12i8);
        assert_eq!(val.to_string().as_str(), "-12");

        let val = TD0Value::Text("This is a test.".to_string());
        assert_eq!(val.to_string().as_str(), "This is a test.");

        let val = TD0Value::U16(544);
        assert_eq!(val.to_string().as_str(), "544");

        let val = TD0Value::U8(221);
        assert_eq!(val.to_string().as_str(), "221");

        let val = TD0Value::U32(299000);
        assert_eq!(val.to_string().as_str(), "299000");
    }
}
