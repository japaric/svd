use crate::elementext::ElementExt;
use xmltree::Element;

use crate::types::Parse;

use crate::encode::{Encode, EncodeChildren};
use crate::new_element;

use crate::error::*;
use crate::svd::{
    register::{RegIter, RegIterMut},
    registercluster::RegisterCluster,
    registerproperties::RegisterProperties,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ClusterInfo {
    /// String to identify the cluster.
    /// Cluster names are required to be unique within the scope of a peripheral
    pub name: String,

    /// String describing the details of the register cluster
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub description: Option<String>,

    // alternateCluster
    /// Specify the struct type name created in the device header file
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub header_struct_name: Option<String>,

    /// Cluster address relative to the `baseAddress` of the peripheral
    pub address_offset: u32,

    /// Default properties for all registers
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub default_register_properties: RegisterProperties,

    pub children: Vec<RegisterCluster>,

    /// Specify the cluster name from which to inherit data.
    /// Elements specified subsequently override inherited values
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub derived_from: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClusterInfoBuilder {
    name: Option<String>,
    description: Option<String>,
    header_struct_name: Option<String>,
    address_offset: Option<u32>,
    default_register_properties: RegisterProperties,
    children: Option<Vec<RegisterCluster>>,
    derived_from: Option<String>,
}

impl From<ClusterInfo> for ClusterInfoBuilder {
    fn from(c: ClusterInfo) -> Self {
        Self {
            name: Some(c.name),
            description: c.description,
            header_struct_name: c.header_struct_name,
            address_offset: Some(c.address_offset),
            default_register_properties: c.default_register_properties,
            children: Some(c.children),
            derived_from: c.derived_from,
        }
    }
}

impl ClusterInfoBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = value;
        self
    }
    pub fn header_struct_name(mut self, value: Option<String>) -> Self {
        self.header_struct_name = value;
        self
    }
    pub fn address_offset(mut self, value: u32) -> Self {
        self.address_offset = Some(value);
        self
    }
    pub fn default_register_properties(mut self, value: RegisterProperties) -> Self {
        self.default_register_properties = value;
        self
    }
    pub fn children(mut self, value: Vec<RegisterCluster>) -> Self {
        self.children = Some(value);
        self
    }
    pub fn derived_from(mut self, value: Option<String>) -> Self {
        self.derived_from = value;
        self
    }
    pub fn build(self) -> Result<ClusterInfo> {
        (ClusterInfo {
            name: self
                .name
                .ok_or_else(|| BuildError::Uninitialized("name".to_string()))?,
            description: self.description,
            header_struct_name: self.header_struct_name,
            address_offset: self
                .address_offset
                .ok_or_else(|| BuildError::Uninitialized("address_offset".to_string()))?,
            default_register_properties: self.default_register_properties,
            children: self
                .children
                .ok_or_else(|| BuildError::Uninitialized("children".to_string()))?,
            derived_from: self.derived_from,
        })
        .validate()
    }
}

impl ClusterInfo {
    #[allow(clippy::unnecessary_wraps)]
    fn validate(self) -> Result<Self> {
        #[cfg(feature = "strict")]
        check_dimable_name(&self.name, "name")?;
        if let Some(_name) = self.derived_from.as_ref() {
            #[cfg(feature = "strict")]
            check_derived_name(_name, "derivedFrom")?;
        } else if self.children.is_empty() {
            #[cfg(feature = "strict")]
            return Err(SVDError::EmptyCluster)?;
        }
        Ok(self)
    }
}

impl Parse for ClusterInfo {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        let name = tree.get_child_text("name")?;
        Self::_parse(tree, name.clone()).with_context(|| format!("In cluster `{}`", name))
    }
}

impl ClusterInfo {
    fn _parse(tree: &Element, name: String) -> Result<Self> {
        ClusterInfoBuilder::default()
            .name(name)
            .description(tree.get_child_text_opt("description")?)
            .header_struct_name(tree.get_child_text_opt("headerStructName")?)
            .address_offset(tree.get_child_u32("addressOffset")?)
            .default_register_properties(RegisterProperties::parse(tree)?)
            .children({
                let children: Result<Vec<_>, _> = tree
                    .children
                    .iter()
                    .filter(|t| t.name == "register" || t.name == "cluster")
                    .map(RegisterCluster::parse)
                    .collect();
                children?
            })
            .derived_from(tree.attributes.get("derivedFrom").map(|s| s.to_owned()))
            .build()
    }

    /// returns iterator over all registers cluster contains
    pub fn reg_iter(&self) -> RegIter {
        let mut rem: Vec<&RegisterCluster> = Vec::with_capacity(self.children.len());
        for r in self.children.iter().rev() {
            rem.push(r);
        }
        RegIter { rem }
    }

    /// returns mutable iterator over all registers cluster contains
    pub fn reg_iter_mut(&mut self) -> RegIterMut {
        let mut rem: Vec<&mut RegisterCluster> = Vec::with_capacity(self.children.len());
        for r in self.children.iter_mut().rev() {
            rem.push(r);
        }
        RegIterMut { rem }
    }
}

impl Encode for ClusterInfo {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let mut e = new_element("cluster", None);

        e.children.push(new_element("name", Some(e.name.clone())));

        if let Some(v) = &self.description {
            e.attributes
                .insert(String::from("description"), v.to_string());
        }

        if let Some(v) = &self.header_struct_name {
            e.children
                .push(new_element("headerStructName", Some(v.clone())));
        }

        e.children.push(new_element(
            "addressOffset",
            Some(format!("{}", self.address_offset)),
        ));

        e.children
            .extend(self.default_register_properties.encode()?);

        for c in &self.children {
            e.children.push(c.encode()?);
        }

        if let Some(v) = &self.derived_from {
            e.attributes
                .insert(String::from("derivedFrom"), v.to_string());
        }

        Ok(e)
    }
}

// TODO: test ClusterInfo encode and decode
