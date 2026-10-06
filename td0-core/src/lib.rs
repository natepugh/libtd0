// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Core types and shared functionality for libtd0.
//! ## Feature flags
#![doc = document_features::document_features!()]

pub(crate) mod header;
pub(crate) mod helpers;
pub(crate) mod result;
pub(crate) mod type_impls;
pub(crate) mod types;

// ============================================================================
//                                   PUBLIC API
//      Items below this line are public (re-)exports constituting the public
//      API of this library.
//
//      Only items with public visibility are permitted here. Any change to
//      their signatures (for functions) or publicly accessible members
//      require updating the library version according to The Cargo Book's
//      [SemVer Compatibility] https://doc.rust-lang.org/cargo/reference/semver.html
//      guidelines.
//
pub use header::{OFFSET_BYTES_REMAINING, TD0_MAGIC, TD0IdChunk, TD0ManifestTag, validate_id_tag};
pub use helpers::{
    calc_range_overlap, copy_ascii_str_to_u8_slice, copy_slice_to_native,
    copy_slice_to_native_padded, in_range_inclusive, try_u32_from_usize, usize_from_u32,
};
pub use result::{TD0Error, TD0Result};
pub use types::{
    ChunkManifest, IntEncodedDecimal, TD0BackupType, TD0ChunkItem, TD0DeviceModel, TD0File,
    TD0Manifest, TD0Value, TD0ValueRaw, Volume,
};

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
        let max_as_i16: i16 = (Volume::MAX.mul(10.0f32).round() as i32)
            .try_into()
            .expect("Can't convert Volume::MAX to i16.");
        let actual = Volume::try_from(max_as_i16).expect("Can't create Volume");
        let expected = Volume(Volume::MAX);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_volume_min_from_i16() {
        let min_as_i16: i16 = (Volume::MIN.mul(10.0f32).round() as i32)
            .try_into()
            .expect("Can't convert Volume::MIN to i16.");
        let actual = Volume::try_from(min_as_i16).expect("Can't create Volume");
        let expected = Volume(Volume::MIN);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_volume_minus_inf_from_i16() {
        let actual = Volume::try_from(Volume::MINUS_INF_I16).expect("Can't create Volume");
        let expected = Volume(Volume::MINUS_INF_FLOAT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_volume_to_string() {
        let v1 = Volume(Volume::MINUS_INF_FLOAT);
        assert_eq!(
            v1.to_string().as_str(),
            Volume::MINUS_INF_DISPLAY,
            "Test {} Volume to string.",
            Volume::MINUS_INF_DISPLAY
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
                result::OutOfRangeErrorTD0Decimal::new(Volume::MIN, Volume::MAX)
            )),
            "Test proper Err from < Volume::MIN"
        );

        let actual = Volume::try_from(61i16);
        assert_eq!(
            actual,
            Err(result::TD0Error::OutOfRangeDecimal(
                result::OutOfRangeErrorTD0Decimal::new(Volume::MIN, Volume::MAX)
            )),
            "Test proper Err from > Volume::MAX"
        );
    }
    #[test]
    fn test_volume_from_str() {
        let actual = Volume::from_str("-60.2");
        assert_eq!(
            actual,
            Err(result::TD0Error::OutOfRangeDecimal(
                result::OutOfRangeErrorTD0Decimal::new(Volume::MIN, Volume::MAX)
            )),
            "Test proper Err from < Volume::MIN"
        );

        let actual = Volume::from_str(Volume::MIN.to_string().as_str()).expect("Test broken.");
        assert_eq!(
            actual,
            Volume(Volume::MIN),
            "Test proper conversion for Volume::MIN."
        );

        let actual = Volume::from_str("6.1");
        assert_eq!(
            actual,
            Err(result::TD0Error::OutOfRangeDecimal(
                result::OutOfRangeErrorTD0Decimal::new(Volume::MIN, Volume::MAX)
            )),
            "Test proper Err from > Volume::MAX"
        );

        let actual = Volume::from_str(Volume::MAX.to_string().as_str()).expect("Test broken.");
        assert_eq!(
            actual,
            Volume(Volume::MAX),
            "Test proper conversion for Volume::MAX."
        );

        let actual = Volume::from_str(Volume::MINUS_INF_DISPLAY).expect("Test broken.");
        assert_eq!(
            actual,
            Volume(Volume::MINUS_INF_FLOAT),
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
            copy_ascii_str_to_u8_slice("I'm 17 chars long", &mut dest, 0u8),
            Err(result::TD0Error::OutOfRange(result::OutOfRangeError::new(
                0, 16
            ))),
            "Error if source string is too long."
        );

        debug_assert_matches!(
            copy_ascii_str_to_u8_slice("I'm not äscii", &mut dest, 0u8),
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
