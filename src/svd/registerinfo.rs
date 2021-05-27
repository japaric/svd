use super::{
    Access, BuildError, Field, ModifiedWriteValues, RegisterProperties, SvdError, WriteConstraint,
};

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "strict")]
    #[error("Register have `fields` tag, but it is empty")]
    EmptyFields,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RegisterInfo {
    /// String to identify the register.
    /// Register names are required to be unique within the scope of a peripheral
    pub name: String,

    /// Specifies a register name without the restritions of an ANSIS C identifier.
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub display_name: Option<String>,

    /// String describing the details of the register
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,

    /// Specifies a group name associated with all alternate register that have the same name
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub alternate_group: Option<String>,

    /// This tag can reference a register that has been defined above to
    /// current location in the description and that describes the memory location already
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub alternate_register: Option<String>,

    /// Define the address offset relative to the enclosing element
    pub address_offset: u32,

    /// Specifies register size, access permission and reset value
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub properties: RegisterProperties,

    /// Specifies the write side effects
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub modified_write_values: Option<ModifiedWriteValues>,

    /// Specifies the subset of allowed write values
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub write_constraint: Option<WriteConstraint>,

    /// `None` indicates that the `<fields>` node is not present
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub fields: Option<Vec<Field>>,

    /// Specify the register name from which to inherit data.
    /// Elements specified subsequently override inherited values
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub derived_from: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RegisterInfoBuilder {
    name: Option<String>,
    display_name: Option<String>,
    description: Option<String>,
    alternate_group: Option<String>,
    alternate_register: Option<String>,
    address_offset: Option<u32>,
    properties: RegisterProperties,
    modified_write_values: Option<ModifiedWriteValues>,
    write_constraint: Option<WriteConstraint>,
    fields: Option<Vec<Field>>,
    derived_from: Option<String>,
}

impl From<RegisterInfo> for RegisterInfoBuilder {
    fn from(r: RegisterInfo) -> Self {
        Self {
            name: Some(r.name),
            display_name: r.display_name,
            description: r.description,
            alternate_group: r.alternate_group,
            alternate_register: r.alternate_register,
            address_offset: Some(r.address_offset),
            properties: r.properties,
            modified_write_values: r.modified_write_values,
            write_constraint: r.write_constraint,
            fields: r.fields,
            derived_from: r.derived_from,
        }
    }
}

impl RegisterInfoBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn display_name(mut self, value: Option<String>) -> Self {
        self.display_name = value;
        self
    }
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    pub fn alternate_group(mut self, value: Option<String>) -> Self {
        self.alternate_group = value;
        self
    }
    pub fn alternate_register(mut self, value: Option<String>) -> Self {
        self.alternate_register = value;
        self
    }
    pub fn address_offset(mut self, value: u32) -> Self {
        self.address_offset = Some(value);
        self
    }
    pub fn properties(mut self, value: RegisterProperties) -> Self {
        self.properties = value;
        self
    }
    pub fn size(mut self, value: Option<u32>) -> Self {
        self.properties.size = value;
        self
    }
    pub fn access(mut self, value: Option<Access>) -> Self {
        self.properties.access = value;
        self
    }
    pub fn reset_value(mut self, value: Option<u64>) -> Self {
        self.properties.reset_value = value;
        self
    }
    pub fn reset_mask(mut self, value: Option<u64>) -> Self {
        self.properties.reset_mask = value;
        self
    }
    pub fn modified_write_values(mut self, value: Option<ModifiedWriteValues>) -> Self {
        self.modified_write_values = value;
        self
    }
    pub fn write_constraint(mut self, value: Option<WriteConstraint>) -> Self {
        self.write_constraint = value;
        self
    }
    pub fn fields(mut self, value: Option<Vec<Field>>) -> Self {
        self.fields = value;
        self
    }
    pub fn derived_from(mut self, value: Option<String>) -> Self {
        self.derived_from = value;
        self
    }
    pub fn build(self) -> Result<RegisterInfo, SvdError> {
        (RegisterInfo {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            display_name: self.display_name,
            description: self.description,
            alternate_group: self.alternate_group,
            alternate_register: self.alternate_register,
            address_offset: self
                .address_offset
                .ok_or_else(|| BuildError::Uninitialized("address_offset".to_string()))?,
            properties: self.properties,
            modified_write_values: self.modified_write_values,
            write_constraint: self.write_constraint,
            fields: self.fields,
            derived_from: self.derived_from,
        })
        .validate()
    }
}

impl RegisterInfo {
    pub fn builder() -> RegisterInfoBuilder {
        RegisterInfoBuilder::default()
    }
    #[allow(clippy::unnecessary_wraps)]
    fn validate(self) -> Result<Self, SvdError> {
        #[cfg(feature = "strict")]
        super::check_dimable_name(&self.name, "name")?;
        #[cfg(feature = "strict")]
        {
            if let Some(name) = self.alternate_group.as_ref() {
                super::check_name(name, "alternateGroup")?;
            }
            if let Some(name) = self.alternate_register.as_ref() {
                super::check_dimable_name(name, "alternateRegister")?;
            }
        }
        if let Some(_name) = self.derived_from.as_ref() {
            #[cfg(feature = "strict")]
            super::check_derived_name(_name, "derivedFrom")?;
        } else if let Some(fields) = self.fields.as_ref() {
            if fields.is_empty() {
                #[cfg(feature = "strict")]
                return Err(Error::EmptyFields)?;
            }
        }
        Ok(self)
    }
}
