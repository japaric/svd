use std::collections::HashMap;

use crate::elementext::ElementExt;
use xmltree::Element;

use crate::types::Parse;

use crate::encode::Encode;
use crate::error::*;
use crate::new_element;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct AddressBlock {
    pub offset: u32,
    pub size: u32,
    pub usage: String,
}

impl Parse for AddressBlock {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        Ok(Self {
            offset: tree.get_child_u32("offset")?,
            size: tree.get_child_u32("size")?,
            usage: tree.get_child_text("usage")?,
        })
    }
}

impl Encode for AddressBlock {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        Ok(Element {
            prefix: None,
            namespace: None,
            namespaces: None,
            name: String::from("addressBlock"),
            attributes: HashMap::new(),
            children: vec![
                new_element("offset", Some(format!("0x{:X}", self.offset))),
                new_element("size", Some(format!("0x{:X}", self.size))),
                new_element("usage", Some(self.usage.clone())),
            ],
            text: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run_test;

    #[test]
    fn decode_encode() {
        let tests = vec![(
            AddressBlock {
                offset: 0,
                size: 0x00000800,
                usage: String::from("registers"),
            },
            "<addressBlock>
                    <offset>0x0</offset>
                    <size>0x800</size>
                    <usage>registers</usage>
                </addressBlock>",
        )];

        run_test::<AddressBlock>(&tests[..]);
    }
}
