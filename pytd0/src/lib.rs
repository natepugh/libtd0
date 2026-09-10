use pyo3::prelude::*;
use pyo3::{PyErr, PyResult};

#[pymodule]
mod pytd0 {
    use super::*;
    use libtd0_core::{
        ChunkManifest as Impl_ChunkManifest, TD0BackupType as Impl_TD0BackupType,
        TD0ChunkItem as Impl_TD0ChunkItem, TD0DeviceModel as Impl_TD0DeviceModel,
        TD0File as Impl_TD0File, TD0Manifest as Impl_TD0Manifest, TD0Value as Impl_TD0Value,
        TD0ValueRaw as Impl_TD0ValueRaw, checksum_bytes_to_string, result::TD0Error,
    };
    use multi_model::{new_td0_file, parse_td0_file};
    use pyo3::exceptions::{PyKeyError, PyRuntimeError, PyValueError};

    fn map_chunk_item_error(err: TD0Error) -> PyErr {
        match err {
            TD0Error::UnknownChunk => PyErr::new::<PyValueError, _>("Unknown chunk"),
            TD0Error::InvalidChunkItem => PyErr::new::<PyValueError, _>("Invalid chunk item index"),
            _ => PyErr::new::<PyRuntimeError, _>(format!("{err}")),
        }
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
        fn name(&self) -> PyResult<String> {
            Ok(self.0.name.clone())
        }
        #[setter]
        fn set_name(&mut self, val: String) -> PyResult<()> {
            self.0.name = val;
            Ok(())
        }

        #[getter]
        fn pos(&self) -> PyResult<usize> {
            Ok(self.0.pos)
        }
        #[setter]
        fn set_pos(&mut self, val: usize) -> PyResult<()> {
            self.0.pos = val;
            Ok(())
        }

        #[getter]
        fn size(&self) -> PyResult<usize> {
            Ok(self.0.size)
        }
        #[setter]
        fn set_size(&mut self, val: usize) -> PyResult<()> {
            self.0.size = val;
            Ok(())
        }

        #[getter]
        fn num_items(&self) -> PyResult<usize> {
            Ok(self.0.num_items)
        }
        #[setter]
        fn set_num_items(&mut self, val: usize) -> PyResult<()> {
            self.0.num_items = val;
            Ok(())
        }

        #[getter]
        fn item_size(&self) -> PyResult<usize> {
            Ok(self.0.item_size)
        }
        #[setter]
        fn set_item_size(&mut self, val: usize) -> PyResult<()> {
            self.0.item_size = val;
            Ok(())
        }

        fn __str__(&self) -> String {
            self.0.to_string()
        }

        fn __repr__(&self) -> String {
            format!(
                "ChunkManifest('{}', {}, {}, {}, {})",
                self.0.name, self.0.pos, self.0.size, self.0.num_items, self.0.item_size
            )
        }
    }

    #[pyclass]
    struct TD0Manifest(Impl_TD0Manifest);

    #[pymethods]
    impl TD0Manifest {
        #[getter]
        fn backup_name(&self) -> PyResult<String> {
            Ok(self.0.backup_name.clone())
        }

        #[getter]
        fn backup_type(&self) -> PyResult<TD0BackupType> {
            Ok(TD0BackupType::from(self.0.backup_type))
        }

        #[getter]
        fn checksum_actual(&self) -> PyResult<String> {
            Ok(checksum_bytes_to_string(&self.0.checksum_actual))
        }

        #[getter]
        fn checksum_calculated(&self) -> PyResult<String> {
            Ok(checksum_bytes_to_string(&self.0.checksum_calculated))
        }

        #[getter]
        fn device_model(&self) -> PyResult<TD0DeviceModel> {
            Ok(TD0DeviceModel::from(self.0.device_model))
        }

        #[getter]
        fn device_firmware_version(&self) -> PyResult<String> {
            Ok(self.0.device_firmware_version.clone())
        }

        #[getter]
        fn device_firmware_build(&self) -> PyResult<String> {
            Ok(self.0.device_firmware_build.clone())
        }

        #[getter]
        fn device_serial(&self) -> PyResult<String> {
            Ok(self.0.device_serial.clone())
        }

        #[getter]
        fn size_actual(&self) -> PyResult<usize> {
            Ok(self.0.size_actual)
        }

        #[getter]
        fn size_calculated(&self) -> PyResult<usize> {
            Ok(self.0.size_calculated)
        }

        #[getter]
        fn chunks(&self) -> PyResult<Vec<ChunkManifest>> {
            Ok(self
                .0
                .chunks
                .iter()
                .map(|ch| ChunkManifest(ch.clone()))
                .collect())
        }

        fn __str__(&self) -> String {
            self.0.to_string()
        }

        fn __repr__(&self) -> String {
            format!(
                "TD0Manifest(backup_name={:?}, backup_type={:?}, device_model={:?}, num_chunks={})",
                self.0.backup_name,
                self.0.backup_type,
                self.0.device_model,
                self.0.chunks.len()
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
                Impl_TD0Value::I8(_) | Impl_TD0Value::I16(_) => Self::Int,
                Impl_TD0Value::U8(_) | Impl_TD0Value::U16(_) | Impl_TD0Value::U32(_) => Self::Int,
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
                Impl_TD0Value::Decimal(val) => Ok(val.into_pyobject(py)?.into_any()),
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
                "EnumStr" | "Text" => Ok(Self::String),
                "I8" | "I16" => Ok(Self::Int),
                "Slice" => Ok(Self::Bytes),
                "TD0Decimal" | "Volume" => Ok(Self::Decimal),
                "U8" | "U16" | "U32" => Ok(Self::Int),
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
    struct TD0ChunkItem {
        pub _impl: Box<dyn Impl_TD0ChunkItem>,
    }

    #[pymethods]
    impl TD0ChunkItem {
        fn get_value(&self, field: &str) -> PyResult<TD0Value> {
            let val: Impl_TD0Value = self._impl.get_value(field).ok_or(PyErr::new::<
                PyKeyError,
                _,
            >(format!(
                "unknown field '{field}'"
            )))?;
            Ok(TD0Value(val))
        }

        fn get_value_raw(&self, field: &str) -> PyResult<TD0ValueRaw> {
            let val: Impl_TD0ValueRaw =
                self._impl
                    .get_value_raw(field)
                    .ok_or(PyErr::new::<PyKeyError, _>(format!(
                        "unknown field '{field}'"
                    )))?;
            Ok(TD0ValueRaw(val))
        }

        fn get_field_type(&self, field: &str) -> Option<TD0ValueType> {
            self._impl.get_field_type(field).map(|val| {
                val.parse::<TD0ValueType>()
                    .expect("all field types accounted for.")
            })
        }

        fn list_fields(&self) -> &'static [&'static str] {
            self._impl.list_fields()
        }

        fn set_value<'py>(&mut self, field: &str, value: Bound<'py, PyAny>) -> PyResult<()> {
            let val: Impl_TD0Value = match self._impl.get_field_type(field) {
                Some("EnumStr") | Some("Text") => Impl_TD0Value::Text(value.extract::<String>()?),
                Some("I8") => Impl_TD0Value::I8(value.extract::<i8>()?),
                Some("I16") => Impl_TD0Value::I16(value.extract::<i16>()?),
                Some("Slice") => {
                    Impl_TD0Value::Slice(value.extract::<Vec<u8>>()?.into_boxed_slice())
                }
                Some("TD0Decimal") | Some("Volume") => {
                    Impl_TD0Value::Decimal(value.extract::<f32>()?)
                }
                Some("U8") => Impl_TD0Value::U8(value.extract::<u8>()?),
                Some("U16") => Impl_TD0Value::U16(value.extract::<u16>()?),
                Some("U32") => Impl_TD0Value::U32(value.extract::<u32>()?),
                Some(unknown) => {
                    return Err(PyErr::new::<PyKeyError, _>(format!(
                        "unknown field type '{unknown}'"
                    )));
                }
                None => {
                    return Err(PyErr::new::<PyKeyError, _>(format!(
                        "unknown field '{field}'"
                    )));
                }
            };
            self._impl
                .set_value(field, &val)
                .map_err(|err| PyErr::new::<PyValueError, _>(format!("{err}")))
        }

        fn set_value_raw<'py>(&mut self, field: &str, value: Bound<'py, PyAny>) -> PyResult<()> {
            let val: Impl_TD0ValueRaw = match self._impl.get_field_type(field) {
                Some("EnumStr") | Some("Slice") | Some("Text") => {
                    Impl_TD0ValueRaw::Slice(value.extract::<Vec<u8>>()?.into_boxed_slice())
                }
                Some("I8") => Impl_TD0ValueRaw::I8(value.extract::<i8>()?),
                Some("I16") => Impl_TD0ValueRaw::I16(value.extract::<i16>()?),
                Some("TD0Decimal") | Some("Volume") => {
                    Impl_TD0ValueRaw::I16(value.extract::<i16>()?)
                }
                Some("U8") => Impl_TD0ValueRaw::U8(value.extract::<u8>()?),
                Some("U16") => Impl_TD0ValueRaw::U16(value.extract::<u16>()?),
                Some("U32") => Impl_TD0ValueRaw::U32(value.extract::<u32>()?),
                Some(unknown) => {
                    return Err(PyErr::new::<PyKeyError, _>(format!(
                        "unknown field type '{unknown}'"
                    )));
                }
                None => {
                    return Err(PyErr::new::<PyKeyError, _>(format!(
                        "unknown field '{field}'"
                    )));
                }
            };
            self._impl
                .set_value_raw(field, &val)
                .map_err(|err| PyErr::new::<PyValueError, _>(format!("{err}")))
        }
    }

    #[pyclass]
    struct TD0File {
        _impl: Box<dyn Impl_TD0File>,
    }

    #[pymethods]
    impl TD0File {
        #[new]
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

            Ok(Self { _impl: td0file })
        }

        pub fn chunk_items_copy(
            &mut self,
            chunk_name: &str,
            source_index: usize,
            dest_index: usize,
        ) -> PyResult<()> {
            self._impl
                .chunk_items_copy(chunk_name, source_index, dest_index)
                .map_err(map_chunk_item_error)?;
            Ok(())
        }

        pub fn chunk_items_swap(
            &mut self,
            chunk_name: &str,
            index_1: usize,
            index_2: usize,
        ) -> PyResult<()> {
            self._impl
                .chunk_items_swap(chunk_name, index_1, index_2)
                .map_err(map_chunk_item_error)?;
            Ok(())
        }

        pub fn chunk_items_reorder(
            &mut self,
            chunk_name: &str,
            new_order: Vec<usize>,
        ) -> PyResult<()> {
            self._impl
                .chunk_items_reorder(chunk_name, new_order.as_slice())
                .map_err(|err| match err {
                    TD0Error::InvalidInputWithMessage(msg) => PyErr::new::<PyValueError, _>(msg),
                    TD0Error::UnknownChunk => PyErr::new::<PyValueError, _>("Unknown chunk"),
                    TD0Error::InvalidChunkItem => {
                        PyErr::new::<PyValueError, _>("Invalid chunk item index")
                    }
                    _ => PyErr::new::<PyRuntimeError, _>(format!("{err}")),
                })?;
            Ok(())
        }

        pub fn finalize(&mut self) -> PyResult<()> {
            self._impl.finalize().map_err(|err| match err {
                TD0Error::InvalidTD0File(msg) => PyErr::new::<PyValueError, _>(msg),
                _ => PyErr::new::<PyRuntimeError, _>(format!("{err}")),
            })?;
            Ok(())
        }

        pub fn to_bytes(&self) -> PyResult<Vec<u8>> {
            Ok(self._impl.to_bytes())
        }

        pub fn get_chunk_item(
            &self,
            chunk_name: &str,
            item_index: usize,
        ) -> PyResult<TD0ChunkItem> {
            Ok(TD0ChunkItem {
                _impl: self
                    ._impl
                    .get_chunk_item_owned(chunk_name, item_index)
                    .map_err(map_chunk_item_error)?,
            })
        }

        pub fn list_chunks(&self) -> Vec<String> {
            self._impl.list_chunks()
        }

        pub fn manifest(&self) -> PyResult<TD0Manifest> {
            Ok(TD0Manifest(self._impl.manifest().map_err(|err| {
                PyErr::new::<PyRuntimeError, _>(format!("{err}"))
            })?))
        }
    }
}
