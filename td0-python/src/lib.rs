// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use pyo3::prelude::*;
use pyo3::{PyErr, PyResult};

/// A parsing and editing library for the .TD0 e-kit/sample pad backup file format.
///
/// This module comprises the Python3 bindings for the rust `td0` library.
/// (see: <https://crates.io/crates/td0> )
///
/// Usage:
/// Read the bytes from a .TD0 file, then instantiate a `TD0File` with the resulting
/// bytes or bytearray.
///
/// Example:
/// --------
/// ```
/// import libtd0
///
/// with open('my_backup.TD0', 'rb') as infile:
///     raw_bytes = infile.read()
///
/// td0file = libtd0.TD0File(raw_bytes)
/// print(td0file.list_chunks())
///
/// # Outputs similar to (this output varies based on device model, firmware version, and file backup type):
/// # ['HDRa', 'PVRa', 'KITb', 'KITa', 'STLa', 'WVPa', 'TGLa', 'STPb', 'STPa', 'TRGa', 'CURa']
/// ```
#[pymodule]
mod libtd0 {
    #[allow(
        clippy::wildcard_imports,
        reason = "the pyo3 imports above MUST be included before the module, and the module requires them."
    )]
    use super::*;
    use pyo3::exceptions::{PyIndexError, PyKeyError, PyRuntimeError, PyTypeError, PyValueError};
    use rust_decimal::prelude::FromPrimitive;
    use td0::{
        ChunkManifest as Impl_ChunkManifest, ChunkTagStr, FieldDef as Impl_FieldDef, FieldType,
        TD0BackupType as Impl_TD0BackupType, TD0Chunk as Impl_TD0Chunk,
        TD0ChunkItem as Impl_TD0ChunkItem, TD0DeviceModel as Impl_TD0DeviceModel, TD0Error,
        TD0File as Impl_TD0File, TD0Manifest as Impl_TD0Manifest, TD0Value as Impl_TD0Value,
        TD0ValueRaw as Impl_TD0ValueRaw, hex_encode, new_td0_file, parse_td0_file,
    };

    #[expect(
        clippy::needless_pass_by_value,
        reason = "simplicity. The error is not needed any longer."
    )]
    fn map_change_error(err: TD0Error) -> PyErr {
        let message: String = format!("{err}");
        match err {
            TD0Error::DataType(_) => PyErr::new::<PyTypeError, _>(message),
            TD0Error::ChecksumMismatch
            | TD0Error::ChunksNotComparable
            | TD0Error::DuplicateChunk(_)
            | TD0Error::InvalidChunkName
            | TD0Error::InvalidInput(_)
            | TD0Error::NotAsciiPrintable
            | TD0Error::UnsupportedDeviceFirmwareVersion(_)
            | TD0Error::UnsupportedDeviceModel(_)
            | TD0Error::OutOfRangeDecimal(_) => PyErr::new::<PyValueError, _>(message),
            TD0Error::InvalidChunk(_) | TD0Error::InvalidField(_) => {
                PyErr::new::<PyKeyError, _>(message)
            }
            TD0Error::OutOfRange(_) | TD0Error::InvalidIndex => {
                PyErr::new::<PyIndexError, _>(message)
            }
            _ => PyErr::new::<PyRuntimeError, _>(format!("{err}")),
        }
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "simplicity. The error is not needed any longer."
    )]
    fn map_read_error(err: TD0Error) -> PyErr {
        let message: String = format!("{err}");
        match err {
            TD0Error::DataType(_) => PyErr::new::<PyTypeError, _>(message),
            TD0Error::ChecksumMismatch
            | TD0Error::ChunksNotComparable
            | TD0Error::DuplicateChunk(_)
            | TD0Error::InvalidChunkName
            | TD0Error::InvalidInput(_)
            | TD0Error::NotAsciiPrintable
            | TD0Error::UnsupportedDeviceFirmwareVersion(_)
            | TD0Error::UnsupportedDeviceModel(_)
            | TD0Error::OutOfRangeDecimal(_) => PyErr::new::<PyValueError, _>(message),
            TD0Error::InvalidChunk(_) | TD0Error::InvalidField(_) => {
                PyErr::new::<PyKeyError, _>(message)
            }
            TD0Error::OutOfRange(_) | TD0Error::InvalidIndex => {
                PyErr::new::<PyIndexError, _>(message)
            }
            _ => PyErr::new::<PyRuntimeError, _>(format!("{err}")),
        }
    }

    /// Parse the device model and firmware version from raw bytes.
    ///
    /// This function may be used to check those values without needing completely parse
    /// the bytes into a `TD0File` object.
    #[pyfunction]
    fn parse_model_and_firmware(raw: &[u8]) -> PyResult<(TD0DeviceModel, String)> {
        Impl_TD0File::parse_model_and_firmware(raw)
            .map(|(impl_model, firmware)| (impl_model.into(), firmware))
            .map_err(map_read_error)
    }

    /// The type of backup a `TD0File` contains.
    ///
    /// Different backup types may contain different sets of chunks.
    #[pyclass(from_py_object)]
    #[derive(Copy, Clone)]
    enum TD0BackupType {
        Kit,
        System,
        Unknown,
    }

    impl From<Impl_TD0BackupType> for TD0BackupType {
        fn from(val: Impl_TD0BackupType) -> Self {
            match val {
                Impl_TD0BackupType::Kit => Self::Kit,
                Impl_TD0BackupType::System => Self::System,
                Impl_TD0BackupType::Unknown => Self::Unknown,
            }
        }
    }

    impl From<TD0BackupType> for Impl_TD0BackupType {
        fn from(val: TD0BackupType) -> Self {
            match val {
                TD0BackupType::Kit => Self::Kit,
                TD0BackupType::System => Self::System,
                TD0BackupType::Unknown => Self::Unknown,
            }
        }
    }

    /// The device model of the device that created the backup.
    #[pyclass(from_py_object)]
    #[derive(Copy, Clone)]
    enum TD0DeviceModel {
        SPDSXPro,
        Unknown,
    }

    impl From<Impl_TD0DeviceModel> for TD0DeviceModel {
        fn from(val: Impl_TD0DeviceModel) -> Self {
            match val {
                Impl_TD0DeviceModel::SPDSXPro => Self::SPDSXPro,
                Impl_TD0DeviceModel::Unknown => Self::Unknown,
            }
        }
    }

    impl From<TD0DeviceModel> for Impl_TD0DeviceModel {
        fn from(val: TD0DeviceModel) -> Self {
            match val {
                TD0DeviceModel::SPDSXPro => Self::SPDSXPro,
                TD0DeviceModel::Unknown => Self::Unknown,
            }
        }
    }

    /// User-friendly description of a `TD0File` data chunk.
    ///
    /// `TD0Manifest` structs contain one `ChunkManifest` item for each data
    /// chunk in the file.
    #[pyclass]
    struct ChunkManifest(Impl_ChunkManifest);

    #[pymethods]
    impl ChunkManifest {
        /// Retrieve the chunk name.
        #[getter]
        fn name(&self) -> String {
            self.0.name()
        }

        /// Retrieve the chunk position in the file in bytes.
        #[getter]
        const fn pos(&self) -> usize {
            self.0.pos()
        }

        /// Retrieve the chunk size in bytes.
        #[getter]
        const fn size(&self) -> usize {
            self.0.size()
        }

        /// Retrieve the number of `TD0ChunkItem` items the chunk contains.
        #[getter]
        const fn item_count(&self) -> usize {
            self.0.item_count()
        }

        /// Retrieve the item size of this chunk's `TD0ChunkItem` items.
        #[getter]
        const fn item_size(&self) -> usize {
            self.0.item_size()
        }

        fn __str__(&self) -> String {
            self.0.to_string()
        }

        fn __repr__(&self) -> String {
            format!(
                "ChunkManifest('{}', {}, {}, {}, {})",
                self.0.name(),
                self.0.pos(),
                self.0.size(),
                self.0.item_count(),
                self.0.item_size()
            )
        }
    }

    /// A user-friendly collection of metadata about the structure of a `TD0File`.
    #[pyclass]
    struct TD0Manifest(Impl_TD0Manifest);

    #[pymethods]
    impl TD0Manifest {
        /// Retrieve the saved backup name of this backup.
        #[getter]
        fn backup_name(&self) -> String {
            self.0.backup_name()
        }

        /// Retrieve the backup type.
        #[getter]
        fn backup_type(&self) -> TD0BackupType {
            TD0BackupType::from(self.0.backup_type())
        }

        /// Retrieve the declared file checkum.
        #[getter]
        fn checksum_actual(&self) -> String {
            self.0.checksum_actual()
        }

        /// Retrieve the calculated file checkum.
        #[getter]
        fn checksum_calculated(&self) -> String {
            self.0.checksum_calculated()
        }

        /// Retrieve the model of the device that created this backup.
        #[getter]
        fn device_model(&self) -> TD0DeviceModel {
            TD0DeviceModel::from(self.0.device_model())
        }

        /// Retrieve the firmware version of the device that created this backup.
        #[getter]
        fn device_firmware_version(&self) -> String {
            self.0.device_firmware_version()
        }

        /// Retrieve the firmware build of the device that created this backup.
        #[getter]
        fn device_firmware_build(&self) -> String {
            self.0.device_firmware_build()
        }

        /// Retrieve the serial number of the device that created this backup.
        #[getter]
        fn device_serial(&self) -> String {
            self.0.device_serial()
        }

        /// Retrieve the actual size in bytes of the `TD0File`.
        #[getter]
        const fn size_actual(&self) -> usize {
            self.0.size_actual()
        }

        /// Retrieve the calculated 'correct' size of the `TD0File`.
        #[getter]
        const fn size_calculated(&self) -> usize {
            self.0.size_calculated()
        }

        /// Retrieve a list of `ChunkManifest` items, one per `TD0Chunk` in the file.
        #[getter]
        fn chunks(&self) -> Vec<ChunkManifest> {
            self.0
                .chunks()
                .iter()
                .map(|ch| ChunkManifest(ch.clone()))
                .collect()
        }

        fn __str__(&self) -> String {
            self.0.to_string()
        }

        fn __repr__(&self) -> String {
            format!(
                "TD0Manifest(backup_name={:?}, backup_type={:?}, device_model={:?}, num_chunks={})",
                self.0.backup_name(),
                self.0.backup_type(),
                self.0.device_model(),
                self.0.chunks().len()
            )
        }
    }

    /// The data type a read returns or a write expects.
    #[pyclass]
    #[derive(Clone)]
    enum TD0ValueType {
        Decimal,
        Int,
        Bytes,
        String,
    }

    impl From<Impl_TD0Value> for TD0ValueType {
        fn from(val: Impl_TD0Value) -> Self {
            match val {
                Impl_TD0Value::Decimal(_) => Self::Decimal,
                Impl_TD0Value::Text(_) => Self::String,
                Impl_TD0Value::Slice(_) => Self::Bytes,
                Impl_TD0Value::I8(_)
                | Impl_TD0Value::I16(_)
                | Impl_TD0Value::U8(_)
                | Impl_TD0Value::U16(_)
                | Impl_TD0Value::U32(_) => Self::Int,
            }
        }
    }

    impl From<&FieldType> for TD0ValueType {
        fn from(value: &FieldType) -> Self {
            match value {
                FieldType::Decimal { .. } => Self::Decimal,
                FieldType::AsciiText { .. } | FieldType::EnumText { .. } => Self::String,
                FieldType::Slice { .. } => Self::Bytes,
                FieldType::I8 { .. }
                | FieldType::I16 { .. }
                | FieldType::U8 { .. }
                | FieldType::U16 { .. }
                | FieldType::U32 { .. } => Self::Int,
            }
        }
    }

    /// A value returned from a read of a `TD0ChunkItem` field.
    struct TD0Value(Impl_TD0Value);

    impl<'py> IntoPyObject<'py> for TD0Value {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            match self.0 {
                Impl_TD0Value::Decimal(val) => {
                    let dec_value = rust_decimal::Decimal::from_f32(val).ok_or_else(|| {
                        PyErr::new::<PyValueError, _>("Unable to convert native value to Decimal.")
                    })?;
                    Ok(dec_value.into_pyobject(py)?.into_any())
                }
                Impl_TD0Value::Text(val) => Ok(val.into_pyobject(py)?.into_any()),
                Impl_TD0Value::Slice(val) => Ok(val.into_pyobject(py)?.into_any()),
                Impl_TD0Value::I8(val) => Ok(val.into_pyobject(py)?.into_any()),
                Impl_TD0Value::I16(val) => Ok(val.into_pyobject(py)?.into_any()),
                Impl_TD0Value::U8(val) => Ok(val.into_pyobject(py)?.into_any()),
                Impl_TD0Value::U16(val) => Ok(val.into_pyobject(py)?.into_any()),
                Impl_TD0Value::U32(val) => Ok(val.into_pyobject(py)?.into_any()),
            }
        }
    }

    /// A value returned from a raw read of a `TD0ChunkItem` field.
    struct TD0ValueRaw(Impl_TD0ValueRaw);

    impl<'py> IntoPyObject<'py> for TD0ValueRaw {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            match self.0 {
                Impl_TD0ValueRaw::Slice(val) => Ok(val.into_pyobject(py)?.into_any()),
                Impl_TD0ValueRaw::I8(val) => Ok(val.into_pyobject(py)?.into_any()),
                Impl_TD0ValueRaw::I16(val) => Ok(val.into_pyobject(py)?.into_any()),
                Impl_TD0ValueRaw::U8(val) => Ok(val.into_pyobject(py)?.into_any()),
                Impl_TD0ValueRaw::U16(val) => Ok(val.into_pyobject(py)?.into_any()),
                Impl_TD0ValueRaw::U32(val) => Ok(val.into_pyobject(py)?.into_any()),
            }
        }
    }

    impl core::str::FromStr for TD0ValueType {
        type Err = PyErr;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                // These match the enum libtd0-derive::TD0FieldType .
                "Text" => Ok(Self::String),
                "I8" | "I16" | "U8" | "U16" | "U32" => Ok(Self::Int),
                "Slice" => Ok(Self::Bytes),
                "Decimal" => Ok(Self::Decimal),
                _ => Err(PyErr::new::<PyValueError, _>(format!(
                    "invalid value type: '{s}'"
                ))),
            }
        }
    }

    /// The type of value returned from a raw read of a `TD0ChunkItem` field.
    #[pyclass]
    enum TD0ValueTypeRaw {
        Int8,
        Int16,
        Bytes,
        UInt8,
        UInt16,
        UInt32,
    }

    /// The constraints that apply to Decimal field.
    #[pyclass(get_all)]
    #[derive(Clone)]
    struct DecimalConstraints {
        min: rust_decimal::Decimal,
        max: rust_decimal::Decimal,
    }

    /// The constraints that apply to an Int field.
    #[pyclass(get_all)]
    #[derive(Clone)]
    struct IntConstraints {
        min: i64,
        max: i64,
    }

    /// The constraints that apply to an Bytes field.
    #[pyclass(get_all)]
    #[derive(Clone)]
    struct BytesConstraints {
        len: usize,
    }

    /// The constraints that apply to an String field.
    ///
    /// - `max_len`: The maximum number of (ASCII printable only) characters the
    ///   value may contain.
    /// - `options`: A list of valid options for the field, or None if there are
    ///   no contraints.
    #[pyclass(get_all)]
    #[derive(Clone)]
    struct StringConstraints {
        max_len: Option<usize>,
        options: Option<&'static [&'static str]>,
    }

    /// A field description containing its `TD0ValueType` as well as any
    /// constraints that may apply to the field.
    ///
    /// Each <type_>_constraints field applies only to fields of that
    /// `field_type`. (e.g. if a `FieldDef` has a `field_type` of 'Decimal',
    /// only the `decimal_constraints` will apply. The remaining `*_constraints`
    /// members will all be set to None.)
    #[pyclass(get_all)]
    #[derive(Clone)]
    struct FieldDef {
        name: String,
        field_type: TD0ValueType,

        decimal_constraints: Option<DecimalConstraints>,
        int_constraints: Option<IntConstraints>,
        bytes_constraints: Option<BytesConstraints>,
        string_constraints: Option<StringConstraints>,
    }

    impl From<&Impl_FieldDef> for FieldDef {
        fn from(val: &Impl_FieldDef) -> Self {
            let mut decimal_constraints: Option<DecimalConstraints> = None;
            let mut int_constraints: Option<IntConstraints> = None;
            let mut bytes_constraints: Option<BytesConstraints> = None;
            let mut string_constraints: Option<StringConstraints> = None;

            match val.field_type {
                FieldType::AsciiText { max_len } => {
                    string_constraints = Some(StringConstraints {
                        max_len: Some(max_len),
                        options: None,
                    });
                }
                FieldType::EnumText { options } => {
                    string_constraints = Some(StringConstraints {
                        max_len: None,
                        options: Some(options),
                    });
                }
                FieldType::I8 { min, max } => {
                    int_constraints = Some(IntConstraints {
                        min: min.into(),
                        max: max.into(),
                    });
                }
                FieldType::I16 { min, max } => {
                    int_constraints = Some(IntConstraints {
                        min: min.into(),
                        max: max.into(),
                    });
                }
                FieldType::U8 { min, max } => {
                    int_constraints = Some(IntConstraints {
                        min: min.into(),
                        max: max.into(),
                    });
                }
                FieldType::U16 { min, max } => {
                    int_constraints = Some(IntConstraints {
                        min: min.into(),
                        max: max.into(),
                    });
                }
                FieldType::U32 { min, max } => {
                    int_constraints = Some(IntConstraints {
                        min: min.into(),
                        max: max.into(),
                    });
                }
                FieldType::Decimal { min, max } => {
                    let dec_min = rust_decimal::Decimal::from_f32(min)
                        .expect("TD0 decimal values are convertable to rust_decimal.");
                    let dec_max = rust_decimal::Decimal::from_f32(max)
                        .expect("TD0 decimal values are convertable to rust_decimal.");
                    decimal_constraints = Some(DecimalConstraints {
                        min: dec_min,
                        max: dec_max,
                    });
                }
                FieldType::Slice { len } => bytes_constraints = Some(BytesConstraints { len }),
            }

            Self {
                name: val.name.to_string(),
                field_type: TD0ValueType::from(&val.field_type),
                decimal_constraints,
                int_constraints,
                bytes_constraints,
                string_constraints,
            }
        }
    }

    /// A data chunk within a `TD0File`.
    ///
    /// A `TD0Chunk` consists of a chunk header, and a data section containing one or more
    /// `TD0ChunkItem` items, which can access or modify the file data. All `TD0ChunkItem`s
    /// within a `TD0Chunk` homogenous- they contain the same fields with the same data types,
    /// and differ only in the actual values they contain.
    ///
    /// `TD0Chunk` instances aren't created manually, instead retrieved from a `TD0File`
    /// via its `TD0File::chunk()` method.
    ///
    /// NOTE: In order to retain Rust's memory safety model, a `TD0Chunk` retrieved from
    /// a `TD0File` is a *copy* of the chunk data, not the underlying data itself. In
    /// order to write changes made to a chunk's items back to the file, the
    /// `TD0File::replace_chunk` method must be called.
    #[pyclass]
    struct TD0Chunk(pub Impl_TD0Chunk);

    impl From<&Impl_TD0Chunk> for TD0Chunk {
        fn from(other: &Impl_TD0Chunk) -> Self {
            Self(other.clone())
        }
    }

    impl From<&mut Impl_TD0Chunk> for TD0Chunk {
        fn from(other: &mut Impl_TD0Chunk) -> Self {
            Self(other.clone())
        }
    }

    #[pymethods]
    impl TD0Chunk {
        /// Copy the `TD0ChunkItem` at `source_index` to `dest_index`.
        ///
        /// The `TD0ChunkItem` previously at `dest_index` is overwritten.
        pub fn copy_item(&mut self, source_index: usize, dest_index: usize) -> PyResult<()> {
            self.0
                .copy_item(source_index, dest_index)
                .map_err(map_change_error)
        }

        /// Return a zeroed-out instance of this `TD0Chunk`'s `TD0ChunkItem` type.
        pub fn default_item(&self) -> TD0ChunkItem {
            TD0ChunkItem(self.0.default_item())
        }

        /// Return a list of `FieldDef` items describing this `TD0Chunk`'s fields.
        pub fn describe_fields(&self) -> Vec<FieldDef> {
            self.0
                .describe_fields()
                .iter()
                .map(|def: &Impl_FieldDef| FieldDef::from(def))
                .collect()
        }

        /*
        pub fn diff(&self, other: &Self) -> PyResult<Option<TD0ChunkDiff>> {
            todo!();
        }
        */

        /// Return the size in bytes of this `TD0Chunk`.
        pub fn size(&self) -> usize {
            self.0.size()
        }

        /// Return a copy of the `TD0ChunkItem` at `index`.
        ///
        /// NOTE: In order to retain Rust's memory safety model, a `TD0ChunkItem`
        /// retrieved from a `TD0Chunk` is a *copy* of the chunk data, not the
        /// underlying data itself. In  order to write changes made to a
        /// chunk's items back to the chunk, the `TD0Chunk::item_replace`
        /// method must be called.
        pub fn item(&self, index: usize) -> PyResult<TD0ChunkItem> {
            self.0.item(index).map_or_else(
                || Err(map_change_error(TD0Error::InvalidIndex)),
                |result| result.map(TD0ChunkItem).map_err(map_read_error),
            )
        }

        /// Return the number of `TD0ChunkItems` this chunk contains.
        fn item_count(&self) -> usize {
            self.0.item_count()
        }

        /// Return the size in bytes of of an individual `TD0ChunkItem`
        /// contained by this `TD0Chunk`.
        pub fn item_size(&self) -> usize {
            self.0.item_size()
        }

        /// Swap the `TD0ChunkItem`s at `index_1` and `index_2`.
        /// Reorder all items in this chunk.
        ///
        /// `new_order` must:
        /// - Contain all indexes from `0..[self.item_count() - 1]` inclusive,
        /// - No index may be repeated,
        /// - Must be of length `self.item_count()`
        #[expect(clippy::needless_pass_by_value, reason = "pyo3's API")]
        pub fn reorder_items(&mut self, new_order: Vec<usize>) -> PyResult<()> {
            self.0.reorder_items(&new_order).map_err(map_change_error)
        }

        /// Replace the `TD0ChunkItem` at `item_index` with another of the same type.
        #[expect(clippy::needless_pass_by_value, reason = "pyo3's API")]
        pub fn replace_item(
            &mut self,
            item_index: usize,
            source: Bound<'_, TD0ChunkItem>,
        ) -> PyResult<()> {
            self.0
                .replace_item(item_index, &*(source.borrow().0))
                .map_err(map_change_error)
        }

        pub fn swap_items(&mut self, index_1: usize, index_2: usize) -> PyResult<()> {
            self.0
                .swap_items(index_1, index_2)
                .map_err(map_change_error)
        }
    }

    /// Data accessors for individual data chunk items.
    ///
    /// (e.g. a single Kit configuration or a single sample metadata tag.)
    /// Contains methods to describe available fields, and read or modify
    /// those fields' values.
    ///
    /// `TD0ChunkItem` instances aren't created directly, but rather retrieved from
    /// a `TD0Chunk` instance.
    #[pyclass]
    struct TD0ChunkItem(Box<dyn Impl_TD0ChunkItem + Send + Sync>);

    #[pymethods]
    impl TD0ChunkItem {
        /// Return a list valid options for field `field`.
        ///
        /// Return None if the field doesn't have a restricted set of options.
        ///
        fn field_options(&self, field: &str) -> Option<Vec<String>> {
            self.0
                .field_options(field)
                .map(|slice| slice.iter().map(std::string::ToString::to_string).collect())
        }

        /// Return the value of `field` as a `TD0Value`, or None if `field` is invalid.
        fn field_value(&self, field: &str) -> PyResult<TD0Value> {
            let val: Impl_TD0Value = self
                .0
                .field_value(field)
                .ok_or_else(|| PyErr::new::<PyKeyError, _>(format!("unknown field '{field}'")))?;
            Ok(TD0Value(val))
        }

        /// Return the raw value of `field` as a `TD0ValueRaw`, or None if `field` is invalid.
        fn field_value_raw(&self, field: &str) -> PyResult<TD0ValueRaw> {
            let val: Impl_TD0ValueRaw = self
                .0
                .field_value_raw(field)
                .ok_or_else(|| PyErr::new::<PyKeyError, _>(format!("unknown field '{field}'")))?;
            Ok(TD0ValueRaw(val))
        }

        /// Return the `TD0ValueType` of `field` or None if `field` is invalid.
        fn field_type(&self, field: &str) -> Option<TD0ValueType> {
            self.0.field_type(field).map(TD0ValueType::from)
        }

        /// Return a list of this chunk item's field names.
        fn list_fields(&self) -> &'static [&'static str] {
            self.0.list_fields()
        }

        /// Set the value of `field` to `TD0Value` `value`.
        #[expect(clippy::needless_pass_by_value, reason = "pyo3's API")]
        #[expect(clippy::elidable_lifetime_names, reason = "pyo3's API")]
        fn set_field_value<'py>(&mut self, field: &str, value: Bound<'py, PyAny>) -> PyResult<()> {
            let val: Impl_TD0Value = match self.0.field_type(field) {
                Some(FieldType::AsciiText { .. } | FieldType::EnumText { .. }) => {
                    Impl_TD0Value::Text(value.extract::<String>()?)
                }
                Some(FieldType::I8 { .. }) => Impl_TD0Value::I8(value.extract::<i8>()?),
                Some(FieldType::I16 { .. }) => Impl_TD0Value::I16(value.extract::<i16>()?),
                Some(FieldType::Slice { .. }) => {
                    Impl_TD0Value::Slice(value.extract::<Vec<u8>>()?.into_boxed_slice())
                }
                Some(FieldType::Decimal { .. }) => {
                    let dec_value = value.extract::<rust_decimal::Decimal>()?;
                    Impl_TD0Value::Decimal(f32::try_from(dec_value).map_err(|err| {
                        PyErr::new::<PyValueError, _>(format!("Out of range: {err}"))
                    })?)
                }
                Some(FieldType::U8 { .. }) => Impl_TD0Value::U8(value.extract::<u8>()?),
                Some(FieldType::U16 { .. }) => Impl_TD0Value::U16(value.extract::<u16>()?),
                Some(FieldType::U32 { .. }) => Impl_TD0Value::U32(value.extract::<u32>()?),
                None => {
                    return Err(PyErr::new::<PyKeyError, _>(format!(
                        "unknown field '{field}'"
                    )));
                }
            };
            self.0
                .set_field_value(field, val)
                .map_err(|err| PyErr::new::<PyValueError, _>(format!("{err}")))
        }

        #[expect(clippy::needless_pass_by_value, reason = "pyo3's API")]
        #[expect(clippy::elidable_lifetime_names, reason = "pyo3's API")]
        fn set_field_value_raw<'py>(
            &mut self,
            field: &str,
            value: Bound<'py, PyAny>,
        ) -> PyResult<()> {
            let val: Impl_TD0ValueRaw = match self.0.field_type(field) {
                Some(
                    FieldType::Slice { .. }
                    | FieldType::AsciiText { .. }
                    | FieldType::EnumText { .. },
                ) => Impl_TD0ValueRaw::Slice(value.extract::<Vec<u8>>()?.into_boxed_slice()),
                Some(FieldType::I8 { .. }) => Impl_TD0ValueRaw::I8(value.extract::<i8>()?),
                Some(FieldType::I16 { .. } | FieldType::Decimal { .. }) => {
                    Impl_TD0ValueRaw::I16(value.extract::<i16>()?)
                }
                Some(FieldType::U8 { .. }) => Impl_TD0ValueRaw::U8(value.extract::<u8>()?),
                Some(FieldType::U16 { .. }) => Impl_TD0ValueRaw::U16(value.extract::<u16>()?),
                Some(FieldType::U32 { .. }) => Impl_TD0ValueRaw::U32(value.extract::<u32>()?),
                None => {
                    return Err(PyErr::new::<PyKeyError, _>(format!(
                        "unknown field '{field}'"
                    )));
                }
            };
            self.0
                .set_field_value_raw(field, val)
                .map_err(map_change_error)
        }
    }

    /// The parsed, in-memory representation of a .TD0 backup file.
    ///
    /// A `TD0File` consists of file manifest, which lists the file's data chunks
    /// as well as the data chunks themselves (represented in this library as
    /// `TD0Chunk` structs.) `TD0Chunk`s contain one or more items, structs that
    /// implement the `TD0ChunkItem` trait, which provide access to the values
    /// (e.g. Kit config data, wav file metadata, playlist items) stored within the
    /// backup file.
    ///
    /// # Parameters:
    /// - `data`: `collections.abc.Buffer|None` - The byte data from a backup file.
    ///   If omitted, a default empty `TD0File` is created.
    #[pyclass]
    struct TD0File(Impl_TD0File);

    #[pymethods]
    impl TD0File {
        #[new]
        #[pyo3(signature = (data = None))]
        pub fn new(data: Option<Vec<u8>>) -> PyResult<Self> {
            let td0file = match data {
                Some(has_data) => parse_td0_file(&has_data).map_err(|err| {
                    PyErr::new::<PyValueError, _>(format!("Unable to parse input buffer: {err}"))
                })?,
                None => new_td0_file(Impl_TD0DeviceModel::SPDSXPro, "1.10").map_err(|err| {
                    PyErr::new::<PyValueError, _>(format!(
                        "Unable to create default TD0File: {err}"
                    ))
                })?,
            };

            Ok(Self(td0file))
        }

        /// Add a chunk of `chunk_name` type to the file, containing `item_count`
        /// `TD0ChunkItem` items of the appropriate type.
        ///
        /// The newly created items will be default `TD0ChunkItem` items of the
        /// expected type for `chunk_name`.
        pub fn add_chunk(&mut self, chunk_name: &str, item_count: usize) -> PyResult<TD0Chunk> {
            let chunk_name: &ChunkTagStr = chunk_name
                .try_into()
                .map_err(|err: TD0Error| PyErr::new::<PyValueError, _>(err.to_string()))?;
            let impl_chunk = self
                .0
                .add_chunk(chunk_name, item_count)
                .map_err(map_change_error)?;
            Ok(TD0Chunk::from(impl_chunk))
        }

        /// Add a chunk of `chunk_name` type to the file, without verifying
        /// that it is appropriate for the file.
        ///
        /// A default `TD0ChunkItem` must be supplied, which will be copied
        /// `item_count` times to create the chunk buffer.
        #[expect(clippy::needless_pass_by_value, reason = "pyo3's API")]
        pub fn add_chunk_unchecked(
            &mut self,
            chunk_name: &str,
            item_count: usize,
            default_item: Bound<'_, TD0ChunkItem>,
        ) -> PyResult<TD0Chunk> {
            let chunk_name: &ChunkTagStr = chunk_name
                .try_into()
                .map_err(|err: TD0Error| PyErr::new::<PyValueError, _>(err.to_string()))?;
            let impl_chunk = self
                .0
                .add_chunk_unchecked(chunk_name, item_count, &*(default_item.borrow()).0)
                .map_err(map_change_error)?;
            Ok(TD0Chunk::from(impl_chunk))
        }

        /// Calculate the checksum for this file's data.
        ///
        /// The file's data in this case means the entirety of the file *except* the included
        /// checksum. The calculated checksum should match the included checksum if the file
        /// hasn't been modified.
        pub fn calc_checksum(&self) -> String {
            hex_encode(&self.0.calc_checksum())
        }

        /// Retrieve a copy of `TD0Chunk` `chunk_name`, or None if no such chunk exists.
        pub fn chunk(&self, chunk_name: &str) -> Option<TD0Chunk> {
            self.0.chunk(chunk_name).map(TD0Chunk::from)
        }

        /// Return a `ChunkManifest` containing user-friendly metadata about `chunk_name`.
        pub fn chunk_info(&self, chunk_name: &str) -> Option<ChunkManifest> {
            self.0.chunk_info(chunk_name).map(ChunkManifest)
        }

        /*
        pub fn diff(&self, other: &Self) -> PyResult<Option<TD0FileDiff>> {
            todo!();
        }
        */

        /// Return the firmware version of this file.
        pub fn firmware_version(&self) -> String {
            self.0.firmware_version().to_string()
        }

        /// Return true if this file contains a chunk named `chunk_name`.
        pub fn has_chunk(&self, chunk_name: &str) -> bool {
            self.0.has_chunk(chunk_name)
        }

        /// Return a list of the names of the chunks present in this file.
        ///
        /// ## Note
        /// This does not list all possible valid chunk names, only those that exist in
        /// this file. To retrieve all valid chunk names for this type of file, use the
        /// `TD0File::valid_chunks` method.
        pub fn list_chunks(&self) -> Vec<String> {
            self.0
                .list_chunks()
                .iter()
                .map(std::string::ToString::to_string)
                .collect()
        }

        /// Return a `TD0Manifest` containing metadata about the file and its data chunks.
        pub fn manifest(&self) -> TD0Manifest {
            TD0Manifest(self.0.manifest())
        }

        /// Return the `TD0DeviceModel` of this backup `TD0File`.
        pub fn model(&self) -> TD0DeviceModel {
            self.0.model().into()
        }

        /// Read the declared checksum from this backup file.
        pub fn read_checksum(&self) -> String {
            hex_encode(&self.0.read_checksum())
        }

        /// Remove the chunk `chunk_name` from the file.
        pub fn remove_chunk(&mut self, chunk_name: &str) -> PyResult<()> {
            self.0.remove_chunk(chunk_name).map_err(map_change_error)
        }

        /// Replace the chunk `chunk_name` with the new chunk `new_chunk`.
        ///
        /// NOTE: In order to retain Rust's memory safety model, a `TD0Chunk` retrieved from
        /// a `TD0File` is a *copy* of the chunk data, not the underlying data itself. In
        /// order to write changes made to a chunk's items back to the file, this method
        /// method must be called.
        #[expect(clippy::needless_pass_by_value, reason = "pyo3's API")]
        pub fn replace_chunk(
            &mut self,
            chunk_name: &str,
            chunk: Bound<'_, TD0Chunk>,
        ) -> PyResult<()> {
            self.0
                .replace_chunk(chunk_name, (chunk.borrow().0).clone())
                .map_err(map_change_error)
        }

        /// Set the firmware version of this file.
        pub fn set_firmware_version(&mut self, version: &str) -> PyResult<()> {
            self.0
                .set_firmware_version(version)
                .map_err(map_change_error)
        }

        /// Return the size of this file in bytes.
        pub fn size(&self) -> usize {
            self.0.size()
        }

        /// Export this file as raw bytes.
        ///
        /// A required step for saving the file to disk.
        pub fn to_bytes(&self) -> Vec<u8> {
            std::convert::Into::<Vec<u8>>::into(self.0.clone())
        }

        /// Return a list of the valid chunk names for this `TD0File`.
        ///
        /// Valid chunks are based on its device model and firmware revision.
        ///
        /// ## Note
        /// All possible valid chunk names are returned, regardless of their presence
        /// or absence in the current file. To retrieve only the chunks found in the
        /// current file, use the `TD0File::list_chunks` method.
        fn valid_chunks(&self) -> &'static [&'static str] {
            self.0.valid_chunks()
        }
    }
}
