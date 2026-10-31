// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use core::ops::Range;

use num_traits::Bounded;
#[cfg(feature = "serde")]
use serde::Serializer;
use zerocopy::{LittleEndian, U32};

use crate::result::{TD0Error, TD0Result};

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
    for bytes in bytes_objs.iter() {
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

pub fn checksum_bytes_to_string(val: &[u8]) -> String {
    val.iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>()
}

pub fn clamp_optional<T>(val: T, min: Option<T>, max: Option<T>) -> T
where
    T: Bounded + Ord,
{
    val.clamp(min.unwrap_or(T::min_value()), max.unwrap_or(T::max_value()))
}

pub fn copy_ascii_str_to_u8_slice(src: &str, dest: &mut [u8], pad_byte: u8) -> TD0Result<()> {
    if src.len() > dest.len() {
        return Err(TD0Error::out_of_range_error_from_usize(0, dest.len()));
    }
    if !src.is_ascii() {
        return Err(TD0Error::InvalidInput(
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
        return Err(TD0Error::InvalidInput(
            "source length != destination length.".to_string(),
        ));
    }
    dest.clone_from_slice(src);
    Ok(())
}

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
    T: PartialOrd + Bounded,
{
    (min.unwrap_or(T::min_value())..=max.unwrap_or(T::max_value())).contains(&val)
}

#[cfg(feature = "serde")]
pub fn serde_checksum_bytes_to_string<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(checksum_bytes_to_string(bytes).as_str())
}

pub fn text_from_u8_array(source: &[u8], pad_val: u8) -> String {
    for pos in (0..source.len()).rev() {
        if source[pos] != pad_val {
            return String::from_utf8_lossy(&source[..=pos]).to_string();
        }
    }

    "".to_string()
}

pub fn try_u32_from_usize(val: usize) -> TD0Result<u32> {
    u32::try_from(val).map_err(|_| TD0Error::out_of_range_error_u32())
}

pub fn try_u32_le_from_usize(val: usize) -> TD0Result<U32<LittleEndian>> {
    Ok(U32::from(
        u32::try_from(val).map_err(|_| TD0Error::out_of_range_error_u32())?,
    ))
}

pub fn usize_from_u32(val: u32) -> usize {
    usize::try_from(val).expect("platform usize is >= 32 bits")
}
