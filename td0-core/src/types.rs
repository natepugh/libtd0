// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::result::TD0Result;

#[cfg(feature = "serde")]
use crate::helpers::serde_checksum_bytes_to_string;

/// Data accessors for individual data chunk items. (e.g. a single Kit configuration or
/// a single sample metadata tag.) Contains methods to describe available fields, read
/// those fields' values and write those values.
///
/// TD0ChunkItems aren't created directly, but rather retrieved from a [TD0File] instance.
pub trait TD0ChunkItem: Send + Sync + core::fmt::Debug {
    /// Return a borrowed reference to this object's buffer.
    ///
    /// Example:
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0ChunkItem, TD0File};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa` There are 10 TGLa items.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/example_tgl_items.TD0");
    /// let td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// let chunk_item = td0file.chunk_item("TGLa", 2)?;
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
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0ChunkItem, TD0File};
    ///
    /// // Load the initial file, which contains a single chunk: `HDRa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_v1_10.TD0");
    /// let td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// # // NOTE FOR DOCTEST MAKERS: CHOOSE KIT FIELDS HERE THAT ARE COMPATIBLE WITH THE (faster_incomplete_compile)
    /// # // CFG OPTION! OTHERWISE THAT OPTION CANNOT BE USED WHEN RUNNING DOCTESTS.
    /// #
    /// // Get a default KITa `TDOChunkItem` and call its field_options. TD0Error::ChunkItemParse is used here for
    /// // brevity, it's not a good fit otherwise.
    /// let default_kit : Box<dyn TD0ChunkItem> = td0file.chunk_item_default("KITa").ok_or(TD0Error::ChunkItemParse)?;
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
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0ChunkItem, TD0File};
    ///
    /// // Load the initial file, which contains a single chunk: `HDRa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_v1_10.TD0");
    /// let td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// # // NOTE FOR DOCTEST MAKERS: CHOOSE KIT FIELDS HERE THAT ARE COMPATIBLE WITH THE (faster_incomplete_compile)
    /// # // CFG OPTION! OTHERWISE THAT OPTION CANNOT BE USED WHEN RUNNING DOCTESTS.
    /// #
    /// // Get a default KITa `TDOChunkItem` and call its field_type. TD0Error::ChunkItemParse is used here for
    /// // brevity, it's not a good fit otherwise.
    /// let default_kit : Box<dyn TD0ChunkItem> = td0file.chunk_item_default("KITa").ok_or(TD0Error::ChunkItemParse)?;
    /// assert_eq!(default_kit.field_type("name"), Some("Text"));
    /// assert_eq!(default_kit.field_type("volume"), Some("Decimal"));
    /// assert_eq!(default_kit.field_type("click_pan"), Some("I8"));
    ///
    /// // field_options() returns None when the field isn't valid for this item.
    /// assert_eq!(default_kit.field_options("foo"), None);
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    fn field_type(&self, field: &str) -> Option<&'static str>;

    /// Return the value of `field` as a Option<[TD0Value]>, or None if `field` is not a valid field for this item.
    ///
    /// Example:
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0ChunkItem, TD0File, TD0Value};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa` There are 10 TGLa items.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/example_tgl_items.TD0");
    /// let td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// let chunk_item = td0file.chunk_item("TGLa", 9)?;
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
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0ChunkItem, TD0File, TD0ValueRaw};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa` There are 10 TGLa items.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/example_tgl_items.TD0");
    /// let td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// // The raw value of a TD0Value::Text field (such as "name") is a boxed [u8] array.
    /// let chunk_item = td0file.chunk_item("TGLa", 4)?;
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
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0ChunkItem, TD0File};
    ///
    /// // Load the initial file, which contains a single chunk: `HDRa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_v1_10.TD0");
    /// let td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// // Get a default TGLa `TDOChunkItem` and call its list_fields. It has one field, "name".
    /// // Some other chunk types' collection of fields, while interesting, are not brief.
    /// let default_tgl : Box<dyn TD0ChunkItem> = td0file.chunk_item_default("TGLa").ok_or(TD0Error::ChunkItemParse)?;
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
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0ChunkItem, TD0File, TD0Value};
    ///
    /// // Load the initial file.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_v1_10.TD0");
    /// let td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// // Get a default KITa `TDOChunkItem` and set its "name" field, which is a text field.
    /// let mut default_kit : Box<dyn TD0ChunkItem> = td0file.chunk_item_default("KITa")
    ///     .ok_or(TD0Error::unknown_chunk_error("KITa"))?;
    /// let set_result = default_kit.set_field_value(
    ///     "name",
    ///     TD0Value::new_text_value("Kilt, er, kit")
    /// );
    /// assert!(set_result.is_ok());
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    /// ## Setting an integer field.
    /// The example shows how to set a [TD0Value::I8] field. The other integer types
    /// are set in the same way, using a TD0Value of that type instad. See [TD0Value].
    /// ```
    /// # use td0::TD0Error;
    /// # use td0::{parse_td0_file, TD0ChunkItem, TD0File, TD0Value};
    /// # const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_v1_10.TD0");
    /// # let td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    /// # let mut default_kit : Box<dyn TD0ChunkItem> = td0file.chunk_item_default("KITa")
    /// #    .ok_or(TD0Error::unknown_chunk_error("KITa"))?;
    /// // (Identical test setup as "Setting a Text field section." omitted here.)
    /// let set_result = default_kit.set_field_value(
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
    /// # use td0::{parse_td0_file, TD0ChunkItem, TD0File, TD0Value};
    /// # const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_v1_10.TD0");
    /// # let td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    /// # let mut default_kit : Box<dyn TD0ChunkItem> = td0file.chunk_item_default("KITa")
    /// #    .ok_or(TD0Error::unknown_chunk_error("KITa"))?;
    /// // (Identical test setup as "Setting a Text field section." omitted here.)
    ///
    /// let my_tempo : f32 = 125.5_f32;
    /// let set_result = default_kit.set_field_value(
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
    /// [set_field_value](TD0ChunkItem::set_field_value) does and is inhereently unsafe to
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
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0ChunkItem, TD0File, TD0ValueRaw};
    ///
    /// // Load the initial file.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_v1_10.TD0");
    /// let td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// // Get a default KITa `TDOChunkItem`.
    /// let mut default_kit : Box<dyn TD0ChunkItem> = td0file.chunk_item_default("KITa")
    ///     .ok_or(TD0Error::unknown_chunk_error("KITa"))?;
    ///
    /// // Set its "name" field, a Text field. Note the spaces used to pad the str
    /// // to 16 characters- the "name" field is 16 bytes in size.
    /// let set_result = default_kit.set_field_value_raw(
    ///     "name",
    ///     TD0ValueRaw::Slice(Box::new(*b"I Like Danger   "))
    /// );
    /// assert!(set_result.is_ok());
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    fn set_field_value_raw(&mut self, field: &str, raw_value: TD0ValueRaw) -> TD0Result<()>;
}

/// Interface to the data in a .TD0 file.
pub trait TD0File: Send + Sync + core::fmt::Debug {
    /// Add a chunk of `chunk_name` type to the file, containing `num_items` [TD0ChunkItem] items of the
    /// appropriate type.
    ///
    /// The newly created items will be default [TD0ChunkItem] items of the expected type for `chunk_name`. See the
    /// documentation for [TD0File::chunk_item_default] for more information on what data the default items contain.
    ///
    /// # Errors
    /// Errors may vary based on model/firmware-specific implementation of TD0File.
    /// Expected errors include:
    /// - [TD0Error::InvalidChunk](crate::result::TD0Error::InvalidChunk)
    ///   - if `chunk_name` is not a valid chunk name. (This can differ per
    ///     device model and per firmware revision.)
    ///   - if a valid chunk header tag was unable to be created.
    /// - [TD0Error::UnsupportedDeviceFirmwareVersion](crate::result::TD0Error::UnsupportedDeviceFirmwareVersion)
    ///   if the file contains an unexpected firmware version.
    /// - [TD0Error::FileParse](crate::result::TD0Error::FileParse) if this
    ///   file's firmware version and/or model can't be determined.
    /// - [TD0Error::OutOfRange](crate::result::TD0Error::OutOfRange)
    ///   - if the supplied `item_size` is larger than the file's native
    ///     representation allows.
    ///   - if the new chunk offset would be larget than the file format
    ///     allows.
    ///
    /// # Example
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0File};
    ///
    /// // Load the initial file, which contains a single chunk: `HDRa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_v1_10.TD0");
    /// let mut td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// // Add a `TGLa` chunk with five items, and verify that it was added.
    /// td0file.add_chunk("TGLa", 5)?;
    /// assert_eq!(td0file.list_chunks(), vec!["HDRa", "TGLa"]);
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    fn add_chunk(&mut self, chunk_name: &str, num_items: usize) -> TD0Result<()>;

    /// Remove the chunk `chunk_name` from the file.
    ///
    /// # Errors
    /// Errors may vary based on model/firmware-specific implementation of TD0File.
    /// Expected errors include:
    /// - [TD0Error::InvalidChunk](crate::result::TD0Error::InvalidChunk)
    ///   - if `chunk_name` is not a valid chunk name. (This can differ per
    ///     device model and per firmware revision.)
    ///   - if the chunk size in the file metadata is invalid.
    /// - [TD0Error::FileParse](crate::result::TD0Error::FileParse) if this
    ///   file's firmware version and/or model can't be determined.
    ///
    /// # Example
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0File};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_two_chunks.TD0");
    /// let mut td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// // Remove the`TGLa` chunk, and verify the chunk was removed.
    /// td0file.remove_chunk("TGLa")?;
    /// assert_eq!(td0file.list_chunks(), vec!["HDRa"]);
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    fn remove_chunk(&mut self, chunk_name: &str) -> TD0Result<()>;

    /// Replace a chunk item with a `source`, another [TD0ChunkItem] of the same type.
    ///
    /// # Errors
    /// - [TD0Error::InvalidChunk](crate::result::TD0Error::InvalidChunk) if `chunk_name` is not a valid chunk name.
    ///   (This can differ per device model and per firmware revision.)
    /// - [TD0Error::InvalidInput](crate::result::TD0Error::InvalidInput) if `source` is an incorrect [TD0ChunkItem]
    ///   type for `chunk_name`.
    /// - [TD0Error::OutOfRange](crate::result::TD0Error::OutOfRange) if `item_index` is greater than the maximum index
    ///   already available in the chunk. (This method may NOT be used to add a new item to the chunk.)
    ///
    /// # Example
    /// ```
    /// use td0::{parse_td0_file, TD0Error, TD0File, TD0ChunkItem, TD0Result, TD0Value};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_two_chunks.TD0");
    /// let mut td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// // Create a default TGLa TD0ChunkItem and set its "name" field.
    /// let mut test_item : Box<dyn TD0ChunkItem> = td0file.chunk_item_default("TGLa").ok_or(
    ///     TD0Error::unknown_chunk_error("TGLa")
    /// )?;
    /// test_item.set_field_value("name", TD0Value::new_text_value("This is a test"))?;
    ///
    /// // Test helper to retrieve the TGLa name field from a TGLa item as a String.
    /// fn tgl_name_as_string(fileobj: &dyn TD0File, item_index: usize) -> TD0Result<String> {
    ///     let chunk_item : &dyn TD0ChunkItem = fileobj.chunk_item_ref("TGLa", item_index)?;
    ///     let field_value : TD0Value = chunk_item.field_value("name").ok_or(
    ///         TD0Error::InvalidField("name".to_string())
    ///     )?;
    ///     Ok(field_value.to_string())
    /// }
    ///
    /// // Ensure that the expected value isn't already set.
    /// assert_ne!(tgl_name_as_string(&*td0file, 0)?, "This is a test".to_string());
    ///
    /// // Replace TGLa item 0 in thefile with test_item, and verify that the replacement worked.
    /// td0file.chunk_item_replace("TGLa", 0, &*test_item)?;
    /// assert_eq!(tgl_name_as_string(&*td0file, 0)?, "This is a test".to_string());
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    fn chunk_item_replace(
        &mut self,
        chunk_name: &str,
        item_index: usize,
        source: &dyn TD0ChunkItem,
    ) -> TD0Result<()>;

    /// Copy a [TD0ChunkItem] within `chunk_name` to `dest_index` overwriting the item at that index.
    ///
    /// The source item is not affected by the copy.
    ///
    /// # Errors
    /// - [TD0Error::InvalidChunk](crate::result::TD0Error::InvalidChunk) if `chunk_name` is not a valid chunk name.
    ///   (This can differ per device model and per firmware revision.)
    /// - [TD0Error::OutOfRange](crate::result::TD0Error::OutOfRange) if `source_index` or `dest_index` is greater
    ///   than the maximum index available in the chunk.
    ///
    /// # Example
    /// ```
    /// use td0::{parse_td0_file, TD0Error, TD0File, TD0ChunkItem, TD0Result, TD0Value};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_two_chunks.TD0");
    /// let mut td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// // Test helper to retrieve the TGLa name field from a TGLa item as a String.
    /// fn tgl_name_as_string(fileobj: &dyn TD0File, item_index: usize) -> TD0Result<String> {
    ///     let chunk_item : &dyn TD0ChunkItem = fileobj.chunk_item_ref("TGLa", item_index)?;
    ///     let field_value : TD0Value = chunk_item.field_value("name").ok_or(
    ///         TD0Error::InvalidField("name".to_string())
    ///     )?;
    ///     Ok(field_value.to_string())
    /// }
    ///
    /// // Set the name field of TGLa item 0.
    /// td0file.chunk_item_mut("TGLa", 0)?.set_field_value("name", TD0Value::new_text_value("My New Tag"))?;
    ///
    /// // Verify that TGLa item 2 isn't already the new value:
    /// assert_ne!(tgl_name_as_string(&*td0file, 2)?, "My New Tag".to_string());
    ///
    /// // Copy item 0 to item 2 and verify the value is now set:
    /// td0file.chunk_item_copy("TGLa", 0, 2)?;
    /// assert_eq!(tgl_name_as_string(&*td0file, 2)?, "My New Tag".to_string());
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    fn chunk_item_copy(
        &mut self,
        chunk_name: &str,
        source_index: usize,
        dest_index: usize,
    ) -> TD0Result<()>;

    /// Swap two chunk items in `chunk_name`.
    ///
    /// # Errors
    /// - [TD0Error::InvalidChunk](crate::result::TD0Error::InvalidChunk) if `chunk_name` is not a valid chunk name.
    ///   (This can differ per device model and per firmware revision.)
    /// - [TD0Error::OutOfRange](crate::result::TD0Error::OutOfRange) if `source_index` or `dest_index` is greater
    ///   than the maximum index available in the chunk.
    ///
    /// # Example
    /// ```
    /// use td0::{parse_td0_file, TD0Error, TD0File, TD0ChunkItem, TD0Result, TD0Value};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_two_chunks.TD0");
    /// let mut td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// // Test helper to retrieve the TGLa name field from a TGLa item as a String.
    /// fn tgl_name_as_string(fileobj: &dyn TD0File, item_index: usize) -> TD0Result<String> {
    ///     let chunk_item : &dyn TD0ChunkItem = fileobj.chunk_item_ref("TGLa", item_index)?;
    ///     let field_value : TD0Value = chunk_item.field_value("name").ok_or(
    ///         TD0Error::InvalidField("name".to_string())
    ///     )?;
    ///     Ok(field_value.to_string())
    /// }
    ///
    /// // Set the name field of TGLa item 0.
    /// td0file.chunk_item_mut("TGLa", 0)?.set_field_value("name", TD0Value::new_text_value("Was item 0"))?;
    /// td0file.chunk_item_mut("TGLa", 1)?.set_field_value("name", TD0Value::new_text_value("Not swapped"))?;
    /// td0file.chunk_item_mut("TGLa", 2)?.set_field_value("name", TD0Value::new_text_value("Was item 2"))?;
    ///
    /// // Perform the swap and verify the expected results.
    /// td0file.chunk_items_swap("TGLa", 0, 2)?;
    /// assert_eq!(tgl_name_as_string(&*td0file, 0)?, "Was item 2".to_string());
    /// assert_eq!(tgl_name_as_string(&*td0file, 1)?, "Not swapped".to_string());
    /// assert_eq!(tgl_name_as_string(&*td0file, 2)?, "Was item 0".to_string());
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    fn chunk_items_swap(
        &mut self,
        chunk_name: &str,
        index_1: usize,
        index_2: usize,
    ) -> TD0Result<()>;

    /// Reorder all [TD0ChunkItem]s in `chunk_name`.
    ///
    /// Every valid item index in `chunk_name` must be present exactly 1 time in `new_order`. Precisely, new_order must:
    /// - Contain every usize value in the range 0..[TD0File::chunk_num_items],
    /// - Must be of size [TD0File::chunk_num_items], and
    /// - Must not have any duplicate values.
    ///
    /// # Errors
    /// - [TD0Error::InvalidChunk](crate::result::TD0Error::InvalidChunk) if `chunk_name` is not a valid chunk name.
    ///   (This can differ per device model and per firmware revision.)
    /// - [TD0Error::InvalidInput](crate::result::TD0Error::InvalidInput) if `new_order` does not meet the preconditions
    ///   outlined above.
    ///
    /// # Example
    /// ```
    /// use td0::{parse_td0_file, TD0Error, TD0File, TD0ChunkItem, TD0Result, TD0Value};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa`.
    /// // `TGLa` has three (3) chunk items.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_two_chunks.TD0");
    /// let mut td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// // Test helper to retrieve the TGLa name field from a TGLa item as a String.
    /// fn tgl_name_as_string(fileobj: &dyn TD0File, item_index: usize) -> TD0Result<String> {
    ///     let chunk_item : &dyn TD0ChunkItem = fileobj.chunk_item_ref("TGLa", item_index)?;
    ///     let field_value : TD0Value = chunk_item.field_value("name").ok_or(
    ///         TD0Error::InvalidField("name".to_string())
    ///     )?;
    ///     Ok(field_value.to_string())
    /// }
    ///
    /// // Set the name field of TGLa item 0.
    /// td0file.chunk_item_mut("TGLa", 0)?.set_field_value("name", TD0Value::new_text_value("Was item 0"))?;
    /// td0file.chunk_item_mut("TGLa", 1)?.set_field_value("name", TD0Value::new_text_value("Was item 1"))?;
    /// td0file.chunk_item_mut("TGLa", 2)?.set_field_value("name", TD0Value::new_text_value("Was item 2"))?;
    ///
    /// // Perform the swap and verify the expected results.
    /// td0file.chunk_items_reorder("TGLa", &[1, 2, 0])?;
    /// assert_eq!(tgl_name_as_string(&*td0file, 0)?, "Was item 1".to_string());
    /// assert_eq!(tgl_name_as_string(&*td0file, 1)?, "Was item 2".to_string());
    /// assert_eq!(tgl_name_as_string(&*td0file, 2)?, "Was item 0".to_string());
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    fn chunk_items_reorder(&mut self, chunk_name: &str, new_order: &[usize]) -> TD0Result<()>;

    /// Ensure that the [TD0File] is ready to be read via [TD0File::try_to_bytes]. This method should be called
    /// prior to exporting the file's bytes, otherwise the user risks exporting an invalid backup file.
    ///
    /// ## Why is this Needed?
    /// [TD0File] implementations MAY perform various optimizations (e.g. caching data or deferring calculations)
    /// behind the scenes, as long as they're transparent to the user. If such leave the data in a stale or inconsistent
    /// state, the implementation MUST raise [TD0Error::ValidationFailed](crate::result::TD0Error::ValidationFailed) upon
    /// a call to [TD0File::try_to_bytes], or any other call that exports the [TD0File]'s bytes.
    ///
    /// An example is deferring the calculation of the file checksum, which speeds up edit operations (vs recalculating
    /// it on every for every edit,) but leaves the file in an inconsistent state. finalize() triggers a checksum
    /// calculation in those cases.
    ///
    /// # Errors
    /// - [TD0Error::FileParse](crate::result::TD0Error::FileParse) If the modified [TD0File] is invalid/unparsable.
    ///
    /// # Example
    /// ```
    /// use td0::{parse_td0_file, TD0Error, TD0File, TD0ChunkItem, TD0Result, TD0Value};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_two_chunks.TD0");
    /// let mut td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// // Modify a value, and attempt to call try_to_bytes.
    /// td0file.chunk_item_mut("TGLa", 0)?.set_field_value("name", TD0Value::new_text_value("Changed value!"))?;
    /// let bytes_result = td0file.try_to_bytes();
    /// // Expected failure, finalize() hasn't been called!
    /// assert!(matches!(bytes_result, Err(TD0Error::ValidationFailed(_))));
    ///
    /// // Try it again...
    /// td0file.finalize()?;
    /// let bytes_result = td0file.try_to_bytes();
    /// assert!(matches!(bytes_result, Ok(_)));
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    fn finalize(&mut self) -> TD0Result<()>;

    /// Parse a new TD0File from bytes.
    ///
    /// This method can't be invokedd directly by clients of `td0` or `td0-core`. Instead, they can use
    /// [td0::parse_td0_file](../td0/fn.parse_td0_file.html), which selects the proper `TD0File` implementation
    /// to parse the bytes.
    ///
    /// # Errors
    /// Errors that may be raised include:
    /// - [TD0Error::FileParse](crate::result::TD0Error::FileParse) if the bytes are unparsable as a TD0File.
    /// - [TD0Error::InvalidChunk](crate::result::TD0Error::InvalidChunk) if a data chunk's chunk header can't be
    ///   parsed.
    ///
    /// # Examples
    /// See [td0::parse_td0_file](../td0/fn.parse_td0_file.html#examples) documentation for an example.
    ///
    fn try_from_bytes(bytes: &[u8]) -> TD0Result<Self>
    where
        Self: Sized;

    /// Return a copy of the file's bytes.
    ///
    /// # Errors
    /// - [TD0Error::ValidationFailed](crate::result::TD0Error::ValidationFailed) if the file is not valid for saving.
    ///   Can often be remedied by calling [TD0File::finalize()] before retrying.
    fn try_to_bytes(&self) -> TD0Result<Vec<u8>>;

    /// Retrieve a reference to a data item from `chunk_name`.
    fn chunk_item_ref(&self, chunk_name: &str, item_index: usize) -> TD0Result<&dyn TD0ChunkItem>;

    /// Retrieve a mutable reference to a data item from `chunk_name`.
    fn chunk_item_mut(
        &mut self,
        chunk_name: &str,
        item_index: usize,
    ) -> TD0Result<&mut dyn TD0ChunkItem>;

    /// Return an owned copy of a data item from `chunk_name`.
    fn chunk_item(&self, chunk_name: &str, item_index: usize) -> TD0Result<Box<dyn TD0ChunkItem>>;

    /// Return a Some(default [TD0ChunkItem]) for `chunk_name` or None if chunk_name is invalid.
    ///
    /// The set of valid chunk names differs based on device model and firmware revision. To retrieve a list of valid
    /// chunks, use the [TD0File::valid_chunk_names] method.
    ///
    /// ## What Does "default" Mean here?
    /// Default items initialize each field to a default value, which varies based on the [TD0Value] type of the field:
    /// - 0 for numeric fields ([TD0Value::U16], [TD0Value::I8], [TD0Value::Decimal], etc.)
    /// - Box<[0; N]> for [TD0Value::Slice] fields,
    /// - The field-appropriate padding character for [TD0Value::Text] fields, or the zeroth option in the field's list
    ///   of options if one exists.
    ///
    /// # Example
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0File};
    ///
    /// // Load the initial file, which contains a single chunk: `HDRa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_v1_10.TD0");
    /// let td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// // A default item will be returned `chunk_name` is valid for model + firmware revision of the file,
    /// // regardless of whether the file contains that chunk or not. (In this case it doesn't.)
    /// let mut chunk_item_option = td0file.chunk_item_default("TGLa");
    /// assert!(matches!(chunk_item_option, Some(_)));
    /// if let Some(chunk_item) = chunk_item_option {
    ///     // TGLa has a single field, "name".
    ///     assert_eq!(chunk_item.list_fields(), ["name"]);
    /// }
    ///
    /// // Returns option None if `chunk_name` is not valid.
    /// assert!(matches!(td0file.chunk_item_default("FOOa"), None));
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    fn chunk_item_default(&self, chunk_name: &str) -> Option<Box<dyn TD0ChunkItem>>;

    /// Return the number of items in `chunk_name`. Returns None if `chunk_name` is present in this file.
    ///
    /// # Example
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0File};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_two_chunks.TD0");
    /// let mut td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// // In this example file `HDRa` has one item and `TGLa` has three.
    /// assert_eq!(td0file.chunk_num_items("HDRa"), Some(1usize));
    /// assert_eq!(td0file.chunk_num_items("TGLa"), Some(3usize));
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    fn chunk_num_items(&self, chunk_name: &str) -> Option<usize>;

    /// Return the absolute position of `chunk_name`. Returns None if `chunk_name` is present in this file.
    ///
    /// # Example
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0File};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_two_chunks.TD0");
    /// let mut td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// assert_eq!(td0file.chunk_pos("HDRa"), Some(48usize));
    /// assert_eq!(td0file.chunk_pos("TGLa"), Some(128usize));
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    fn chunk_pos(&self, chunk_name: &str) -> Option<usize>;

    /// Return a reference to the bytes of a `chunk_name`. Returns None if `chunk_name` is present in this file.
    ///
    /// # Example
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0File, TD0Value};
    ///
    /// // Load the initial file, which contains a single chunk: `HDRa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_v1_10.TD0");
    /// let mut td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// // Add the shortest chunk possible for this file type, TGLa, then set its one item's name.
    /// td0file.add_chunk("TGLa", 1)?;
    /// let mut tgl_item = td0file.chunk_item_mut("TGLa", 0)?;
    /// tgl_item.set_field_value("name", TD0Value::new_text_value("Taggy tag tag"))?;
    ///
    /// /// The expected output is implementation-specific.
    /// let mut expected : Vec<u8> = vec![
    ///     // Chunk header, describes the number of items, the item size, the start offset of the items.
    ///     1, 0, 0, 0, 16, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0
    /// ];
    /// expected.extend(b"Taggy tag tag   ");   // Must be 16 bytes for TGLa in this file type.
    ///
    /// assert_eq!(td0file.chunk_raw("TGLa").unwrap_or_default(), expected.as_slice());
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    fn chunk_raw(&self, chunk_name: &str) -> Option<&[u8]>;

    /// Return the size of chunk `chunk_name`. Returns None if `chunk_name` is present in this file.
    ///
    /// # Example
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0File};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_two_chunks.TD0");
    /// let td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// assert_eq!(td0file.chunk_size("HDRa"), Some(80usize));
    /// assert_eq!(td0file.chunk_size("TGLa"), Some(64usize));
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    fn chunk_size(&self, chunk_name: &str) -> Option<usize>;

    /// Return a list of the names of the chunks present in this file.
    ///
    /// ## Note
    /// This does not list all possible valid chunk names, only those that exist in this file.
    /// To retrieve all valid chunk names for this type of file, use the [TD0File::valid_chunk_names] method.
    ///
    /// # Example
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0File};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_two_chunks.TD0");
    /// let td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// assert_eq!(td0file.list_chunks(), vec!["HDRa".to_string(), "TGLa".to_string()]);
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    fn list_chunks(&self) -> Vec<String>;

    /// Return a [TD0Manifest] containing metadata about the file an its data chunks.
    ///
    /// # Errors:
    /// - [TD0Error::FileParse](crate::result::TD0Error::FileParse)
    ///   - if the file header cannot be parsed.
    ///   - if the file manifest tags can't be parsed.
    ///   - if the file backup manifest chunk (`HDRa`) isn't present or can't be parsed.
    /// - [TD0Error::InvalidChunk](crate::result::TD0Error::InvalidChunk) if chunk header data in the file can't
    ///   be parsed.
    ///
    /// # Example
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0BackupType, TD0DeviceModel, TD0File, TD0Manifest};
    ///
    /// // Load the initial file, which contains two chunks: `HDRa` and `TGLa`.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_two_chunks.TD0");
    /// let td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    ///
    /// let manifest : TD0Manifest = td0file.manifest()?;
    ///
    /// // Test that the manifest has retrievable data.
    /// assert_eq!(manifest.backup_name(), "USER KIT        ");
    /// assert_eq!(manifest.backup_type(), TD0BackupType::Kit);
    /// assert_eq!(manifest.device_model(), TD0DeviceModel::SPDSXPro);
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    fn manifest(&self) -> TD0Result<TD0Manifest>;

    /// Return a new default [TD0File].
    ///
    /// To be implemented when creating new TD0File types.
    /// Regular users of this library should use [td0::new_td0_file](../td0/fn.new_td0_file.html#examples) instead.
    fn new() -> Self
    where
        Self: Sized;

    /// Return the valid chunk names for this TD0File, based on its device model and firmware revision.
    ///
    /// ## Note
    /// All possible valid chunk names are returned, regardless of their presence or absence in the current file.
    /// To retrieve only the chunks found in the current file, use the [TD0File::list_chunks] method.
    ///
    /// # Example
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{new_td0_file, TD0File, TD0DeviceModel, TD0BackupType};
    ///
    /// let file_v1_10 = new_td0_file(TD0DeviceModel::SPDSXPro, "1.10")?;
    /// assert_eq!(file_v1_10.valid_chunk_names()?, &[
    ///     "HDRa", "KITa", "CURa", "STLa", "STPa", "TGLa", "TRGa", "WVPa",
    /// ]);
    ///
    /// let file_v2_0 = new_td0_file(TD0DeviceModel::SPDSXPro, "2.00")?;
    /// assert_eq!(file_v2_0.valid_chunk_names()?, &[
    ///     "HDRa", "KITa", "KITb", "CURa", "PVRa", "STLa", "STPa", "STPb", "TGLa", "TRGa", "WVPa",
    /// ]);
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    fn valid_chunk_names(&self) -> TD0Result<&'static [&'static str]>;

    /// Perform a post-parse validation of the file.
    ///
    /// (Implementation specific.) Validate that that the file meets expectations after parsing,
    /// raise a TD0Error if problems were found.
    ///
    /// # Errors
    /// - [TD0Error::ValidationFailed](crate::result::TD0Error::ValidationFailed) - if the file is not correctly
    ///   formatted, or its metadata doesn't match the actual data.
    ///
    /// # Examples
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0File};
    ///
    /// // Load the initial file, test that it passes validation.
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_w_two_chunks.TD0");
    /// let td0file_good : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    /// assert!(td0file_good.validate_load().is_ok());
    ///
    /// // Load a broken version of the initial file, test that it fails validation.
    /// let td0file_bad : Box<dyn TD0File> = parse_td0_file(&TD0_BYTES[0..TD0_BYTES.len() - 16])?;
    /// assert!(matches!(td0file_bad.validate_load(), Err(TD0Error::ValidationFailed(_))));
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    fn validate_load(&self) -> TD0Result<()>;

    /// Perform a pre-save validation of the file.
    ///
    /// (Implementation specific.) Validate that that the file meets expectations prior to saving,
    /// raise a TD0Error if problems were found.
    ///
    /// # Errors
    /// - [TD0Error::ValidationFailed](crate::result::TD0Error::ValidationFailed)
    ///   - if the file is dirty (thus requiring [TD0File::finalize] to be called on it.)
    ///   - if the file is not correctly formatted, or its metadata doesn't match the actual data.
    ///
    /// # Examples
    /// ```
    /// # use td0::TD0Error;
    /// use td0::{parse_td0_file, TD0File};
    ///
    /// // Load the initial file, add a chunk (thus making it dirty.)
    /// const TD0_BYTES : &[u8] = include_bytes!("../example/data/minimal_v1_10.TD0");
    /// let mut td0file : Box<dyn TD0File> = parse_td0_file(TD0_BYTES)?;
    /// td0file.add_chunk("TGLa", 2)?;
    /// // Test for the expected TD0Error:
    /// assert!(matches!(td0file.validate_save(), Err(TD0Error::ValidationFailed(_))));
    ///
    /// // Finalize the file and try again:
    /// td0file.finalize()?;
    /// assert!(td0file.validate_save().is_ok());
    ///
    /// # Ok::<(), TD0Error>(())
    /// ```
    ///
    fn validate_save(&self) -> TD0Result<()>;
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
    pub(crate) checksum_actual: [u8; 16],

    #[cfg_attr(
        feature = "serde",
        serde(serialize_with = "serde_checksum_bytes_to_string")
    )]
    pub(crate) checksum_calculated: [u8; 16],
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
