
use std::collections::HashMap;

use xmltree::Element;


use elementext::ElementExt;
use helpers::{ParseElem, EncodeElem, new_element};
use parse;



#[derive(Clone, Debug, PartialEq)]
pub struct Interrupt {
    pub name: String,
    pub description: Option<String>,
    pub value: u32,
}

impl ParseElem for Interrupt {
    fn parse(tree: &Element) -> Interrupt {
        Interrupt {
            name: try_get_child!(tree.get_child_text("name")),
            description: tree.get_child_text("description"),
            value: try_get_child!(parse::u32(try_get_child!(tree.get_child("value")))),
        }
    }
}

impl EncodeElem for Interrupt {
    fn encode(&self) -> Element {
        Element {
            name: String::from("interrupt"),
            attributes: HashMap::new(),
            children: vec![
                new_element("name", Some(self.name.clone())),
                new_element("description", self.description.clone()),
                new_element("value", Some(format!("{}", self.value))),
            ],
            text: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_encode() {
        let types = vec![
            (
                Interrupt {
                    name: String::from("test"),
                    description: Some(String::from("description")),
                    value: 14,
                },
                String::from(
                    "
                <interrupt>
                    <name>test</name>
                    <description>description</description>
                    <value>14</value>
                </interrupt>",
                )
            ),
        ];

        for (a, s) in types {
            let tree1 = &try_get_child!(Element::parse(s.as_bytes()));
            let v = Interrupt::parse(tree1);
            assert_eq!(v, a, "Parsing `{}` expected `{:?}`", s, a);
            let tree2 = &v.encode();
            assert_eq!(tree1, tree2, "Encoding {:?} expected {}", a, s);
        }
    }
}
