
use std::collections::HashMap;

use xmltree::Element;

use ::parse;
use ::types::{Parse, Encode, new_element};
use ::error::SVDError;
use ::svd::endian::Endian;


#[derive(Clone, Debug, PartialEq)]
pub struct Cpu {
    pub name: String,
    pub revision: String,
    pub endian: Endian,
    pub mpu_present: bool,
    pub fpu_present: bool,
    pub nvic_priority_bits: u32,
    pub has_vendor_systick: bool,

    // Reserve the right to add more fields to this struct
    _extensible: (),
}

impl Parse for Cpu {
    type Object = Cpu;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<Cpu, SVDError> {
        if tree.name != "cpu" {
            return Err(SVDError::NameMismatch(tree.clone()));
        }

        Ok(Cpu {
            name: parse::get_child_string("name", tree)?,
            revision: parse::get_child_string("revision", tree)?,
            endian: Endian::parse(parse::get_child_elem("endian", tree)?)?,
            mpu_present: parse::get_child_bool("mpuPresent", tree)?,
            fpu_present: parse::get_child_bool("fpuPresent", tree)?,
            nvic_priority_bits: parse::get_child_u32("nvicPrioBits", tree)?,
            has_vendor_systick: parse::get_child_bool("vendorSystickConfig", tree)?,
            _extensible: (),
        })
    }
}

impl Encode for Cpu {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        Ok(Element {
            name: String::from("cpu"),
            attributes: HashMap::new(),
            children: vec![
                new_element("name", Some(self.name.clone())),
                new_element("revision", Some(self.revision.clone())),
                self.endian.encode()?,
                new_element("mpuPresent", Some(format!("{}", self.mpu_present))),
                new_element("fpuPresent", Some(format!("{}", self.fpu_present))),
                new_element("nvicPrioBits", Some(format!("{}", self.nvic_priority_bits))),
                new_element("vendorSystickConfig",Some(format!("{}", self.has_vendor_systick))),
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
mod tests {
    use super::*;

    #[test]
    fn decode_encode() {
        let types = vec![
            (
                Cpu {
                    name: String::from("EFM32JG12B500F512GM48"),
                    revision: String::from("5.1.1"),
                    endian: Endian::Little,
                    mpu_present: true,
                    fpu_present: true,
                    nvic_priority_bits: 8,
                    has_vendor_systick: false,
                    _extensible: (),
                },
                String::from("
                        <cpu>
                            <name>EFM32JG12B500F512GM48</name>  
                            <revision>5.1.1</revision>
                            <endian>little</endian>
                            <mpuPresent>true</mpuPresent>
                            <fpuPresent>true</fpuPresent>
                            <nvicPrioBits>8</nvicPrioBits>
                            <vendorSystickConfig>false</vendorSystickConfig>
                        </cpu>
                ")
            ),
        ];

        for (a, s) in types {
            let tree1 = Element::parse(s.as_bytes()).unwrap();
            let value = Cpu::parse(&tree1).unwrap();
            assert_eq!(value, a, "Parsing `{}` expected `{:?}`", s, a);
            let tree2 = value.encode().unwrap();
            assert_eq!(tree1, tree2, "Encoding {:?} expected {}", a, s);
        }
    }
}