// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use bytes::BytesMut;
use std::fmt;
use zerocopy::{LittleEndian, U32};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned};

use crate::header::TD0ManifestTag;
use crate::helpers::{bytes_or_error, try_u32_from_usize};
use crate::result::{TD0Error, TD0Result};
use crate::td0file::ERR_CHUNK_OOB;
use crate::types::Diff;
use crate::types::{FieldDef, FieldType, TD0ChunkItem, TD0Value, TD0ValueRaw};

const ERR_CHUNK_HEADER_PARSE: &str = "unable to parse chunk header bytes.";
const ERR_CHUNK_HEADER_READ: &str = "unable to read chunk header bytes.";
const ERR_CHUNK_READ: &str = "unable to read chunk item(s).";

pub type FieldsDiff = Vec<(String, Diff<TD0Value>)>;
pub type ChunkItemsDiff = Vec<(usize, FieldsDiff)>;

/// The header of a `TD0Chunk` in a `TD0File`.
#[derive(Copy, Clone, Debug, Eq, FromBytes, PartialEq, Immutable, IntoBytes, KnownLayout)]
#[repr(C, packed)]
pub struct ChunkHeader {
    /// The number of items contained in the chunk.
    num_items: U32<LittleEndian>,

    /// The size of an individual chunk item.
    item_size: U32<LittleEndian>,

    /// The offset of the first item, relative to the first byte of the chunk header.
    first_item_offset: U32<LittleEndian>,

    /// Additional chunk header data of unknown purpose.
    unknown_header_data: [u8; 4],
}

impl ChunkHeader {
    pub const fn first_item_offset(&self) -> usize {
        self.first_item_offset.get() as usize
    }

    pub const fn item_size(&self) -> usize {
        self.item_size.get() as usize
    }

    pub const fn num_items(&self) -> usize {
        self.num_items.get() as usize
    }

    pub fn new(
        num_items: u32,
        item_size: u32,
        first_item_offset: u32,
        unknown_header_data: Option<&[u8; 4]>,
    ) -> Self {
        let mut unknown_buf: [u8; 4] = [0; 4];
        if let Some(buf) = unknown_header_data {
            unknown_buf.copy_from_slice(buf);
        }

        Self {
            num_items: U32::from(num_items),
            item_size: U32::from(item_size),
            first_item_offset: U32::from(first_item_offset),
            unknown_header_data: unknown_buf,
        }
    }
    pub fn set_num_items(&mut self, val: u32) {
        self.num_items = U32::from(val);
    }

    pub fn set_item_size(&mut self, val: u32) {
        self.item_size = U32::from(val);
    }

    pub fn set_first_item_offset(&mut self, val: u32) {
        self.first_item_offset = U32::from(val);
    }

    pub fn try_read_from_buf_at(buf: &[u8], pos: usize) -> TD0Result<Self> {
        use zerocopy::TryFromBytes;

        let bytes = bytes_or_error(buf, pos..pos + size_of::<Self>(), || {
            TD0Error::file_parse_error("unable to read manifest tag.")
        })?;

        Self::try_read_from_bytes(bytes)
            .map_err(|_| TD0Error::FileParse("invalid manifest tag.".to_string()))
    }

    pub fn validate(&self, pos: usize, buf_size: usize) -> TD0Result<()> {
        if (pos + self.first_item_offset() + (self.item_size() * self.num_items())) > buf_size {
            Err(TD0Error::validation_error(ERR_CHUNK_OOB))
        } else {
            Ok(())
        }
    }
}

impl Default for ChunkHeader {
    fn default() -> Self {
        Self {
            num_items: U32::from(1),
            item_size: U32::from(u32::try_from(size_of::<TD0ManifestTag>()).unwrap()),
            first_item_offset: U32::from(u32::try_from(size_of::<Self>()).unwrap()),
            unknown_header_data: [0u8; 4],
        }
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

#[derive(Clone, Debug, PartialEq)]
pub struct TD0ChunkDiff {
    pub num_items: Option<Diff<usize>>,
    pub first_item_offset: Option<Diff<usize>>,
    pub items: Option<ChunkItemsDiff>,
}

#[derive(TryFromBytes, KnownLayout, IntoBytes, Immutable, Debug, Unaligned)]
#[repr(transparent)]
pub struct DefaultChunkItem {
    data: [u8; 16],
}

impl TD0ChunkItem for DefaultChunkItem {
    fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    fn field_options(&self, _field: &str) -> Option<&'static [&'static str]> {
        None
    }

    fn field_type(&self, field: &str) -> Option<&'static FieldType> {
        if field == "data" {
            Some(&FieldType::Slice { len: 16 })
        } else {
            None
        }
    }

    fn field_types(&self) -> &'static [FieldDef] {
        &[FieldDef {
            name: "data",
            field_type: FieldType::Slice { len: 16 },
        }]
    }

    fn field_value(&self, field: &str) -> Option<TD0Value> {
        if field == "data" {
            Some(TD0Value::Slice(Box::from(self.data)))
        } else {
            None
        }
    }

    fn field_value_raw(&self, field: &str) -> Option<TD0ValueRaw> {
        if field == "data" {
            Some(TD0ValueRaw::Slice(Box::from(self.data)))
        } else {
            None
        }
    }

    fn list_fields(&self) -> &'static [&'static str] {
        &["data"]
    }

    fn set_field_value(&mut self, field: &str, value: TD0Value) -> TD0Result<()> {
        match (field, value) {
            ("data", TD0Value::Slice(_)) => Err(TD0Error::ReadOnlyField("data".to_string())),
            ("data", _) => Err(TD0Error::DataType(
                "invalid data type setting field 'data'. Expected `Slice`".to_string(),
            )),
            _ => Err(TD0Error::InvalidField(field.to_string())),
        }
    }

    fn set_field_value_raw(&mut self, field: &str, raw_value: TD0ValueRaw) -> TD0Result<()> {
        match (field, raw_value) {
            ("data", TD0ValueRaw::Slice(_)) => Err(TD0Error::ReadOnlyField("data".to_string())),
            ("data", _) => Err(TD0Error::DataType(
                "invalid data type setting field 'data'. Expected `Slice`".to_string(),
            )),
            _ => Err(TD0Error::InvalidField(field.to_string())),
        }
    }
}

impl DefaultChunkItem {
    pub const fn new() -> Self {
        Self { data: [0; 16] }
    }
}

impl Default for DefaultChunkItem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct ChunkItemAccessorCallbacks {
    pub chunk_item_ref_from_bytes_cb: for<'b> fn(&'b [u8]) -> TD0Result<&'b dyn TD0ChunkItem>,
    pub chunk_item_mut_from_bytes_cb:
        for<'b> fn(&'b mut [u8]) -> TD0Result<&'b mut dyn TD0ChunkItem>,
    pub chunk_item_owned_from_bytes_cb: fn(&[u8]) -> TD0Result<Box<dyn TD0ChunkItem>>,
    pub default_chunk_item_cb: fn() -> Box<dyn TD0ChunkItem>,
}

impl ChunkItemAccessorCallbacks {
    fn default_chunk_item_ref_from_bytes(bytes: &[u8]) -> TD0Result<&dyn TD0ChunkItem> {
        use zerocopy::TryFromBytes;

        Ok(DefaultChunkItem::try_ref_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?)
    }

    fn default_chunk_item_mut_from_bytes(bytes: &mut [u8]) -> TD0Result<&mut dyn TD0ChunkItem> {
        use zerocopy::TryFromBytes;

        Ok(DefaultChunkItem::try_mut_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?)
    }

    fn default_chunk_item_owned_from_bytes(bytes: &[u8]) -> TD0Result<Box<dyn TD0ChunkItem>> {
        use zerocopy::TryFromBytes;

        Ok(Box::new(
            DefaultChunkItem::try_read_from_bytes(bytes).map_err(|_| TD0Error::ChunkItemParse)?,
        ))
    }

    fn default_default_chunk_item() -> Box<dyn TD0ChunkItem> {
        use zerocopy::TryFromBytes;

        let bytes: [u8; size_of::<DefaultChunkItem>()] = [0; size_of::<DefaultChunkItem>()];
        Box::new(DefaultChunkItem::try_read_from_bytes(&bytes).unwrap())
    }
}

impl Default for ChunkItemAccessorCallbacks {
    fn default() -> Self {
        Self {
            chunk_item_ref_from_bytes_cb: Self::default_chunk_item_ref_from_bytes,
            chunk_item_mut_from_bytes_cb: Self::default_chunk_item_mut_from_bytes,
            chunk_item_owned_from_bytes_cb: Self::default_chunk_item_owned_from_bytes,
            default_chunk_item_cb: Self::default_default_chunk_item,
        }
    }
}

/// A data chunk within a [`TD0File`](crate::td0file::TD0File).
///
/// A `TD0Chunk` consists of a chunk header, and a data section containing one or more
/// [`TD0ChunkItem`] items, which can access or modify the file data. All `TD0ChunkItem`s
/// within a `TD0Chunk` homogenous- they contain the same fields with the same data types,
/// and differ only in the actual values they contain.
///
/// [`TD0Chunk`] instances aren't created manually, instead retrieved from a `TD0File`
/// via its [`TD0File::chunk()`](crate::td0file::TD0File::chunk) and
/// [`TD0File::chunk_mut()`](crate::td0file::TD0File::chunk_mut) methods.
#[derive(Default, Clone, Debug)]
pub struct TD0Chunk {
    header: BytesMut,
    chunk_extra_data: BytesMut,
    items: Vec<BytesMut>,
    // Callbacks for item data:
    callbacks: ChunkItemAccessorCallbacks,
}

impl TD0Chunk {
    pub const fn new(
        header: BytesMut,
        chunk_extra_data: BytesMut,
        items: Vec<BytesMut>,
        callbacks: ChunkItemAccessorCallbacks,
    ) -> Self {
        Self {
            header,
            chunk_extra_data,
            items,
            callbacks,
        }
    }

    /// Try to create a new `TD0Chunk` with the same item type as `default_item`.
    ///
    /// # Errors
    /// [`TD0Error::OutOfRange`] - If `num_items` or the size of `default_item` is not supported by
    /// this file type.
    pub fn new_with(num_items: usize, default_item: &dyn TD0ChunkItem) -> TD0Result<Self> {
        use zerocopy::IntoBytes;

        let header: ChunkHeader = ChunkHeader::new(
            try_u32_from_usize(num_items)?,
            try_u32_from_usize(size_of_val(default_item))?,
            try_u32_from_usize(size_of::<ChunkHeader>())?,
            None,
        );

        let items: Vec<BytesMut> = vec![BytesMut::from(default_item.as_bytes()); num_items];

        Ok(Self {
            header: BytesMut::from(header.as_bytes()),
            chunk_extra_data: BytesMut::new(),
            items,
            ..Default::default()
        })
    }

    pub fn header(&self) -> &ChunkHeader {
        use zerocopy::{FromBytes, IntoBytes};

        ChunkHeader::ref_from_bytes(self.header.as_bytes())
            .expect("only valid chunk headers are stored.")
    }

    pub fn chunk_item_types_comparable(obj1: &dyn TD0ChunkItem, obj2: &dyn TD0ChunkItem) -> bool {
        size_of_val(obj1) == size_of_val(obj2) && obj1.field_types() == obj2.field_types()
    }

    /// Return a diff between self and other, or None if there are no differences.
    ///
    /// # Errors
    /// - [`TD0Error::ChunksNotComparable`] if the chunks don't hold items of the same type.
    /// - [`TD0Error::ChunkItemParse`] if a chunk has invalid item data.
    pub fn diff(&self, other: &Self) -> TD0Result<Option<TD0ChunkDiff>> {
        let (my_default_item, other_default_item) = (self.default_item(), other.default_item());
        if !Self::chunk_item_types_comparable(&*my_default_item, &*other_default_item) {
            return Err(TD0Error::ChunksNotComparable);
        }

        // Compare header values: num_items and first_item_offset.
        let (my_header, other_header) = (self.header(), other.header());

        let (left, right) = (my_header.num_items(), other_header.num_items());
        let num_items: Option<Diff<usize>> = if left == right {
            None
        } else {
            Some(Diff { left, right })
        };

        let (left, right) = (
            my_header.first_item_offset(),
            other_header.first_item_offset(),
        );
        let first_item_offset: Option<Diff<usize>> = if left == right {
            None
        } else {
            Some(Diff { left, right })
        };

        // Compare items:
        let fields = my_default_item.list_fields();
        let mut chunk_items_diff: ChunkItemsDiff = ChunkItemsDiff::new();

        for (index, (left_bytes, right_bytes)) in
            self.items.iter().zip(other.items.iter()).enumerate()
        {
            if left_bytes != right_bytes {
                let mut fields_diff = FieldsDiff::new();
                // Retrieve these same bytes as TD0ChunkItems to compare their fields.
                let (left_obj, right_obj) = (
                    (self.callbacks.chunk_item_ref_from_bytes_cb)(left_bytes)
                        .map_err(|_| TD0Error::ChunkItemParse)?,
                    (other.callbacks.chunk_item_ref_from_bytes_cb)(right_bytes)
                        .map_err(|_| TD0Error::ChunkItemParse)?,
                );

                for field in fields {
                    let (left, right) = (
                        left_obj.field_value(field).unwrap(),
                        right_obj.field_value(field).unwrap(),
                    );
                    if left != right {
                        fields_diff.push((field.to_string(), Diff { left, right }));
                    }
                }

                chunk_items_diff.push((index, fields_diff));
            }
        }

        if num_items.is_some() || first_item_offset.is_some() || !chunk_items_diff.is_empty() {
            Ok(Some(TD0ChunkDiff {
                num_items,
                first_item_offset,
                items: if chunk_items_diff.is_empty() {
                    None
                } else {
                    Some(chunk_items_diff)
                },
            }))
        } else {
            Ok(None)
        }
    }

    /// Return the size in bytes of this `TD0Chunk`.
    pub fn size(&self) -> usize {
        self.header.len() + (self.items.first().unwrap().len() * self.items.len())
    }

    pub fn collect_buffers(&self) -> Vec<&BytesMut> {
        // Precalculate the number of buffers that will be processed.
        let mut all_bufs: Vec<&BytesMut> = Vec::with_capacity(self.items.len() + 1); // +1 for the chunk header.
        all_bufs.push(&self.header);
        all_bufs.extend(&self.items);

        all_bufs
    }

    pub fn item_at_raw(&self, index: usize) -> Option<&[u8]> {
        Some(self.items.get(index)?)
    }

    pub fn item_at_raw_mut(&mut self, index: usize) -> Option<&mut [u8]> {
        Some(self.items.get_mut(index)?)
    }

    /// Return a zeroed-out instance of this [`TD0Chunk`]'s [`TD0ChunkItem`] type.
    pub fn default_item(&self) -> Box<dyn TD0ChunkItem> {
        (self.callbacks.default_chunk_item_cb)()
    }

    /// Return a mutable reference to the [`TD0ChunkItem`] at `index`.
    ///
    /// # Errors
    /// [`TD0Error::OutOfRange`] - if `index` is greater than the number of items this chunk has.
    /// - [`TD0Error::ChunkItemParse`] if the chunk item in the file is invalid.
    pub fn item_at_mut(&mut self, index: usize) -> TD0Result<&mut dyn TD0ChunkItem> {
        if index < self.items.len() {
            (self.callbacks.chunk_item_mut_from_bytes_cb)(&mut self.items[index])
        } else {
            Err(TD0Error::out_of_range_error_from_usize(0, self.items.len()))
        }
    }

    pub fn num_items(&self) -> usize {
        self.header().num_items()
    }

    pub fn item_size(&self) -> usize {
        self.header().item_size()
    }

    /// Return an owned [`TD0ChunkItem`] copied from the item at `index`.
    ///
    /// # Errors
    /// [`TD0Error::OutOfRange`] - if `index` is greater than the number of items this chunk has.
    pub fn item_copy(&mut self, source_index: usize, dest_index: usize) -> TD0Result<()> {
        if source_index >= self.items.len() || dest_index >= self.items.len() {
            return Err(TD0Error::out_of_range_error_from_usize(0, self.items.len()));
        }

        // Can't use Vec::copy_within because BytesMut is not copyable.
        let tmp = self.items[source_index].clone();
        self.items[dest_index] = tmp;
        Ok(())
    }

    /// Replace the [`TD0ChunkItem`] at `pos` with `source`.
    ///
    /// # Errors
    /// - [`TD0Error::OutOfRange`] - if `index` is greater than the number of items this chunk has.
    /// - [`TD0Error::InvalidInput`] - if `source` does not match this chunk's item type.
    pub fn item_replace(&mut self, pos: usize, source: &dyn TD0ChunkItem) -> TD0Result<()> {
        let existing = self.item_at(pos)?;

        if !Self::chunk_item_types_comparable(&*existing, source) {
            return Err(TD0Error::InvalidInput("invalid item type".to_string()));
        }

        self.items[pos] = BytesMut::from(source.as_bytes());
        Ok(())
    }

    /// Replace the [`TD0ChunkItem`] at `pos` with `source`.
    ///
    /// # Errors
    /// - [`TD0Error::OutOfRange`] - if `index` is greater than the number of items this chunk has.
    pub fn items_swap(&mut self, index_1: usize, index_2: usize) -> TD0Result<()> {
        let items_len = self.items.len();
        if index_1 < items_len && index_2 < items_len {
            self.items.swap(index_1, index_2);
            Ok(())
        } else {
            Err(TD0Error::out_of_range_error_from_usize(0, items_len))
        }
    }

    /// Reorder all items in this chunk.
    ///
    /// # Errors
    /// - [`TD0Error::InvalidInput`] if `new_order` does not contain every
    ///   index from 0..[num items] (exclusive) exactly once.
    pub fn items_reorder(&mut self, new_order: &[usize]) -> TD0Result<()> {
        // Check preconditions
        // new_order must have correct number of elements
        // new order must visit contain every index from 0..chunk items len. (in any order.)
        let expected: Vec<usize> = (0usize..self.num_items()).collect();
        let mut param_sorted = new_order.to_vec();
        param_sorted.sort_unstable();
        if !expected.eq(&param_sorted) {
            return Err(TD0Error::InvalidInput(format!(
                "new_order must contain all indexes from 0..{} with no repeated entries.",
                self.num_items()
            )));
        }

        let mut new_items: Vec<BytesMut> = Vec::with_capacity(self.items.len());
        for &index in new_order {
            new_items.push(self.items[index].clone());
        }
        self.items = new_items;

        Ok(())
    }

    /// Return an array of [`FieldDef`] items describing this [`TD0Chunk`]'s fields.
    pub fn describe_fields(&self) -> &'static [FieldDef] {
        self.default_item().field_types()
    }

    /// Return an owned copy of the [`TD0ChunkItem`] at `index`.
    ///
    /// # Errors
    /// - [`TD0Error::OutOfRange`] if `index` is an invalid index.
    /// - [`TD0Error::ChunkItemParse`] if the chunk item in the file is invalid.
    pub fn item_at(&self, index: usize) -> TD0Result<Box<dyn TD0ChunkItem>> {
        if index < self.items.len() {
            (self.callbacks.chunk_item_owned_from_bytes_cb)(&self.items[index])
        } else {
            Err(TD0Error::out_of_range_error_from_usize(0, self.items.len()))
        }
    }

    /// Return a reference to the [`TD0ChunkItem`] at `index`.
    ///
    /// # Errors
    /// - [`TD0Error::OutOfRange`] if `index` is an invalid index.
    /// - [`TD0Error::ChunkItemParse`] if the chunk item in the file is invalid.
    pub fn item_at_ref(&self, index: usize) -> TD0Result<&dyn TD0ChunkItem> {
        if index < self.items.len() {
            (self.callbacks.chunk_item_ref_from_bytes_cb)(&self.items[index])
        } else {
            Err(TD0Error::out_of_range_error_from_usize(0, self.items.len()))
        }
    }

    pub const fn set_item_callbacks(&mut self, callbacks: ChunkItemAccessorCallbacks) {
        self.callbacks = callbacks;
    }
}

impl PartialEq for TD0Chunk {
    fn eq(&self, other: &Self) -> bool {
        // Compare the data, not the function pointers.
        self.header == other.header
            && self.chunk_extra_data == other.chunk_extra_data
            && self.items == other.items
    }
}

impl Eq for TD0Chunk {}

impl From<TD0Chunk> for Vec<u8> {
    fn from(value: TD0Chunk) -> Self {
        Self::from(&value)
    }
}

impl From<&TD0Chunk> for Vec<u8> {
    fn from(value: &TD0Chunk) -> Self {
        let mut output = BytesMut::new();
        for buf in value.collect_buffers() {
            output.extend(buf);
        }
        output.into()
    }
}

impl<'a> TryFrom<&'a [u8]> for TD0Chunk {
    type Error = TD0Error;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Self::try_from(BytesMut::from(value))
    }
}

impl TryFrom<BytesMut> for TD0Chunk {
    type Error = TD0Error;

    fn try_from(mut value: BytesMut) -> Result<Self, Self::Error> {
        use zerocopy::TryFromBytes;

        let bytes_size: usize = value.len();
        if bytes_size < size_of::<ChunkHeader>() {
            return Err(TD0Error::file_parse_error(ERR_CHUNK_HEADER_READ));
        }
        let header_bytes: BytesMut = value.split_to(size_of::<ChunkHeader>());
        let header: &ChunkHeader = ChunkHeader::try_ref_from_bytes(&header_bytes)
            .map_err(|_| TD0Error::file_parse_error(ERR_CHUNK_HEADER_PARSE))?;

        let pos = size_of::<ChunkHeader>();
        let chunk_extra_data: BytesMut = if pos < header.first_item_offset() {
            let sz_extra = header.first_item_offset() - pos;
            if value.len() < sz_extra {
                return Err(TD0Error::file_parse_error(ERR_CHUNK_READ));
            }
            value.split_to(sz_extra)
        } else {
            BytesMut::new()
        };

        if header.item_size() * header.num_items() != value.len() {
            return Err(TD0Error::file_parse_error(ERR_CHUNK_READ));
        }

        let mut items: Vec<BytesMut> = Vec::new();
        let sz_item = header.item_size();

        while !value.is_empty() {
            items.push(value.split_to(sz_item));
        }

        Ok(Self {
            header: header_bytes,
            chunk_extra_data,
            items,
            callbacks: ChunkItemAccessorCallbacks::default(),
        })
    }
}
