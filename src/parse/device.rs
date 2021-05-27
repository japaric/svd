use super::{elementext::ElementExt, optional, Context, Element, Parse, Result, SVDError};
use crate::svd::{
    cpu::Cpu, peripheral::Peripheral, registerproperties::RegisterProperties, Device,
};
use rayon::prelude::*;

/// Parses a SVD file
impl Parse for Device {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if tree.name != "device" {
            return Err(SVDError::NotExpectedTag(tree.clone(), "device".to_string()).into());
        }
        let name = tree.get_child_text("name")?;
        parse_device(tree, name.clone()).with_context(|| format!("In device `{}`", name))
    }
}

fn parse_device(tree: &Element, name: String) -> Result<Device> {
    Ok(Device::builder()
        .name(name)
        .version(tree.get_child_text_opt("version")?)
        .description(tree.get_child_text_opt("description")?)
        .cpu(optional::<Cpu>("cpu", tree)?)
        .address_unit_bits(optional::<u32>("addressUnitBits", tree)?)
        .width(optional::<u32>("width", tree)?)
        .default_register_properties(RegisterProperties::parse(tree)?)
        .peripherals({
            let ps: Result<Vec<_>, _> = tree
                .get_child_elem("peripherals")?
                .children
                .par_iter()
                .map(Peripheral::parse)
                .collect();
            ps?
        })
        .schema_version(tree.attributes.get("schemaVersion").cloned())
        .build()?)
}
