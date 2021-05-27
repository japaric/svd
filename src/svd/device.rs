use super::{BuildError, Cpu, Peripheral, RegisterProperties, SvdError};

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("Device must contain at least one peripheral")]
    EmptyDevice,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Device {
    // vendor

    // vendorID
    /// The string identifies the device or device series. Device names are required to be unique
    pub name: String,

    // series
    /// Define the version of the SVD file
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub version: Option<String>,

    /// Describe the main features of the device (for example CPU, clock frequency, peripheral overview)
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,

    // licenseText
    /// Describe the processor included in the device
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub cpu: Option<Cpu>,

    /// Define the number of data bits uniquely selected by each address
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub address_unit_bits: Option<u32>,

    /// Define the number of data bit-width of the maximum single data transfer supported by the bus infrastructure
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub width: Option<u32>,

    /// Default properties for all registers
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub default_register_properties: RegisterProperties,

    /// Group to define peripherals
    pub peripherals: Vec<Peripheral>,

    /// Specify the compliant CMSIS-SVD schema version
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub(crate) schema_version: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct DeviceBuilder {
    name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    cpu: Option<Cpu>,
    address_unit_bits: Option<u32>,
    width: Option<u32>,
    default_register_properties: RegisterProperties,
    peripherals: Option<Vec<Peripheral>>,
    schema_version: Option<String>,
}

impl From<Device> for DeviceBuilder {
    fn from(d: Device) -> Self {
        Self {
            name: Some(d.name),
            version: d.version,
            description: d.description,
            cpu: d.cpu,
            address_unit_bits: d.address_unit_bits,
            width: d.width,
            default_register_properties: d.default_register_properties,
            peripherals: Some(d.peripherals),
            schema_version: d.schema_version,
        }
    }
}

impl DeviceBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn version(mut self, value: Option<String>) -> Self {
        self.version = value;
        self
    }
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    pub fn cpu(mut self, value: Option<Cpu>) -> Self {
        self.cpu = value;
        self
    }
    pub fn address_unit_bits(mut self, value: Option<u32>) -> Self {
        self.address_unit_bits = value;
        self
    }
    pub fn width(mut self, value: Option<u32>) -> Self {
        self.width = value;
        self
    }
    pub fn default_register_properties(mut self, value: RegisterProperties) -> Self {
        self.default_register_properties = value;
        self
    }
    pub fn peripherals(mut self, value: Vec<Peripheral>) -> Self {
        self.peripherals = Some(value);
        self
    }
    pub fn schema_version(mut self, value: Option<String>) -> Self {
        self.schema_version = value;
        self
    }
    pub fn build(self) -> Result<Device, SvdError> {
        (Device {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            version: self.version,
            description: self.description,
            cpu: self.cpu,
            address_unit_bits: self.address_unit_bits,
            width: self.width,
            default_register_properties: self.default_register_properties,
            peripherals: self
                .peripherals
                .ok_or_else(|| BuildError::Uninitialized("peripherals".to_string()))?,
            schema_version: self.schema_version,
        })
        .validate()
    }
}

impl Device {
    pub fn builder() -> DeviceBuilder {
        DeviceBuilder::default()
    }
    fn validate(self) -> Result<Self, SvdError> {
        // TODO
        if self.peripherals.is_empty() {
            return Err(Error::EmptyDevice.into());
        }
        Ok(self)
    }
}
