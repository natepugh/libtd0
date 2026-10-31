// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::result::TD0Result;

#[cfg(feature = "serde")]
use crate::helpers::serde_checksum_bytes_to_string;

/// Data accessors for individual data chunk items. (e.g. a single Kit
/// configuration or a single sample metadata tag.) Contains methods to
/// describe available fields, and read or modify those fields' values.
///
/// TD0ChunkItem instances aren't created directly, but rather retrieved from
/// a [TD0File] instance.
pub trait TD0ChunkItem: Send + Sync + core::fmt::Debug {
    /// Return a borrowed reference to this object's buffer.
    ///
    /// Example:
    /// ```
    /// use td0::{ChunkTagStr, TD0Chunk, TD0ChunkItem, TD0Error, TD0File, parse_td0_file};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa` There are 10 TGLa items.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/example_tgl_items.TD0");
    /// let td0file : TD0File = parse_td0_file(TD0_BYTES)?;
    /// let chunk_name : &ChunkTagStr = "TGLa".try_into()?;
    /// let chunk : &TD0Chunk = td0file.chunk(chunk_name).ok_or(TD0Error::unknown_chunk_error(chunk_name))?;
    ///
    /// let chunk_item = chunk.item_at(2)?;
    /// assert_eq!(chunk_item.as_bytes(), b"Kick Proc/Elec  ");
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    fn as_bytes(&self) -> &[u8];

    /// Return the set of valid options for field `field`. Return None if the field doesn't have a restricted set
    /// of options.
    ///
    /// # Example
    /// ```
    /// use td0::{ChunkTagStr, TD0Chunk, TD0ChunkItem, TD0Error, TD0File, new_td0_file, TD0DeviceModel};
    ///
    /// // Load the initial file, which contains a single chunk: `HDRa`.
    /// let mut td0file : TD0File = new_td0_file(TD0DeviceModel::SPDSXPro, "1.10")?;
    /// let chunk_name : &ChunkTagStr = "KITa".try_into()?;
    /// let chunk : &TD0Chunk = td0file.add_chunk(chunk_name, 1)?;
    ///
    /// # // NOTE FOR DOCTEST MAKERS: CHOOSE KIT FIELDS HERE THAT ARE COMPATIBLE WITH THE (faster_incomplete_compile)
    /// # // CFG OPTION! OTHERWISE THAT OPTION CANNOT BE USED WHEN RUNNING DOCTESTS.
    /// #
    /// // Get a default KITa `TDOChunkItem` and call its field_options. TD0Error::ChunkItemParse is used here for
    /// // brevity, it's not a good fit otherwise.
    /// let default_kit : Box<dyn TD0ChunkItem> = chunk.default_item();
    /// assert_eq!(
    ///     default_kit.field_options("click_mode").ok_or(TD0Error::ChunkItemParse)?,
    ///     &["PLAY INTERNAL CLICK", "PLAY WAVE as CLICK", "PLAY WAVE as CLICK-TRACK",]
    /// );
    ///
    /// // field_options() returns None when the field doesn't have a restricted set of options.
    /// assert_eq!(default_kit.field_options("name"), None);
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    fn field_options(&self, field: &str) -> Option<&'static [&'static str]>;

    /// Return the name of the [TD0Value] data type of `field`. Return None if the `field` is not valid for this item.
    ///
    /// # Example
    /// ```
    /// use td0::{parse_td0_file, FieldType, TD0Error, TD0Chunk, TD0ChunkItem, TD0File};
    ///
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_kit.TD0");
    /// let td0file : TD0File = parse_td0_file(TD0_BYTES)?;
    /// let chunk : &TD0Chunk = td0file.chunk("KITa").ok_or(TD0Error::unknown_chunk_error("KITa"))?;
    /// let kit : &dyn TD0ChunkItem = chunk.item_at_ref(0)?;
    ///
    /// // Get a default KITa `TDOChunkItem` and call its field_type.
    /// assert_eq!(kit.field_type("name"), Some(&FieldType::AsciiText{ max_len: 16 }));
    /// assert_eq!(kit.field_type("volume"), Some(&FieldType::Decimal{ min: -60.0, max: 6.0 }));
    /// assert_eq!(kit.field_type("click_pan"), Some(&FieldType::I8{ min: -15, max: 15 }));
    ///
    /// // field_type() returns None when the field isn't valid for this item.
    /// assert_eq!(kit.field_type("foo"), None);
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    fn field_type(&self, field: &str) -> Option<&'static FieldType>;

    /// Return an array of [FieldDef] structs describing all available fields for this chunk.
    /// The description includes the field type and known constraints. For more information,
    /// see [FieldDef].
    ///
    /// # Example
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{FieldDef, FieldType, TD0Chunk, TD0ChunkItem, TD0File, parse_td0_file};
    ///
    /// // This file contains a WVPa chunk with a single item. WVPa items describe the
    /// // sample files stored on the device.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_wvp.TD0");
    /// let td0file : TD0File = parse_td0_file(TD0_BYTES)?;
    /// let chunk : &TD0Chunk = td0file.chunk("WVPa").ok_or(TD0Error::unknown_chunk_error("WVPa"))?;
    /// let wvp : &dyn TD0ChunkItem = chunk.item_at_ref(0)?;
    /// // WVPa contains eleven fields, of four different types, U32, U8, Slice, and AsciiText.
    /// let defs : &[FieldDef] = wvp.field_types();
    ///
    /// let mut actual: Vec<String> = Vec::new();
    /// for field_def in defs.iter() {
    ///     // Skip all unknown fields except the first, so the Slice output will be shown.
    ///     if !field_def.name.starts_with("unknown") || field_def.name == "unknown4" {
    ///         actual.push(format!("{}: {:?}", field_def.name, field_def.field_type));
    ///     }
    /// }
    ///
    /// let expected = vec![
    ///     "start_point: U32 { min: 0, max: 4294967295 }",
    ///     "end_point: U32 { min: 0, max: 4294967295 }",
    ///     "loop_point: U32 { min: 0, max: 4294967295 }",
    ///     "volume: U8 { min: 0, max: 127 }",
    ///     "unknown4: Slice { len: 3 }",
    ///     "name: AsciiText { max_len: 16 }",
    ///     "filename: AsciiText { max_len: 100 }",
    ///     "device_serial_number: AsciiText { max_len: 8 }",
    /// ];
    ///
    /// assert_eq!(actual, expected);
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    fn field_types(&self) -> &'static [FieldDef];

    /// Return the value of `field` as a Option<[TD0Value]>, or None if `field` is not a valid field for this item.
    ///
    /// Example:
    /// ```
    /// use td0::{parse_td0_file, TD0Error, TD0Chunk, TD0ChunkItem, TD0File, TD0Value};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa` There are 10 TGLa items.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/example_tgl_items.TD0");
    /// let td0file : TD0File = parse_td0_file(TD0_BYTES)?;
    /// let tgl_chunk : &TD0Chunk = td0file.chunk("TGLa").ok_or(TD0Error::unknown_chunk_error("TGLa"))?;
    /// let chunk_item : &dyn TD0ChunkItem = tgl_chunk.item_at_ref(9)?;
    /// assert_eq!(chunk_item.field_value("name"), Some(TD0Value::new_text_value("HiHat")));
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    fn field_value(&self, field: &str) -> Option<TD0Value>;

    /// Return the raw value of `field` as a Option<[TD0ValueRaw]>, or None if `field` is not a valid field
    /// for this item.
    ///
    /// Example:
    /// ```
    /// use td0::{parse_td0_file, TD0Chunk, TD0ChunkItem, TD0Error, TD0File, TD0ValueRaw};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa` There are 10 TGLa items.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/example_tgl_items.TD0");
    /// let td0file : TD0File = parse_td0_file(TD0_BYTES)?;
    /// let tgl_chunk : &TD0Chunk = td0file.chunk("TGLa").ok_or(TD0Error::unknown_chunk_error("TGLa"))?;
    /// let chunk_item : &dyn TD0ChunkItem = tgl_chunk.item_at_ref(4)?;
    ///
    /// // The raw value of a TD0Value::Text field (such as "name") is a boxed [u8] array.
    /// assert_eq!(
    ///     chunk_item.field_value_raw("name"),
    ///     Some(TD0ValueRaw::Slice(Box::from(*b"Snare Proc/Elec ")))
    /// );
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    fn field_value_raw(&self, field: &str) -> Option<TD0ValueRaw>;

    /// Return this chunk item's field names as an &str array.
    ///
    /// # Example
    /// ```
    /// use td0::{ChunkTagStr, TD0Chunk, TD0ChunkItem, TD0Error, TD0File, parse_td0_file};
    ///
    /// // Load the initial file, which contains a single chunk: `HDRa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_two_chunks.TD0");
    /// let td0file : TD0File = parse_td0_file(TD0_BYTES)?;
    /// let chunk_name : &ChunkTagStr = "TGLa".try_into()?;
    /// let chunk : &TD0Chunk = td0file.chunk(chunk_name).ok_or(TD0Error::unknown_chunk_error(chunk_name))?;
    ///
    /// // Get a default TGLa `TDOChunkItem` and call its list_fields. It has one field, "name".
    /// // Some other chunk types' collection of fields, while interesting, are not brief.
    /// let default_tgl : Box<dyn TD0ChunkItem> = chunk.default_item();
    /// assert_eq!(default_tgl.list_fields(), ["name"]);
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    fn list_fields(&self) -> &'static [&'static str];

    /// Set the value of `field` to [TD0Value] `value`.
    ///
    /// # Errors
    /// - [TD0Error::InvalidField](crate::result::TD0Error::InvalidField) if `field` is not valid for this item type.
    /// - [TD0Error::DataType](crate::result::TD0Error::DataType) if `value` is not the correct [TD0Value]
    ///   variant for `field`. To determine the variant, use [TD0ChunkItem::field_type].
    /// - [TD0Error::OutOfRange](crate::result::TD0Error::OutOfRange)
    ///   - if `value` is outside of the allowable range (for int types.)
    ///   - if `value` is too long (for Text types.)
    /// - [TD0Error::OutOfRangeDecimal](crate::result::TD0Error::OutOfRangeDecimal) if `value` is outside of the
    ///   allowable range (for Decimal types.)
    /// - [TD0Error::InvalidInput](crate::result::TD0Error::InvalidInput) if `value` is of the correct type but not
    ///   an allowable value, and none of the OutOfRange errors apply.
    ///   Some possible reasons:
    ///   - `field` requires only ASCII printable characters, and `value` doesn't conform.
    ///   - `field` has a set of allowable options that `value` doesn't belong to.
    /// - [TD0Error::ReadOnlyField](crate::result::TD0Error::ReadOnlyField) if `field` is not editable. Used as
    ///   a safety feature when it's not yet determined what a range of bytes in the raw data represents.
    ///   An escape hatch exists for the clever, or foolhardy,
    ///   [set_field_value_raw](TD0ChunkItem::set_field_value_raw) allows setting any field.
    ///
    /// # Examples
    ///
    /// ## Setting a [Text](TD0Value::Text) field
    /// ```
    /// use td0::{parse_td0_file, TD0Chunk, TD0ChunkItem, TD0Error, TD0File, TD0Value};
    ///
    /// // Load the initial file.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_kit.TD0");
    /// let mut td0file : TD0File = parse_td0_file(TD0_BYTES)?;
    /// let chunk : &mut TD0Chunk = td0file.chunk_mut("KITa").ok_or(TD0Error::unknown_chunk_error("KITa"))?;
    ///
    /// // Get the kit item from the example file. Its name field is currently set to "Dance".
    /// let mut kit : &mut dyn TD0ChunkItem = chunk.item_at_mut(0)?;
    /// assert_eq!(kit.field_value("name"), Some(TD0Value::new_text_value("Dance")));
    ///
    /// // Change its name field and verify the value was changed.
    /// let set_result = kit.set_field_value(
    ///     "name",
    ///     TD0Value::new_text_value("Kilt, er, kit")
    /// );
    /// assert_eq!(kit.field_value("name"), Some(TD0Value::new_text_value("Kilt, er, kit")));
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    /// ## Setting an integer field.
    /// The example shows how to set a [TD0Value::I8] field. The other integer types
    /// are set in the same way, using a TD0Value of that type instead. See [TD0Value].
    /// ```
    /// # use td0::TD0Error;
    /// # use td0::{parse_td0_file, TD0Chunk, TD0ChunkItem, TD0File, TD0Value};
    /// # const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_kit.TD0");
    /// # let mut td0file : TD0File = parse_td0_file(TD0_BYTES)?;
    /// # let chunk : &mut TD0Chunk = td0file.chunk_mut("KITa").ok_or(TD0Error::unknown_chunk_error("KITa"))?;
    /// # let mut kit : &mut dyn TD0ChunkItem = chunk.item_at_mut(0)?;
    /// // (Identical test setup as "Setting a Text field section." omitted here.)
    /// let set_result = kit.set_field_value(
    ///     "click_pan",
    ///     TD0Value::I8(-12)
    /// );
    /// assert!(set_result.is_ok());
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    /// # Setting a [Decimal](TD0Value::Decimal) field
    /// Decimal fields are set with a f32, rounded to a single decimal point.
    /// Ensure that your f32 value is rounded ahead of time to avoid any unexpected results.
    /// See the note here: [TD0Value::Decimal]
    /// ```
    /// # use td0::TD0Error;
    /// # use td0::{parse_td0_file, TD0Chunk, TD0ChunkItem, TD0File, TD0Value};
    /// # const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_kit.TD0");
    /// # let mut td0file : TD0File = parse_td0_file(TD0_BYTES)?;
    /// # let chunk : &mut TD0Chunk = td0file.chunk_mut("KITa").ok_or(TD0Error::unknown_chunk_error("KITa"))?;
    /// # let mut kit : &mut dyn TD0ChunkItem = chunk.item_at_mut(0)?;
    /// // (Identical test setup as "Setting a Text field section." omitted here.)
    ///
    /// let my_tempo : f32 = 125.5_f32;
    /// let set_result = kit.set_field_value(
    ///     "tempo",
    ///     TD0Value::Decimal(my_tempo)
    /// );
    /// assert!(set_result.is_ok());
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    fn set_field_value(&mut self, field: &str, value: TD0Value) -> TD0Result<()>;

    /// Set the raw value of `field` to [TD0ValueRaw] `raw_value`.
    ///
    /// # Errors
    /// - [TD0Error::InvalidField](crate::result::TD0Error::InvalidField) if `field` is not valid for this item type.
    /// - [TD0Error::DataType](crate::result::TD0Error::DataType) if `value` is not the correct [TD0ValueRaw]
    ///   variant for `field`. To determine the variant, use [TD0ChunkItem::field_type].
    /// - [TD0Error::InvalidInput](crate::result::TD0Error::InvalidInput) if `value` is of the correct type but not
    ///   an allowable value, (e.g. a [TD0ValueRaw::Slice] is the wrong size.)
    ///
    /// # Warning!
    /// The intended use of this method is as an escape hatch during development and debugging.
    /// It does not perform any of the data validation that
    /// [set_field_value](TD0ChunkItem::set_field_value) does and is inherently unsafe to
    /// use for editing any backup file that may get loaded into an actual physical device.
    ///
    /// THE AUTHOR(S) OF THIS LIBRARY ASSUME NO RESPONSIBILITY FOR YOU BRICKING YOUR SAMPLE
    /// PAD TEN MINUTES BEFORE GIG (WHAT WERE YOU EVEN THINKING?) AND GETTING FIRED, DESTROYING
    /// YOUR REPUTATION, MISSING OUT ON YOUR "BIG BREAK", ETC.
    ///
    /// # Examples
    ///
    /// ## Setting the raw data of a [Text](TD0Value::Text) as a boxed [u8] slice.
    /// ```
    /// use td0::{parse_td0_file, TD0Chunk, TD0ChunkItem, TD0Error, TD0File, TD0Value, TD0ValueRaw};
    ///
    /// // Load the initial file.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_kit.TD0");
    /// let mut td0file : TD0File = parse_td0_file(TD0_BYTES)?;
    /// let chunk : &mut TD0Chunk = td0file.chunk_mut("KITa").ok_or(TD0Error::unknown_chunk_error("KITa"))?;
    ///
    /// // Get the kit item from the example file. Its name field is currently set to "Dance".
    /// let mut kit : &mut dyn TD0ChunkItem = chunk.item_at_mut(0)?;
    ///
    /// // Set its "name" field, a Text field. Note the spaces used to pad the str
    /// // to 16 characters- the "name" field is 16 bytes in size.
    /// let set_result = kit.set_field_value_raw(
    ///     "name",
    ///     TD0ValueRaw::Slice(Box::new(*b"I Like Danger   "))
    /// );
    /// assert!(set_result.is_ok());
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    fn set_field_value_raw(&mut self, field: &str, raw_value: TD0ValueRaw) -> TD0Result<()>;
}

/// A read-only description of a [`TD0File`] data chunk.
/// [`TD0Manifest`] structs contain one ChunkManifest item for each data chunk in the file.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChunkManifest {
    pub(crate) name: String,
    pub(crate) pos: usize,
    pub(crate) size: usize,
    pub(crate) num_items: usize,
    pub(crate) item_size: usize,
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

    /// Create a new ChunkManfest struct.
    ///
    /// # Example
    /// ```
    /// use td0::ChunkManifest;
    ///
    /// let chunk_manifest : ChunkManifest = ChunkManifest::new("FOOa", 16, 96, 5, 16);
    /// assert_eq!(chunk_manifest.to_string().as_str(), "FOOa:  pos: 16  size: 96  num_items: 5  item_size: 16");
    /// ```
    ///
    pub fn new(
        name: impl Into<String>,
        pos: usize,
        size: usize,
        num_items: usize,
        item_size: usize,
    ) -> Self {
        Self {
            name: name.into(),
            pos,
            size,
            num_items,
            item_size,
        }
    }
}

/// An intermediate type between TD0's native decimal storage (as an integer) and
/// the float-backed Decimal representation found in [TD0Value]. Used for input
/// validation.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct IntEncodedDecimal(pub f32);

/// The type of backup a [`TD0File`] contains.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum TD0BackupType {
    /// A backup of a single kit.
    Kit,
    /// A backup of the entire set of system settings including all kits.
    System,
    /// This library's default when none of the above apply.
    Unknown,
}

/// The model of the device that created the [`TD0File`] backup.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum TD0DeviceModel {
    /// Roland SPD-SX Pro (NOT compatible with the original SPD-SX.)
    SPDSXPro,
    /// This library's default when none of the above apply.
    Unknown,
}

/// Publicly accessible metadata for a [`TD0File`]-implementing struct.
///
/// ## Feature flags
#[doc = document_features::document_features!()]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Debug)]
pub struct TD0Manifest {
    pub(crate) backup_name: String,
    pub(crate) backup_type: TD0BackupType,

    #[cfg_attr(
        feature = "serde",
        serde(serialize_with = "serde_checksum_bytes_to_string")
    )]
    pub(crate) checksum_actual: Vec<u8>,

    #[cfg_attr(
        feature = "serde",
        serde(serialize_with = "serde_checksum_bytes_to_string")
    )]
    pub(crate) checksum_calculated: Vec<u8>,
    pub(crate) device_model: TD0DeviceModel,
    pub(crate) device_firmware_version: String,
    pub(crate) device_firmware_build: String,
    pub(crate) device_serial: String,
    pub(crate) size_actual: usize,
    pub(crate) size_calculated: usize,
    pub(crate) chunks: Vec<ChunkManifest>,
}

/// The value type returned from [TD0ChunkItem::field_value()], and required as
/// the value parameter in [TD0ChunkItem::set_field_value()].
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum TD0Value {
    /// A decimal value such as 12.0 or -7.2.
    ///
    /// NOTE:
    /// This value is rounded to one decimal place (0.1) upon display and
    /// before setting the value of any field. To prevent unexpected results,
    /// ensure that your f32 value has already been rounded to one decimal
    /// place _before_ creating a [TD0Value::Decimal].
    Decimal(f32),
    /// A signed 8-bit integer.
    I8(i8),
    /// A signed 16-bit integer.
    I16(i16),
    /// A boxed slice of bytes.
    Slice(Box<[u8]>),
    /// A text string.
    ///
    /// NOTE: The currently implemented hardware device supports only
    /// the printable ASCII characters (32 - 126).
    Text(String),
    /// An unsigned 8-bit integer.
    U8(u8),
    /// An unsigned 16-bit integer.
    U16(u16),
    /// An unsigned 32-bit integer.
    U32(u32),
}

/// The value type returned from [TD0ChunkItem::field_value_raw()], and required as
/// the value parameter in [TD0ChunkItem::set_field_value_raw()].
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum TD0ValueRaw {
    /// A signed 8-bit integer.
    I8(i8),
    /// A signed 16-bit integer.
    I16(i16),
    /// A boxed slice of bytes.
    Slice(Box<[u8]>),
    /// An unsigned 8-bit integer.
    U8(u8),
    /// An unsigned 16-bit integer.
    U16(u16),
    /// An unsigned 32-bit integer.
    U32(u32),
}

/// An intermediate between a TD0Decimal/f32 type and a Decimal volume value.
/// This struct's implementation provides validation for TD0 "main" volume
/// values.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Volume(pub f32);
impl core::fmt::Display for Volume {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.0.eq(&Self::MINUS_INF_FLOAT) {
            write!(f, "{}", Self::MINUS_INF_DISPLAY)
        } else {
            write!(f, "{:.1}", self.0)
        }
    }
}

/// Rich field type information containing field-specific validation constraints.
#[derive(Clone, PartialEq, Debug)]
pub enum FieldType {
    /// A string composed of only printable ASCII characters, of `max_len` or
    /// shorter length. Printable characters are code points 32 (" ") - 126 "~".
    AsciiText { max_len: usize },
    /// A string that precisely matches one of the options in `options`.
    EnumText { options: &'static [&'static str] },
    /// A signed 8-bit integer. Valid values are between `min` and `max`, inclusive.
    I8 { min: i8, max: i8 },
    /// A signed 16-bit integer. Valid values are between `min` and `max`, inclusive.
    I16 { min: i16, max: i16 },
    /// A Decimal value, accurate to 1 decimal place.
    /// Valid values are between `min` and `max`, inclusive.
    Decimal { min: f32, max: f32 },
    /// An unsigned 8-bit integer. Valid values are between `min` and `max`, inclusive.
    U8 { min: u8, max: u8 },
    /// An unsigned 16-bit integer. Valid values are between `min` and `max`, inclusive.
    U16 { min: u16, max: u16 },
    /// An unsigned 32-bit integer. Valid values are between `min` and `max`, inclusive.
    U32 { min: u32, max: u32 },
    /// An [u8] slice of exactly `len` size.
    Slice { len: usize },
}

/// A field definition containing the field name and a FieldType value.
#[derive(Clone, PartialEq, Debug)]
pub struct FieldDef {
    pub name: &'static str,
    pub field_type: FieldType,
}

/// A generic difference between two items.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Diff<T>
where
    T: Clone + core::fmt::Debug + PartialEq,
{
    pub left: T,
    pub right: T,
}
