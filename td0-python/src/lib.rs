// SPDX-FileCopyrightText: © 2026 Nathan Pugh <natepugh@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use pyo3::prelude::*;
use pyo3::{PyErr, PyResult};

#[pymodule]
mod libtd0 {
    #[allow(
        clippy::wildcard_imports,
        reason = "the pyo3 imports above MUST be included before the module, and the module requires them."
    )]
    use super::*;
    use pyo3::exceptions::{PyKeyError, PyRuntimeError, PyValueError};
    use rust_decimal::prelude::FromPrimitive;
    use td0::{FieldType, TD0Chunk, TD0Error, TD0Result, new_td0_file, parse_td0_file};
    use td0_core::{
        ChunkManifest as Impl_ChunkManifest, ChunkTagStr, TD0BackupType as Impl_TD0BackupType,
        TD0ChunkItem as Impl_TD0ChunkItem, TD0DeviceModel as Impl_TD0DeviceModel,
        TD0File as Impl_TD0File, TD0Manifest as Impl_TD0Manifest, TD0Value as Impl_TD0Value,
        TD0ValueRaw as Impl_TD0ValueRaw,
    };

    fn map_chunk_item_error(err: TD0Error) -> PyErr {
        match err {
            TD0Error::InvalidChunk(chunk_err) => {
                PyErr::new::<PyValueError, _>(chunk_err.to_string())
            }
            TD0Error::ChunkItemParse => PyErr::new::<PyValueError, _>("Invalid chunk item index"),
            _ => PyErr::new::<PyRuntimeError, _>(format!("{err}")),
        }
    }

    fn chunk<'a>(td0file: &'a Impl_TD0File, chunk_name: &str) -> TD0Result<&'a TD0Chunk> {
        let chunk_name: &ChunkTagStr = chunk_name.try_into()?;
        td0file
            .chunk(chunk_name)
            .ok_or_else(|| TD0Error::unknown_chunk_error(chunk_name))
    }

    fn chunk_mut<'a>(
        td0file: &'a mut Impl_TD0File,
        chunk_name: &str,
    ) -> TD0Result<&'a mut TD0Chunk> {
        let chunk_name: &ChunkTagStr = chunk_name.try_into()?;
        td0file
            .chunk_mut(chunk_name)
            .ok_or_else(|| TD0Error::unknown_chunk_error(chunk_name))
    }

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

    #[pyclass]
    struct ChunkManifest(Impl_ChunkManifest);

    #[pymethods]
    impl ChunkManifest {
        #[getter]
        fn name(&self) -> String {
            self.0.name()
        }

        #[getter]
        const fn pos(&self) -> usize {
            self.0.pos()
        }

        #[getter]
        const fn size(&self) -> usize {
            self.0.size()
        }

        #[getter]
        const fn num_items(&self) -> usize {
            self.0.num_items()
        }

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
                self.0.num_items(),
                self.0.item_size()
            )
        }
    }

    #[pyclass]
    struct TD0Manifest(Impl_TD0Manifest);

    #[pymethods]
    impl TD0Manifest {
        #[getter]
        fn backup_name(&self) -> String {
            self.0.backup_name()
        }

        #[getter]
        fn backup_type(&self) -> TD0BackupType {
            TD0BackupType::from(self.0.backup_type())
        }

        #[getter]
        fn checksum_actual(&self) -> String {
            self.0.checksum_actual()
        }

        #[getter]
        fn checksum_calculated(&self) -> String {
            self.0.checksum_calculated()
        }

        #[getter]
        fn device_model(&self) -> TD0DeviceModel {
            TD0DeviceModel::from(self.0.device_model())
        }

        #[getter]
        fn device_firmware_version(&self) -> String {
            self.0.device_firmware_version()
        }

        #[getter]
        fn device_firmware_build(&self) -> String {
            self.0.device_firmware_build()
        }

        #[getter]
        fn device_serial(&self) -> String {
            self.0.device_serial()
        }

        #[getter]
        const fn size_actual(&self) -> usize {
            self.0.size_actual()
        }

        #[getter]
        const fn size_calculated(&self) -> usize {
            self.0.size_calculated()
        }

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

    #[pyclass]
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

    #[pyclass]
    enum TD0ValueTypeRaw {
        Int8,
        Int16,
        Bytes,
        UInt8,
        UInt16,
        UInt32,
    }

    #[pyclass]
    struct TD0ChunkItem(Box<dyn Impl_TD0ChunkItem>);

    #[pymethods]
    impl TD0ChunkItem {
        fn field_options(&self, field: &str) -> Option<Vec<String>> {
            self.0
                .field_options(field)
                .map(|slice| slice.iter().map(std::string::ToString::to_string).collect())
        }

        fn field_value(&self, field: &str) -> PyResult<TD0Value> {
            let val: Impl_TD0Value = self
                .0
                .field_value(field)
                .ok_or_else(|| PyErr::new::<PyKeyError, _>(format!("unknown field '{field}'")))?;
            Ok(TD0Value(val))
        }

        fn field_value_raw(&self, field: &str) -> PyResult<TD0ValueRaw> {
            let val: Impl_TD0ValueRaw = self
                .0
                .field_value_raw(field)
                .ok_or_else(|| PyErr::new::<PyKeyError, _>(format!("unknown field '{field}'")))?;
            Ok(TD0ValueRaw(val))
        }

        fn field_type(&self, field: &str) -> Option<TD0ValueType> {
            self.0.field_type(field).map(TD0ValueType::from)
        }

        fn list_fields(&self) -> &'static [&'static str] {
            self.0.list_fields()
        }

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
                .map_err(|err| PyErr::new::<PyValueError, _>(format!("{err}")))
        }
    }

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

        // TODO: Add the concept of TD0Chunk to Python libtd0, then return the value from 0.add_chunk.
        pub fn add_chunk(&mut self, chunk_name: &str, num_items: usize) -> PyResult<()> {
            let chunk_name: &ChunkTagStr = chunk_name
                .try_into()
                .map_err(|err: TD0Error| PyErr::new::<PyValueError, _>(err.to_string()))?;
            _ = self
                .0
                .add_chunk(chunk_name, num_items)
                .map_err(|err| PyErr::new::<PyRuntimeError, _>(err.to_string()))?;
            Ok(())
        }

        #[expect(clippy::needless_pass_by_value, reason = "pyo3's API")]
        pub fn chunk_item_replace(
            &mut self,
            chunk_name: &str,
            item_index: usize,
            source: Bound<'_, TD0ChunkItem>,
        ) -> PyResult<()> {
            let chunk = chunk_mut(&mut self.0, chunk_name)
                .map_err(|err| PyErr::new::<PyRuntimeError, _>(err.to_string()))?;
            chunk
                .item_replace(item_index, &*(source.borrow().0))
                .map_err(|err| PyErr::new::<PyRuntimeError, _>(err.to_string()))
        }

        pub fn chunk_items_copy(
            &mut self,
            chunk_name: &str,
            source_index: usize,
            dest_index: usize,
        ) -> PyResult<()> {
            let chunk = chunk_mut(&mut self.0, chunk_name)
                .map_err(|err| PyErr::new::<PyRuntimeError, _>(err.to_string()))?;
            chunk
                .item_copy(source_index, dest_index)
                .map_err(|err| PyErr::new::<PyRuntimeError, _>(err.to_string()))
        }

        pub fn chunk_items_swap(
            &mut self,
            chunk_name: &str,
            index_1: usize,
            index_2: usize,
        ) -> PyResult<()> {
            let chunk = chunk_mut(&mut self.0, chunk_name)
                .map_err(|err| PyErr::new::<PyRuntimeError, _>(err.to_string()))?;
            chunk
                .items_swap(index_1, index_2)
                .map_err(|err| PyErr::new::<PyRuntimeError, _>(err.to_string()))
        }

        #[expect(clippy::needless_pass_by_value, reason = "pyo3's API")]
        pub fn chunk_items_reorder(
            &mut self,
            chunk_name: &str,
            new_order: Vec<usize>,
        ) -> PyResult<()> {
            let chunk = chunk_mut(&mut self.0, chunk_name)
                .map_err(|err| PyErr::new::<PyRuntimeError, _>(err.to_string()))?;
            chunk
                .items_reorder(&new_order)
                .map_err(|err| PyErr::new::<PyValueError, _>(err.to_string()))
        }

        pub fn to_bytes(&self) -> Vec<u8> {
            std::convert::Into::<Vec<u8>>::into(self.0.clone())
        }

        pub fn get_chunk_item(
            &self,
            chunk_name: &str,
            item_index: usize,
        ) -> PyResult<TD0ChunkItem> {
            let chunk = chunk(&self.0, chunk_name)
                .map_err(|err| PyErr::new::<PyRuntimeError, _>(err.to_string()))?;

            chunk
                .item_at(item_index)
                .map(TD0ChunkItem)
                .map_err(map_chunk_item_error)
        }

        fn get_chunk_item_default(&self, chunk_name: &str) -> Option<TD0ChunkItem> {
            let chunk = chunk(&self.0, chunk_name).ok()?;
            Some(TD0ChunkItem(chunk.default_item()))
        }

        pub fn list_chunks(&self) -> Vec<String> {
            self.0
                .list_chunks()
                .iter()
                .map(std::string::ToString::to_string)
                .collect()
        }

        pub fn manifest(&self) -> TD0Manifest {
            TD0Manifest(self.0.manifest())
        }
    }
}
