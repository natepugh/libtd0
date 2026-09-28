// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod header;
pub mod result;

use crate::result::TD0Error;
use core::ops::{Div, Mul, Range};
use num_traits::Bounded;
use result::TD0Result;
#[cfg(feature = "serde")]
use serde::Serializer;
use zerocopy::{ByteOrder, I16, U16};

pub const SZ_MD5_DIGEST: usize = 16;
pub const VOLUME_MIN: f32 = -60.0f32;
pub const VOLUME_MAX: f32 = 6.0f32;
pub const VOLUME_MINUS_INF_FLOAT: f32 = f32::NEG_INFINITY;
pub const VOLUME_MINUS_INF_I16: i16 = -601;
pub const VOLUME_MINUS_INF_DISPLAY: &str = "-inf";

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

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum TD0DeviceModel {
    SPDSXPro,
    Unknown,
}

impl TryFrom<&str> for TD0DeviceModel {
    type Error = TD0Error;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        match val {
            "SPDSXPro" => Ok(Self::SPDSXPro),
            "Unknown" => Ok(Self::Unknown),
            _ => Err(TD0Error::InvalidInput(
                "input string is not a valid TD0DeviceModel.".to_string(),
            )),
        }
    }
}

impl core::fmt::Display for TD0DeviceModel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let strval = match self {
            Self::SPDSXPro => "SPDSXPro",
            Self::Unknown => "Unknown",
        };
        write!(f, "{strval}")
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum TD0BackupType {
    Kit,
    System,
    Unknown,
}

impl core::fmt::Display for TD0BackupType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let strval = match self {
            Self::Kit => "Kit",
            Self::System => "System",
            Self::Unknown => "Unknown",
        };
        write!(f, "{strval}")
    }
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

impl ChunkManifest {
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn pos(&self) -> usize {
        self.pos
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn num_items(&self) -> usize {
        self.num_items
    }
    pub fn item_size(&self) -> usize {
        self.item_size
    }

    pub fn new(name: String, pos: usize, size: usize, num_items: usize, item_size: usize) -> Self {
        Self {
            name,
            pos,
            size,
            num_items,
            item_size,
        }
    }
}

impl core::fmt::Display for ChunkManifest {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}:  pos: {}  size: {}  num_items: {}  item_size: {}",
            self.name, self.pos, self.size, self.num_items, self.item_size
        )
    }
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

impl TD0Manifest {
    pub fn backup_name(&self) -> String {
        self.backup_name.clone()
    }

    pub fn backup_type(&self) -> TD0BackupType {
        self.backup_type
    }

    pub fn checksum_actual(&self) -> String {
        checksum_bytes_to_string(&self.checksum_actual)
    }

    pub fn checksum_calculated(&self) -> String {
        checksum_bytes_to_string(&self.checksum_calculated)
    }

    pub fn device_model(&self) -> TD0DeviceModel {
        self.device_model
    }

    pub fn device_firmware_version(&self) -> String {
        self.device_firmware_version.clone()
    }

    pub fn device_firmware_build(&self) -> String {
        self.device_firmware_build.clone()
    }

    pub fn device_serial(&self) -> String {
        self.device_serial.clone()
    }

    pub fn size_actual(&self) -> usize {
        self.size_actual
    }

    pub fn size_calculated(&self) -> usize {
        self.size_calculated
    }

    pub fn chunks(&self) -> &[ChunkManifest] {
        &self.chunks
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "A TD0Manifest is composed of many items."
    )]
    pub fn new(
        backup_name: impl Into<String>,
        backup_type: impl Into<TD0BackupType>,
        checksum_actual: &[u8],
        checksum_calculated: &[u8],
        device_model: impl Into<TD0DeviceModel>,
        device_firmware_version: impl Into<String>,
        device_firmware_build: impl Into<String>,
        device_serial: impl Into<String>,
        size_actual: usize,
        size_calculated: usize,
        chunks: Vec<ChunkManifest>,
    ) -> Self {
        let mut my_checksum_actual: [u8; 16] = [0u8; 16];
        let input_slice_def = 0..checksum_actual.len().clamp(0, 16usize);
        my_checksum_actual
            .get_mut(input_slice_def.clone())
            .unwrap()
            .copy_from_slice(&checksum_actual[input_slice_def]);

        let mut my_checksum_calculated: [u8; 16] = [0u8; 16];
        let input_slice_def = 0..checksum_calculated.len().clamp(0, 16usize);
        my_checksum_calculated
            .get_mut(input_slice_def.clone())
            .unwrap()
            .copy_from_slice(&checksum_calculated[input_slice_def]);

        Self {
            backup_name: backup_name.into(),
            backup_type: backup_type.into(),
            checksum_actual: my_checksum_actual,
            checksum_calculated: my_checksum_calculated,
            device_model: device_model.into(),
            device_firmware_version: device_firmware_version.into(),
            device_firmware_build: device_firmware_build.into(),
            device_serial: device_serial.into(),
            size_actual,
            size_calculated,
            chunks,
        }
    }
}

pub fn checksum_bytes_to_string(val: &[u8]) -> String {
    val.iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>()
}

#[cfg(feature = "serde")]
fn serde_checksum_bytes_to_string<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(checksum_bytes_to_string(bytes).as_str())
}

impl core::fmt::Display for TD0Manifest {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Backup Name: {}\n\
             Backup Type: {:?}\n\
             Checksum from File   : {}\n\
             Checksum (calculated): {}\n\
             Device Model: {}\n\
             Device Firmware: {}\n\
             Device Firmware Build: {}\n\
             Device Serial: {}\n\
             Actual Size: {}\n\
             Calculated Size: {}\n\
             Chunks:
    {}",
            self.backup_name,
            self.backup_type,
            checksum_bytes_to_string(&self.checksum_actual),
            checksum_bytes_to_string(&self.checksum_calculated),
            self.device_model,
            self.device_firmware_version,
            self.device_firmware_build,
            self.device_serial,
            self.size_actual,
            self.size_calculated,
            self.chunks
                .iter()
                .map(|ch| format!("{}", ch))
                .collect::<Vec<String>>()
                .join("\n    "),
        )
    }
}

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
    fn new() -> TD0Result<Self>
    where
        Self: Sized;
    fn to_bytes(&self) -> Vec<u8>;
    fn validate_load(&self) -> TD0Result<()>;
    fn validate_save(&self) -> TD0Result<()>;
}

impl TD0Value {
    pub fn text_from_u8_array(source: &[u8], pad_val: &u8) -> Self {
        for pos in (0..source.len()).rev() {
            if source[pos] != *pad_val {
                return Self::Text(String::from_utf8_lossy(&source[..=pos]).to_string());
            }
        }

        Self::Text("".to_string())
    }
}

impl core::fmt::Display for TD0Value {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let repr: String = match self {
            Self::Slice(val) => format!("{val:?}"),
            Self::Decimal(val) => format!("{val:.1}"),
            Self::I16(val) => val.to_string(),
            Self::I8(val) => val.to_string(),
            Self::Text(val) => val.to_string(),
            Self::U16(val) => val.to_string(),
            Self::U32(val) => val.to_string(),
            Self::U8(val) => val.to_string(),
        };

        write!(f, "{}", repr)
    }
}

pub fn in_range_inclusive<T>(val: T, min: Option<T>, max: Option<T>) -> bool
where
    T: PartialOrd + Bounded,
{
    (min.unwrap_or(T::min_value())..=max.unwrap_or(T::max_value())).contains(&val)
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

pub fn try_new_bounded<T>(val: T, min: T, max: T) -> Option<T>
where
    T: PartialOrd + Bounded + Copy,
{
    if !in_range_inclusive(val, Some(min), Some(max)) {
        None
    } else {
        Some(val)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct IntEncodedDecimal(pub f32);

impl IntEncodedDecimal {
    pub fn val(&self) -> f32 {
        self.0
    }
    pub fn val_mut(&mut self) -> &f32 {
        &mut self.0
    }
}

impl core::fmt::Display for IntEncodedDecimal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}

impl core::str::FromStr for IntEncodedDecimal {
    type Err = result::TD0Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let selfobj = Self(s.parse::<f32>().map_err(|_| {
            result::TD0Error::InvalidInput("input string not parsable as a Decimal.".to_string())
        })?);
        Ok(selfobj)
    }
}

impl From<i8> for IntEncodedDecimal {
    fn from(val: i8) -> Self {
        Self(f32::from(val).div(10.0f32))
    }
}

impl TryFrom<IntEncodedDecimal> for i8 {
    type Error = <i8 as TryFrom<i32>>::Error;

    fn try_from(val: IntEncodedDecimal) -> Result<Self, Self::Error> {
        i8::try_from(val.0.mul(10.0f32).round() as i32)
    }
}

impl From<u8> for IntEncodedDecimal {
    fn from(val: u8) -> Self {
        Self(f32::from(val).div(10.0f32))
    }
}

impl TryFrom<IntEncodedDecimal> for u8 {
    type Error = <u8 as TryFrom<i32>>::Error;

    fn try_from(val: IntEncodedDecimal) -> Result<Self, Self::Error> {
        u8::try_from(val.0.mul(10.0f32).round() as i32)
    }
}

impl From<f32> for IntEncodedDecimal {
    fn from(val: f32) -> Self {
        Self(val)
    }
}

impl From<IntEncodedDecimal> for f32 {
    fn from(val: IntEncodedDecimal) -> f32 {
        val.0
    }
}

impl From<i32> for IntEncodedDecimal {
    fn from(val: i32) -> Self {
        Self((val as f32).div(10.0f32))
    }
}

impl From<IntEncodedDecimal> for i32 {
    fn from(val: IntEncodedDecimal) -> i32 {
        val.0.mul(10.0f32).round() as i32
    }
}

impl From<i16> for IntEncodedDecimal {
    fn from(val: i16) -> Self {
        Self(f32::from(val).div(10.0f32))
    }
}

impl TryFrom<IntEncodedDecimal> for i16 {
    type Error = <i16 as TryFrom<i32>>::Error;

    fn try_from(val: IntEncodedDecimal) -> Result<Self, Self::Error> {
        i16::try_from(val.0.mul(10.0f32).round() as i32)
    }
}

impl From<u16> for IntEncodedDecimal {
    fn from(val: u16) -> Self {
        Self(f32::from(val).div(10.0f32))
    }
}

impl TryFrom<IntEncodedDecimal> for u16 {
    type Error = <u16 as TryFrom<i32>>::Error;

    fn try_from(val: IntEncodedDecimal) -> Result<Self, Self::Error> {
        u16::try_from(val.0.mul(10.0f32).round() as i32)
    }
}

impl<T: ByteOrder> From<U16<T>> for IntEncodedDecimal {
    fn from(val: U16<T>) -> Self {
        Self(f32::from(val.get()).div(10.0f32))
    }
}

impl<T: ByteOrder> TryFrom<IntEncodedDecimal> for U16<T> {
    type Error = <u16 as TryFrom<i32>>::Error;

    fn try_from(val: IntEncodedDecimal) -> Result<Self, Self::Error> {
        let i32val = val.0.mul(10.0f32).round() as i32;
        Ok(U16::from(u16::try_from(i32val)?))
    }
}

impl<T: ByteOrder> From<I16<T>> for IntEncodedDecimal {
    fn from(val: I16<T>) -> Self {
        Self(f32::from(val.get()).div(10.0f32))
    }
}

impl<T: ByteOrder> TryFrom<IntEncodedDecimal> for I16<T> {
    type Error = <i16 as TryFrom<i32>>::Error;

    fn try_from(val: IntEncodedDecimal) -> Result<Self, Self::Error> {
        let i32val = val.0.mul(10.0f32).round() as i32;
        Ok(I16::from(i16::try_from(i32val)?))
    }
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

impl Volume {
    fn validate_impl_range(val: &f32) -> Result<(), result::TD0Error> {
        if (val.lt(&VOLUME_MIN) || val.gt(&VOLUME_MAX)) && val.ne(&VOLUME_MINUS_INF_FLOAT) {
            Err(result::TD0Error::OutOfRangeDecimal(
                result::OutOfRangeErrorTD0Decimal::new(VOLUME_MIN, VOLUME_MAX),
            ))
        } else {
            Ok(())
        }
    }

    pub fn validate(&self) -> Result<(), result::TD0Error> {
        Self::validate_impl_range(&self.0)
    }
}

impl core::str::FromStr for Volume {
    type Err = result::TD0Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case(VOLUME_MINUS_INF_DISPLAY) {
            Ok(Self(VOLUME_MINUS_INF_FLOAT))
        } else {
            let selfobj = Self(s.parse::<f32>().map_err(|_| {
                result::TD0Error::InvalidInput(
                    "input string not parsable as a Decimal.".to_string(),
                )
            })?);
            selfobj.validate()?;
            Ok(selfobj)
        }
    }
}

impl TryFrom<f32> for Volume {
    type Error = result::TD0Error;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        let selfobj = Self(value);
        selfobj.validate()?;
        Ok(selfobj)
    }
}

impl From<Volume> for f32 {
    fn from(value: Volume) -> Self {
        value.0
    }
}

impl TryFrom<i16> for Volume {
    type Error = result::TD0Error;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let f32_value = if value == VOLUME_MINUS_INF_I16 {
            // Special handling for "-Infinity" value.
            VOLUME_MINUS_INF_FLOAT
        } else {
            f32::from(value).div(10.0f32)
        };
        let selfobj = Self(f32_value);
        selfobj.validate()?;
        Ok(selfobj)
    }
}

impl TryFrom<Volume> for i16 {
    type Error = <i16 as TryFrom<i32>>::Error;

    fn try_from(val: Volume) -> Result<Self, Self::Error> {
        if val.0 == VOLUME_MINUS_INF_FLOAT {
            Ok(VOLUME_MINUS_INF_I16)
        } else {
            i16::try_from(val.0.mul(10.0f32).round() as i32)
        }
    }
}

impl<T: ByteOrder> TryFrom<Volume> for I16<T> {
    type Error = <i16 as TryFrom<i32>>::Error;

    fn try_from(val: Volume) -> Result<Self, Self::Error> {
        if val.0 == VOLUME_MINUS_INF_FLOAT {
            Ok(I16::from(VOLUME_MINUS_INF_I16))
        } else {
            Ok(I16::from(i16::try_from(val.0.mul(10.0f32).round() as i32)?))
        }
    }
}

pub fn usize_from_u32(val: u32) -> usize {
    usize::try_from(val).expect("platform usize is >= 32 bits")
}

pub fn calc_range_overlap<T>(r1: Range<T>, r2: Range<T>) -> Option<Range<T>>
where
    T: Ord,
{
    let start = r1.start.max(r2.start);
    let end = r1.end.min(r2.end);

    if start > end { None } else { Some(start..end) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::debug_assert_matches;
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
