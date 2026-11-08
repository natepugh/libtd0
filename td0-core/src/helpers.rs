// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use core::ops::Range;

use num_traits::Bounded;
#[cfg(feature = "serde")]
use serde::Serializer;
use zerocopy::{LittleEndian, U32};

use crate::{
    ascii_string::is_ascii_printable,
    result::{TD0Error, TD0Result},
};

pub const SZ_MD5_DIGEST: usize = 16;

pub fn bytes_or_error<F>(
    buf: &[u8],
    range: std::ops::Range<usize>,
    error_factory: F,
) -> TD0Result<&[u8]>
where
    F: FnOnce() -> TD0Error,
{
    buf.get(range).ok_or_else(error_factory)
}

pub fn calc_md5_checksum(bytes_objs: &[&[u8]]) -> [u8; SZ_MD5_DIGEST] {
    use md5::Digest;
    use zerocopy::IntoBytes;

    let mut hasher: md5::Md5 = md5::Md5::new();
    for bytes in bytes_objs {
        hasher.update(bytes);
    }

    let mut checksum: [u8; SZ_MD5_DIGEST] = [0u8; SZ_MD5_DIGEST];
    checksum.copy_from_slice(hasher.finalize().as_bytes());
    checksum
}

pub fn calc_range_overlap<T>(r1: Range<T>, r2: Range<T>) -> Option<Range<T>>
where
    T: Ord,
{
    let start = r1.start.max(r2.start);
    let end = r1.end.min(r2.end);

    if start >= end { None } else { Some(start..end) }
}

pub fn hex_encode(bytes: &[u8]) -> String {
    use core::fmt::Write;
    bytes.iter().fold(String::new(), |mut output, b| {
        write!(output, "{b:02x}").unwrap();
        output
    })
}

pub fn clamp_optional<T>(val: T, min: Option<T>, max: Option<T>) -> T
where
    T: Bounded + Ord,
{
    val.clamp(
        min.unwrap_or_else(|| T::min_value()),
        max.unwrap_or_else(|| T::max_value()),
    )
}

/// Copies a source &str into a destination u8 Slice, padding any excess with `pad_byte`.
///
/// # Errors
/// - [`TD0Error::OutOfRange`] - If the length of `src` is greater than the length of `dest`.
/// - [`TD0Error::InvalidInput`] - If `src` isn't composed entirely of ASCII-printable characters.
pub fn copy_ascii_str_to_u8_slice(src: &str, dest: &mut [u8], pad_byte: u8) -> TD0Result<()> {
    if src.len() > dest.len() {
        return Err(TD0Error::out_of_range_error_from_usize(0, dest.len()));
    }
    if !is_ascii_printable(src) {
        return Err(TD0Error::InvalidInput(
            "a printable ASCII string is required.".to_string(),
        ));
    }

    let src_bytes = src.as_bytes();
    dest[src_bytes.len()..].fill(pad_byte);
    dest[..src_bytes.len()].copy_from_slice(src_bytes);
    Ok(())
}

/// Copies a source byte slice into a destination slice.
///
/// # Errors
/// - [`TD0Error::InvalidInput`] - If `src` and `dest` aren't of identical lengths.
pub fn copy_slice_to_native(src: &[u8], dest: &mut [u8]) -> TD0Result<()> {
    if src.len() != dest.len() {
        return Err(TD0Error::InvalidInput(
            "source length != destination length.".to_string(),
        ));
    }
    dest.clone_from_slice(src);
    Ok(())
}

/// Copies a source byte slice into a destination slice, with padding.
///
/// # Errors
/// - [`TD0Error::InvalidInput`] - If `src` is longer than `dest`.
pub fn copy_slice_to_native_padded(src: &[u8], dest: &mut [u8], pad: u8) -> TD0Result<()> {
    if src.len() > dest.len() {
        return Err(TD0Error::InvalidInput(
            "source length != destination length.".to_string(),
        ));
    }

    dest[src.len()..].fill(pad);
    dest[..src.len()].clone_from_slice(src);
    Ok(())
}

pub fn in_range_inclusive<T>(val: T, min: Option<T>, max: Option<T>) -> bool
where
    T: PartialOrd + Bounded + Copy,
{
    (min.unwrap_or_else(|| T::min_value())..=max.unwrap_or_else(|| T::max_value())).contains(&val)
}

#[cfg(feature = "serde")]
pub fn serde_checksum_bytes_to_string<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(hex_encode(bytes).as_str())
}

/// Lossy-extract ASCII text from `source`, trimming `pad_val` bytes from the
/// end prior to conversion.
pub fn text_from_u8_array(source: &[u8], pad_val: u8) -> String {
    for pos in (0..source.len()).rev() {
        if source[pos] != pad_val {
            return String::from_utf8_lossy(&source[..=pos]).to_string();
        }
    }

    String::new()
}

/// Try to convert a usize to a u32, returning a `TD0Error` on failure.
///
/// # Errors
/// [`TD0Error::OutOfRange`] - If the usize is larger than `u32::MAX`.
pub fn try_u32_from_usize(val: usize) -> TD0Result<u32> {
    u32::try_from(val).map_err(|_| TD0Error::out_of_range_error_u32())
}

/// Try to convert a usize to a zerocopy U32<LittleEndian>,
/// returning a `TD0Error` on failure.
///
/// # Errors
/// [`TD0Error::OutOfRange`] - If the usize is larger than `u32::MAX`.
pub fn try_u32_le_from_usize(val: usize) -> TD0Result<U32<LittleEndian>> {
    Ok(U32::from(
        u32::try_from(val).map_err(|_| TD0Error::out_of_range_error_u32())?,
    ))
}
