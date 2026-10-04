use super::result::{OutOfRangeErrorTD0Decimal, TD0Error};
use super::{
    ChunkManifest, IntEncodedDecimal, TD0BackupType, TD0DeviceModel, TD0Manifest, TD0Value,
    VOLUME_MAX, VOLUME_MIN, VOLUME_MINUS_INF_DISPLAY, VOLUME_MINUS_INF_FLOAT, VOLUME_MINUS_INF_I16,
    Volume, checksum_bytes_to_string,
};
use core::ops::{Div, Mul};
use zerocopy::{ByteOrder, I16, U16};


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
    pub fn new(name: impl Into<String>, pos: usize, size: usize, num_items: usize, item_size: usize) -> Self {
        Self {
            name: name.into(),
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
    type Err = TD0Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let selfobj = Self(s.parse::<f32>().map_err(|_| {
            TD0Error::InvalidInput("input string not parsable as a Decimal.".to_string())
        })?);
        Ok(selfobj)
    }
}

impl<T: ByteOrder> TryFrom<IntEncodedDecimal> for I16<T> {
    type Error = <i16 as TryFrom<i32>>::Error;

    fn try_from(val: IntEncodedDecimal) -> Result<Self, Self::Error> {
        let i32val = val.0.mul(10.0f32).round() as i32;
        Ok(I16::from(i16::try_from(i32val)?))
    }
}

impl<T: ByteOrder> TryFrom<IntEncodedDecimal> for U16<T> {
    type Error = <u16 as TryFrom<i32>>::Error;

    fn try_from(val: IntEncodedDecimal) -> Result<Self, Self::Error> {
        let i32val = val.0.mul(10.0f32).round() as i32;
        Ok(U16::from(u16::try_from(i32val)?))
    }
}

impl From<IntEncodedDecimal> for f32 {
    fn from(val: IntEncodedDecimal) -> f32 {
        val.0
    }
}

impl TryFrom<IntEncodedDecimal> for i8 {
    type Error = <i8 as TryFrom<i32>>::Error;

    fn try_from(val: IntEncodedDecimal) -> Result<Self, Self::Error> {
        i8::try_from(val.0.mul(10.0f32).round() as i32)
    }
}

impl TryFrom<IntEncodedDecimal> for i16 {
    type Error = <i16 as TryFrom<i32>>::Error;

    fn try_from(val: IntEncodedDecimal) -> Result<Self, Self::Error> {
        i16::try_from(val.0.mul(10.0f32).round() as i32)
    }
}

impl From<IntEncodedDecimal> for i32 {
    fn from(val: IntEncodedDecimal) -> i32 {
        val.0.mul(10.0f32).round() as i32
    }
}

impl TryFrom<IntEncodedDecimal> for u8 {
    type Error = <u8 as TryFrom<i32>>::Error;

    fn try_from(val: IntEncodedDecimal) -> Result<Self, Self::Error> {
        u8::try_from(val.0.mul(10.0f32).round() as i32)
    }
}

impl TryFrom<IntEncodedDecimal> for u16 {
    type Error = <u16 as TryFrom<i32>>::Error;

    fn try_from(val: IntEncodedDecimal) -> Result<Self, Self::Error> {
        u16::try_from(val.0.mul(10.0f32).round() as i32)
    }
}

impl<T: ByteOrder> From<I16<T>> for IntEncodedDecimal {
    fn from(val: I16<T>) -> Self {
        Self(f32::from(val.get()).div(10.0f32))
    }
}

impl<T: ByteOrder> From<U16<T>> for IntEncodedDecimal {
    fn from(val: U16<T>) -> Self {
        Self(f32::from(val.get()).div(10.0f32))
    }
}

impl From<f32> for IntEncodedDecimal {
    fn from(val: f32) -> Self {
        Self(val)
    }
}

impl From<i8> for IntEncodedDecimal {
    fn from(val: i8) -> Self {
        Self(f32::from(val).div(10.0f32))
    }
}

impl From<i16> for IntEncodedDecimal {
    fn from(val: i16) -> Self {
        Self(f32::from(val).div(10.0f32))
    }
}

impl From<i32> for IntEncodedDecimal {
    fn from(val: i32) -> Self {
        Self((val as f32).div(10.0f32))
    }
}

impl From<u8> for IntEncodedDecimal {
    fn from(val: u8) -> Self {
        Self(f32::from(val).div(10.0f32))
    }
}

impl From<u16> for IntEncodedDecimal {
    fn from(val: u16) -> Self {
        Self(f32::from(val).div(10.0f32))
    }
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

impl TD0DeviceModel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SPDSXPro => "SSXP",
            Self::Unknown => "UNKN",
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

impl Volume {
    fn validate_impl_range(val: &f32) -> Result<(), TD0Error> {
        if (val.lt(&VOLUME_MIN) || val.gt(&VOLUME_MAX)) && val.ne(&VOLUME_MINUS_INF_FLOAT) {
            Err(TD0Error::OutOfRangeDecimal(OutOfRangeErrorTD0Decimal::new(
                VOLUME_MIN, VOLUME_MAX,
            )))
        } else {
            Ok(())
        }
    }

    pub fn validate(&self) -> Result<(), TD0Error> {
        Self::validate_impl_range(&self.0)
    }
}

impl core::str::FromStr for Volume {
    type Err = TD0Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case(VOLUME_MINUS_INF_DISPLAY) {
            Ok(Self(VOLUME_MINUS_INF_FLOAT))
        } else {
            let selfobj = Self(s.parse::<f32>().map_err(|_| {
                TD0Error::InvalidInput("input string not parsable as a Decimal.".to_string())
            })?);
            selfobj.validate()?;
            Ok(selfobj)
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

impl From<Volume> for f32 {
    fn from(value: Volume) -> Self {
        value.0
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

impl TryFrom<f32> for Volume {
    type Error = TD0Error;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        let selfobj = Self(value);
        selfobj.validate()?;
        Ok(selfobj)
    }
}

impl TryFrom<i16> for Volume {
    type Error = TD0Error;
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