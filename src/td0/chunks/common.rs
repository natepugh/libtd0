use fastnum::D64 as Decimal;
use fastnum::decimal::Context as DecimalContext;
use libtd0_core::result::{TD0Error, TD0Result};
use std::collections::HashMap;
use std::convert::{From, TryInto};
use std::fmt;
use std::str::FromStr;
use zerocopy::{ByteOrder, FromBytes, I16, LittleEndian, U16, U32};
use zerocopy_derive::{IntoBytes, KnownLayout};

const SZ_HDR_EXTRA_DATA: usize = 4;
// Constants for fields.
const VOLUME_MIN: Decimal = Decimal::from_i8(-60i8);
const VOLUME_MAX: Decimal = Decimal::from_i8(6i8);
const VOLUME_MINUS_INF_DECIMAL: Decimal = Decimal::NEG_INFINITY;
const VOLUME_MINUS_INF_I16: i16 = -601;
const VOLUME_MINUS_INF_DISPLAY: &str = "-Infinity";

pub trait HasMinAndMax {
    const MIN: Self;
    const MAX: Self;
}

impl HasMinAndMax for u16 {
    const MIN: Self = u16::MIN;
    const MAX: Self = u16::MAX;
}

impl HasMinAndMax for u8 {
    const MIN: Self = u8::MIN;
    const MAX: Self = u8::MAX;
}

impl HasMinAndMax for i8 {
    const MIN: Self = i8::MIN;
    const MAX: Self = i8::MAX;
}

impl HasMinAndMax for i16 {
    const MIN: Self = i16::MIN;
    const MAX: Self = i16::MAX;
}

impl<T> HasMinAndMax for U16<T> {
    const MIN: Self = U16::MIN;
    const MAX: Self = U16::MAX;
}

pub fn validate_is_in_range<T: Ord + HasMinAndMax + Into<i128> + Copy>(
    val: T,
    min: Option<T>,
    max: Option<T>,
) -> TD0Result<()> {
    let min = min.unwrap_or(T::MIN);
    let max = max.unwrap_or(T::MAX);

    if val != val.clone().clamp(min, max) {
        Err(TD0Error::ConvertRangeError {
            min: min.into(),
            max: max.into(),
        })
    } else {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Chunk {
    pub pos: usize,
    pub first_item_pos: usize,
    pub num_items: usize,
    pub sz_item: usize,
}

impl Chunk {
    pub fn item_pos(&self, item_index: usize) -> Option<usize> {
        if item_index >= self.num_items {
            return None;
        }

        Some(self.first_item_pos + (self.sz_item * item_index))
    }

    pub fn item_pos_relative(&self, item_index: usize) -> Option<usize> {
        // Return the item pos relative to the start of the chunk header.
        if item_index >= self.num_items {
            return None;
        }
        Some(self.first_item_pos - self.pos + self.sz_item * item_index)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct IntEncodedDecimal(pub Decimal);

impl IntEncodedDecimal {
    pub fn get_val(&self) -> &Decimal {
        &self.0
    }
    pub fn get_val_mut(&mut self) -> &Decimal {
        &mut self.0
    }
}

impl fmt::Display for IntEncodedDecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}

impl FromStr for IntEncodedDecimal {
    type Err = TD0Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let selfobj = Self {
            0: Decimal::from_str(s, DecimalContext::default())
                .map_err(|_| TD0Error::ConvertFromStringError {
                    type_name: "IntEncodedDecimal".to_string(),
                    value: s.to_string(),
                })?
                .round(1),
        };
        Ok(selfobj)
    }
}

impl From<i8> for IntEncodedDecimal {
    fn from(val: i8) -> Self {
        Self {
            0: Decimal::from(val).div(Decimal::TEN).round(1),
        }
    }
}

impl TryFrom<IntEncodedDecimal> for i8 {
    type Error = <i8 as TryFrom<Decimal>>::Error;

    fn try_from(val: IntEncodedDecimal) -> Result<Self, Self::Error> {
        val.0.mul(Decimal::TEN).round(0).to_i8()
    }
}

impl From<u8> for IntEncodedDecimal {
    fn from(val: u8) -> Self {
        Self {
            0: Decimal::from(val).div(Decimal::TEN).round(1),
        }
    }
}

impl TryFrom<IntEncodedDecimal> for u8 {
    type Error = <u8 as TryFrom<Decimal>>::Error;

    fn try_from(val: IntEncodedDecimal) -> Result<Self, Self::Error> {
        val.0.mul(Decimal::TEN).round(0).to_u8()
    }
}

impl From<i16> for IntEncodedDecimal {
    fn from(val: i16) -> Self {
        Self {
            0: Decimal::from(val).div(Decimal::TEN).round(1),
        }
    }
}

impl TryFrom<IntEncodedDecimal> for i16 {
    type Error = <i16 as TryFrom<Decimal>>::Error;

    fn try_from(val: IntEncodedDecimal) -> Result<Self, Self::Error> {
        val.0.mul(Decimal::TEN).round(0).to_i16()
    }
}

impl From<u16> for IntEncodedDecimal {
    fn from(val: u16) -> Self {
        Self {
            0: Decimal::from(val).div(Decimal::TEN).round(1),
        }
    }
}

impl TryFrom<IntEncodedDecimal> for u16 {
    type Error = <u16 as TryFrom<Decimal>>::Error;

    fn try_from(val: IntEncodedDecimal) -> Result<Self, Self::Error> {
        val.0.mul(Decimal::TEN).round(0).to_u16()
    }
}

impl<T: ByteOrder> From<U16<T>> for IntEncodedDecimal {
    fn from(val: U16<T>) -> Self {
        Self {
            0: Decimal::from(val.get()).div(Decimal::TEN).round(1),
        }
    }
}

impl<T: ByteOrder> TryFrom<IntEncodedDecimal> for U16<T> {
    type Error = <u16 as TryFrom<Decimal>>::Error;

    fn try_from(val: IntEncodedDecimal) -> Result<Self, Self::Error> {
        Ok(U16::from(val.0.mul(Decimal::TEN).to_u16()?))
    }
}

impl<T: ByteOrder> From<I16<T>> for IntEncodedDecimal {
    fn from(val: I16<T>) -> Self {
        Self {
            0: Decimal::from(val.get()).div(Decimal::TEN).round(1),
        }
    }
}

impl<T: ByteOrder> TryFrom<IntEncodedDecimal> for I16<T> {
    type Error = <i16 as TryFrom<Decimal>>::Error;

    fn try_from(val: IntEncodedDecimal) -> Result<Self, Self::Error> {
        Ok(I16::from(val.0.mul(Decimal::TEN).round(0).to_i16()?))
    }
}

impl From<Decimal> for IntEncodedDecimal {
    fn from(val: Decimal) -> Self {
        Self { 0: val }
    }
}

impl From<IntEncodedDecimal> for Decimal {
    fn from(val: IntEncodedDecimal) -> Self {
        val.0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Volume(Decimal);
impl fmt::Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.eq(&VOLUME_MINUS_INF_DECIMAL) {
            write!(f, "{}", VOLUME_MINUS_INF_DISPLAY)
        } else {
            write!(f, "{:.1}", self.0)
        }
    }
}

impl Volume {
    fn validate_impl_range(val: &Decimal) -> Result<(), TD0Error> {
        if (val.lt(&VOLUME_MIN) || val.gt(&VOLUME_MAX)) && val.ne(&VOLUME_MINUS_INF_DECIMAL) {
            Err(TD0Error::ConvertRangeError {
                min: VOLUME_MIN
                    .to_i128()
                    .expect("Should be able to convert val to i128."),
                max: VOLUME_MAX
                    .to_i128()
                    .expect("Should be able to convert val to i128."),
            })
        } else {
            Ok(())
        }
    }

    pub fn validate(&self) -> Result<(), TD0Error> {
        Self::validate_impl_range(&self.0)
    }
}

impl FromStr for Volume {
    type Err = TD0Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case(VOLUME_MINUS_INF_DISPLAY) {
            Ok(Self {
                0: VOLUME_MINUS_INF_DECIMAL.clone(),
            })
        } else {
            let selfobj = Self {
                0: Decimal::from_str(s, DecimalContext::default())
                    .map_err(|_| TD0Error::ConvertFromStringError {
                        type_name: "Volume".to_string(),
                        value: s.to_string(),
                    })?
                    .round(1),
            };
            selfobj.validate()?;
            Ok(selfobj)
        }
    }
}

impl TryFrom<i16> for Volume {
    type Error = TD0Error;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let dec_value = if value == VOLUME_MINUS_INF_I16 {
            // Special handling for "-Infinity" value.
            VOLUME_MINUS_INF_DECIMAL
        } else {
            Decimal::from_f64(f64::from(value) / 10.0f64).round(1)
        };
        let selfobj = Self { 0: dec_value };
        selfobj.validate()?;
        Ok(selfobj)
    }
}

impl TryInto<u16> for Volume {
    type Error = <Decimal as TryInto<u16>>::Error;

    fn try_into(self) -> Result<u16, Self::Error> {
        let myval: Decimal = self.0.mul(Decimal::TEN).round(0);
        let converted: u16 = myval.try_into()?;
        Ok(converted)
    }
}

impl<T> TryInto<I16<T>> for Volume
where
    T: zerocopy::ByteOrder,
{
    type Error = <Decimal as TryInto<i16>>::Error;

    fn try_into(self) -> Result<I16<T>, Self::Error> {
        if self.0 == VOLUME_MINUS_INF_DECIMAL {
            // Special handling for "-Infinity" value.
            Ok(I16::from(VOLUME_MINUS_INF_I16))
        } else {
            let myval: i16 = self.0.try_into()?;
            Ok(myval.into())
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ChunkItemValue {
    EnumStr(&'static str),
    I16(i16),
    I8(i8),
    Slice(Box<[u8]>),
    TD0Decimal(IntEncodedDecimal),
    Text(String),
    U16(u16),
    U8(u8),
    Volume(Volume),
}

impl ChunkItemValue {
    pub fn text_from_u8_array(source: &[u8], pad_val: &u8) -> Self {
        for pos in (0..source.len()).rev() {
            if source[pos] != *pad_val {
                return Self::Text(String::from_utf8_lossy(&source[..=pos]).to_string());
            }
        }

        Self::Text("".to_string())
    }
}

#[derive(Debug, PartialEq)]
pub enum ChunkItemValueRaw {
    I16(i16),
    I8(i8),
    Slice(Box<[u8]>),
    U16(u16),
    U8(u8),
}

impl fmt::Display for ChunkItemValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr: String = match self {
            ChunkItemValue::TD0Decimal(val) => val.to_string(),
            ChunkItemValue::U16(val) => val.to_string(),
            ChunkItemValue::I8(val) => val.to_string(),
            ChunkItemValue::I16(val) => val.to_string(),
            ChunkItemValue::U8(val) => val.to_string(),
            ChunkItemValue::Text(val) => val.to_string(),
            ChunkItemValue::EnumStr(val) => val.to_string(),
            ChunkItemValue::Volume(val) => val.to_string(),
            ChunkItemValue::Slice(val) => format!("{val:?}"),
        };

        write!(f, "{}", repr)
    }
}

pub trait ChunkItem: std::fmt::Debug {
    fn get_value(&self, field: &str) -> Option<ChunkItemValue>;
    fn get_value_raw(&self, field: &str) -> Option<ChunkItemValueRaw>;
    fn set_value(&mut self, field: &str, value: &ChunkItemValue) -> TD0Result<()>;
    fn set_value_raw(&mut self, field: &str, value: &ChunkItemValueRaw) -> TD0Result<()>;

    fn from_bytes(bytes: &[u8], chunk_name: Option<&str>) -> TD0Result<Self>
    where
        Self: FromBytes + Sized,
    {
        let selfobj = Self::read_from_bytes(&bytes[..size_of::<Self>()]).map_err(|_| {
            TD0Error::InvalidChunkError {
                chunk_name: chunk_name.unwrap_or("Unknown").to_string(),
            }
        })?;
        Ok(selfobj)
    }

    fn copy_from_bytes(bytes: &[u8], chunk_name: Option<&str>) -> TD0Result<Self>
    where
        Self: Clone + FromBytes + Sized,
    {
        let selfobj = Self::read_from_bytes(&bytes[..size_of::<Self>()]).map_err(|_| {
            TD0Error::InvalidChunkError {
                chunk_name: chunk_name.unwrap_or("Unknown").to_string(),
            }
        })?;
        Ok(selfobj.clone())
    }

    fn get_values(&self) -> HashMap<String, ChunkItemValue> {
        let mut values: HashMap<String, ChunkItemValue> = HashMap::new();

        for fld in self.get_fields().iter() {
            values.insert(
                String::from(*fld),
                self.get_value(*fld)
                    .unwrap_or(ChunkItemValue::Text("Unknown".to_string())),
            );
        }
        values
    }

    fn get_fields(&self) -> &'static [&'static str];
}

#[derive(FromBytes, IntoBytes, KnownLayout)]
#[repr(C, packed)]
pub struct ChunkHeader {
    pub num_items: U32<LittleEndian>,
    pub item_size: U32<LittleEndian>,
    pub first_item_offset: U32<LittleEndian>,
    unknown_header_data: [u8; SZ_HDR_EXTRA_DATA],
}

impl ChunkHeader {
    pub fn get_num_items(&self) -> u32 {
        return self.num_items.get();
    }

    pub fn get_item_size(&self) -> u32 {
        return self.item_size.get();
    }

    pub fn get_first_item_offset(&self) -> u32 {
        return self.first_item_offset.get();
    }

    pub fn get_unknown_header_data(&self) -> &[u8; SZ_HDR_EXTRA_DATA] {
        &self.unknown_header_data
    }

    pub fn get_unknown_header_data_mut(&mut self) -> &[u8; SZ_HDR_EXTRA_DATA] {
        &mut self.unknown_header_data
    }
}

impl fmt::Display for ChunkHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "num_items: {} : item_size: {}  first_item_offset: {}  unknown_header_data: {:?}",
            self.num_items, self.item_size, self.first_item_offset, self.unknown_header_data,
        )
    }
}

#[derive(Clone, Copy, Debug, Default, FromBytes, IntoBytes, KnownLayout)]
#[repr(C, packed)]
pub struct HDRaItem {
    tag: [u8; 8],
    data: [u8; 8],
    name: [u8; 16],
    firmware: [u8; 4],
    build: [u8; 4],
    device_serial: [u8; 8],
    suffix: [u8; 16],
}

pub fn copy_ascii_str_to_native(
    src: &String,
    dest: &mut [u8],
    dest_name: &str,
    pad_byte: u8,
) -> TD0Result<()> {
    if src.len() > dest.len() {
        return Err(TD0Error::ConvertToNativeTypeError {
            field: dest_name.to_string(),
            reason: "Source String too long.".to_string(),
        });
    }
    if !src.is_ascii() {
        return Err(TD0Error::ConvertToNativeTypeError {
            field: dest_name.to_string(),
            reason: "Source String is not ascii-only.".to_string(),
        });
    }

    let src_bytes = src.as_bytes();
    dest[src_bytes.len()..].fill(pad_byte);
    dest[..src_bytes.len()].copy_from_slice(&src_bytes);
    Ok(())
}

pub fn copy_slice_to_native(src: &[u8], dest: &mut [u8], dest_name: &str) -> TD0Result<()> {
    if src.len() != dest.len() {
        return Err(TD0Error::ConvertToNativeTypeError {
            field: dest_name.to_string(),
            reason: "Length Mismatch".to_string(),
        });
    }
    dest.clone_from_slice(src);
    Ok(())
}

pub fn copy_slice_to_native_padded(
    src: &[u8],
    dest: &mut [u8],
    dest_name: &str,
    pad: u8,
) -> TD0Result<()> {
    if src.len() > dest.len() {
        return Err(TD0Error::ConvertToNativeTypeError {
            field: dest_name.to_string(),
            reason: "src length > dest length".to_string(),
        });
    }
    dest[src.len()..].fill(pad);
    dest[..src.len()].clone_from_slice(src);
    Ok(())
}

const FIELDS_HDR_A_ITEM: [&str; 7] = [
    "tag",
    "data",
    "name",
    "firmware",
    "build",
    "device_serial",
    "suffix",
];

impl ChunkItem for HDRaItem {
    fn get_value(&self, field: &str) -> Option<ChunkItemValue> {
        return match field {
            "tag" => Some(ChunkItemValue::Text(
                String::from_utf8_lossy(&self.tag).to_string(),
            )),
            "data" => Some(ChunkItemValue::Text(format!("{:?}", self.data))),
            "name" => Some(ChunkItemValue::Text(
                String::from_utf8_lossy(&self.name).to_string(),
            )),
            "firmware" => Some(ChunkItemValue::Text(
                String::from_utf8_lossy(&self.firmware).to_string(),
            )),
            "build" => Some(ChunkItemValue::Text(
                String::from_utf8_lossy(&self.build).to_string(),
            )),
            "device_serial" => Some(ChunkItemValue::Text(
                String::from_utf8_lossy(&self.device_serial).to_string(),
            )),
            "suffix" => Some(ChunkItemValue::Text(format!("{:?}", self.suffix))),
            _ => None,
        };
    }
    fn get_value_raw(&self, field: &str) -> Option<ChunkItemValueRaw> {
        return match field {
            "tag" => Some(ChunkItemValueRaw::Slice(Box::new(self.tag.clone()))),
            "data" => Some(ChunkItemValueRaw::Slice(Box::new(self.data.clone()))),
            "name" => Some(ChunkItemValueRaw::Slice(Box::new(self.name.clone()))),
            "firmware" => Some(ChunkItemValueRaw::Slice(Box::new(self.firmware.clone()))),
            "build" => Some(ChunkItemValueRaw::Slice(Box::new(self.build.clone()))),
            "device_serial" => Some(ChunkItemValueRaw::Slice(Box::new(
                self.device_serial.clone(),
            ))),
            "suffix" => Some(ChunkItemValueRaw::Slice(Box::new(self.suffix.clone()))),
            _ => None,
        };
    }
    fn set_value(&mut self, field: &str, value: &ChunkItemValue) -> TD0Result<()> {
        match (field, value) {
            ("tag", ChunkItemValue::Text(val)) => {
                copy_ascii_str_to_native(val, &mut self.tag, field, 0)?;
            }
            ("data", ..) => {
                return Err(TD0Error::ConvertToNativeTypeError {
                    field: field.to_string(),
                    reason: "Field may not be set.".to_string(),
                });
            }
            ("name", ChunkItemValue::Text(val)) => {
                copy_ascii_str_to_native(val, &mut self.name, field, 0)?;
            }
            ("firmware", ChunkItemValue::Text(val)) => {
                copy_ascii_str_to_native(val, &mut self.firmware, field, 0)?;
            }
            ("build", ChunkItemValue::Text(val)) => {
                copy_ascii_str_to_native(val, &mut self.build, field, 0)?;
            }
            ("device_serial", ChunkItemValue::Text(val)) => {
                copy_ascii_str_to_native(val, &mut self.device_serial, field, 0)?;
            }
            ("suffix", ..) => {
                return Err(TD0Error::ConvertToNativeTypeError {
                    field: field.to_string(),
                    reason: "Field may not be set.".to_string(),
                });
            }
            _ => {
                return Err(TD0Error::ConvertToNativeTypeError {
                    field: field.to_string(),
                    reason: "Unknown field or improper value type.".to_string(),
                });
            }
        }

        Ok(())
    }

    fn set_value_raw(&mut self, field: &str, value: &ChunkItemValueRaw) -> TD0Result<()> {
        match (field, value) {
            ("tag", ChunkItemValueRaw::Slice(val)) => {
                copy_slice_to_native(val, &mut self.tag, field)?;
            }
            ("data", ChunkItemValueRaw::Slice(val)) => {
                copy_slice_to_native(val, &mut self.data, field)?;
            }
            ("name", ChunkItemValueRaw::Slice(val)) => {
                copy_slice_to_native(val, &mut self.name, field)?;
            }
            ("firmware", ChunkItemValueRaw::Slice(val)) => {
                copy_slice_to_native(val, &mut self.firmware, field)?;
            }
            ("build", ChunkItemValueRaw::Slice(val)) => {
                copy_slice_to_native(val, &mut self.build, field)?;
            }
            ("device_serial", ChunkItemValueRaw::Slice(val)) => {
                copy_slice_to_native(val, &mut self.device_serial, field)?;
            }
            ("suffix", ..) => {
                return Err(TD0Error::ConvertToNativeTypeError {
                    field: field.to_string(),
                    reason: "Field may not be set.".to_string(),
                });
            }
            _ => {
                return Err(TD0Error::ConvertToNativeTypeError {
                    field: field.to_string(),
                    reason: "Unknown field or improper value type.".to_string(),
                });
            }
        }

        Ok(())
    }

    fn get_fields(&self) -> &'static [&'static str] {
        &FIELDS_HDR_A_ITEM
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::debug_assert_matches;
    use fastnum::dec64;

    #[test]
    fn test_int_encoded_decimal_from_u16() {
        let actual = IntEncodedDecimal::from(200u16);
        assert_eq!(actual.get_val(), &dec64!(20.0), "Properly converts u16");
    }

    #[test]
    fn test_int_encoded_decimal_into_u16() {
        let actual = IntEncodedDecimal(dec64!(20.0));
        assert_eq!(
            u16::try_from(actual).expect("Or test is broken."),
            200u16,
            "Properly converts to u16"
        );
    }

    #[test]
    fn test_volume_from_i16() {
        let actual = Volume::try_from(12i16).expect("Can't create Volume");
        let expected = Volume(dec64!(1.2));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_volume_max_from_i16() {
        let max_as_i16: i16 = VOLUME_MAX
            .mul(Decimal::TEN)
            .try_into()
            .expect("Can't convert VOLUME_MAX to i16.");
        let actual = Volume::try_from(max_as_i16).expect("Can't create Volume");
        let expected = Volume(VOLUME_MAX);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_volume_min_from_i16() {
        let min_as_i16: i16 = VOLUME_MIN
            .mul(Decimal::TEN)
            .try_into()
            .expect("Can't convert VOLUME_MIN to i16.");
        let actual = Volume::try_from(min_as_i16).expect("Can't create Volume");
        let expected = Volume(VOLUME_MIN);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_volume_minus_inf_from_i16() {
        let actual = Volume::try_from(VOLUME_MINUS_INF_I16).expect("Can't create Volume");
        let expected = Volume(VOLUME_MINUS_INF_DECIMAL);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_volume_to_string() {
        let v1 = Volume(VOLUME_MINUS_INF_DECIMAL);
        assert_eq!(
            v1.to_string().as_str(),
            VOLUME_MINUS_INF_DISPLAY,
            "Test {} Volume to string.",
            VOLUME_MINUS_INF_DISPLAY
        );

        let v2 = Volume(dec64!(-25));
        assert_eq!(
            v2.to_string().as_str(),
            "-25.0",
            "Test a negative Volume value."
        );

        let v3 = Volume(dec64!(4.5));
        assert_eq!(
            v3.to_string().as_str(),
            "4.5",
            "Test a positive Volume value."
        );
    }

    #[test]
    fn test_volume_from_i16_out_of_range() {
        let actual = Volume::try_from(-602i16);
        debug_assert_matches!(
            actual,
            Err(TD0Error::ConvertRangeError { .. }),
            "Test proper Err from > VOLUME_MAX"
        );

        let actual = Volume::try_from(61i16);
        debug_assert_matches!(
            actual,
            Err(TD0Error::ConvertRangeError { .. }),
            "Test proper Err from > VOLUME_MAX"
        );
    }
    #[test]
    fn test_volume_from_str() {
        let actual = Volume::from_str("-60.2");
        debug_assert_matches!(
            actual,
            Err(TD0Error::ConvertRangeError { .. }),
            "Test proper Err from < VOLUME_MIN"
        );

        let actual = Volume::from_str(VOLUME_MIN.to_string().as_str()).expect("Test broken.");
        assert_eq!(
            actual,
            Volume(VOLUME_MIN),
            "Test proper conversion for VOLUME_MIN."
        );

        let actual = Volume::from_str("6.1");
        debug_assert_matches!(
            actual,
            Err(TD0Error::ConvertRangeError { .. }),
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
            Volume(VOLUME_MINUS_INF_DECIMAL),
            "Test proper conversion for -Infinity"
        );
    }

    #[test]
    fn test_chunk_item_val_from_u8_array() {
        let val = ChunkItemValue::text_from_u8_array(" Test \0\0\0\0".as_bytes(), &0);
        assert_eq!(
            val,
            ChunkItemValue::Text(" Test ".to_string()),
            "Should keep space chars before and after token."
        );

        let val = ChunkItemValue::text_from_u8_array("Test With Spaces".as_bytes(), &0x20);
        assert_eq!(
            val,
            ChunkItemValue::Text("Test With Spaces".to_string()),
            "Should keep all non-space chars af end of string."
        );
    }

    #[test]
    fn test_chunk_item_u8_to_u8() {
        let val = ChunkItemValue::U8(23u8);
        if let ChunkItemValue::U8(x) = val {
            assert_eq!(x, 23u8);
        } else {
            assert!(false, "failed");
        }
    }

    #[test]
    fn test_copy_ascii_str_to_native() {
        let mut dest = [0u8; 16];
        let src: String = "I'm 16chars long".to_string();
        let result = copy_ascii_str_to_native(&src, &mut dest, "test_buffer", 0x0);
        assert_eq!(result, Ok(()), "Should not return an Err.");
        assert_eq!(
            String::from_utf8(dest.to_vec())
                .expect("Test broken")
                .as_str(),
            "I'm 16chars long",
            "Should properly copy to dest."
        );

        let src: String = "Needs padding".to_string();
        let result = copy_ascii_str_to_native(&src, &mut dest, "test_buffer", '.' as u8);
        assert_eq!(result, Ok(()), "Testing padding. Should not return an Err.");
        assert_eq!(
            String::from_utf8(dest.to_vec())
                .expect("Test broken")
                .as_str(),
            "Needs padding...",
            "Should properly pad dest bytes."
        );

        debug_assert_matches!(
            copy_ascii_str_to_native(
                &"I'm 17 chars long".to_string(),
                &mut dest,
                "test_buffer",
                0u8
            ),
            Err(TD0Error::ConvertToNativeTypeError { .. }),
            "Error if source string is too long."
        );

        debug_assert_matches!(
            copy_ascii_str_to_native(&"I'm not äscii".to_string(), &mut dest, "test_buffer", 0u8),
            Err(TD0Error::ConvertToNativeTypeError { .. }),
            "Error if non-ascii chars are in source.."
        );
    }

    #[test]
    fn test_copy_slice_to_native_padded() {
        let mut dest = [0u8; 16];
        let src: &[u8] = "I'm 16chars long".as_bytes();

        let result = copy_slice_to_native_padded(src, &mut dest, "test_buffer", 0);
        assert_eq!(result, Ok(()), "Should not return an error.");
        assert_eq!(&dest, src);

        let result = copy_slice_to_native_padded(
            "Needs padding".as_bytes(),
            &mut dest,
            "test_buffer",
            '.' as u8,
        );
        assert_eq!(
            result,
            Ok(()),
            "Testing padding. Should not return an error."
        );
        assert_eq!(&dest, "Needs padding...".as_bytes());

        debug_assert_matches!(
            copy_slice_to_native_padded(
                "I'm 17 chars long".as_bytes(),
                &mut dest,
                "test_buffer",
                0u8
            ),
            Err(TD0Error::ConvertToNativeTypeError { .. }),
            "Error if source slice is too long."
        );
    }
}
