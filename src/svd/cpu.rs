#[cfg(feature = "unproven")]
use std::collections::HashMap;

use xmltree::Element;

use crate::elementext::ElementExt;
#[cfg(feature = "unproven")]
use crate::encode::Encode;
use crate::error::*;
#[cfg(feature = "unproven")]
use crate::new_element;
use crate::svd::endian::Endian;
use crate::types::Parse;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Cpu {
    pub name: String,
    pub revision: String,
    pub endian: Endian,
    pub mpu_present: bool,
    pub fpu_present: bool,
    pub nvic_priority_bits: u32,
    pub has_vendor_systick: bool,
}

impl Parse for Cpu {
    type Object = Cpu;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<Cpu, SVDError> {
        if tree.name != "cpu" {
            return Err(SVDErrorKind::NameMismatch(tree.clone()).into());
        }

        Ok(Cpu {
            name: tree.get_child_text("name")?,
            revision: tree.get_child_text("revision")?,
            endian: Endian::parse(tree.get_child_elem("endian")?)?,
            mpu_present: tree.get_child_bool("mpuPresent")?,
            fpu_present: tree.get_child_bool("fpuPresent")?,
            nvic_priority_bits: tree.get_child_u32("nvicPrioBits")?,
            has_vendor_systick: tree.get_child_bool("vendorSystickConfig")?,
        })
    }
}

#[cfg(feature = "unproven")]
impl Encode for Cpu {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        Ok(Element {
            prefix: None,
            namespace: None,
            namespaces: None,
            name: String::from("cpu"),
            attributes: HashMap::new(),
            children: vec![
                new_element("name", Some(self.name.clone())),
                new_element("revision", Some(self.revision.clone())),
                self.endian.encode()?,
                new_element("mpuPresent", Some(format!("{}", self.mpu_present))),
                new_element("fpuPresent", Some(format!("{}", self.fpu_present))),
                new_element("nvicPrioBits", Some(format!("{}", self.nvic_priority_bits))),
                new_element(
                    "vendorSystickConfig",
                    Some(format!("{}", self.has_vendor_systick)),
                ),
            ],
            text: None,
        })
    }
}

impl Cpu {
    pub fn is_cortex_m(&self) -> bool {
        self.name.starts_with("CM")
    }
}

#[cfg(test)]
#[cfg(feature = "unproven")]
mod tests {
    use super::*;
    use crate::run_test;

    #[test]
    fn decode_encode() {
        let tests = vec![(
            Cpu {
                name: String::from("EFM32JG12B500F512GM48"),
                revision: String::from("5.1.1"),
                endian: Endian::Little,
                mpu_present: true,
                fpu_present: true,
                nvic_priority_bits: 8,
                has_vendor_systick: false,
            },
            "
                    <cpu>
                        <name>EFM32JG12B500F512GM48</name>  
                        <revision>5.1.1</revision>
                        <endian>little</endian>
                        <mpuPresent>true</mpuPresent>
                        <fpuPresent>true</fpuPresent>
                        <nvicPrioBits>8</nvicPrioBits>
                        <vendorSystickConfig>false</vendorSystickConfig>
                    </cpu>
                ",
        )];

        run_test::<Cpu>(&tests[..]);
    }
}
