use crate::elementext::ElementExt;
use crate::NS;
use minidom::Element;

use crate::encode::Encode;
use crate::error::*;

use crate::new_element;
use crate::types::Parse;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WriteConstraint {
    WriteAsRead(bool),
    UseEnumeratedValues(bool),
    Range(WriteConstraintRange),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WriteConstraintRange {
    pub min: u64,
    pub max: u64,
}

impl Parse for WriteConstraint {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if tree.children().count() == 1 {
            let field = &tree.children().next().unwrap().name();
            // Write constraint can only be one of the following
            match field.as_ref() {
                "writeAsRead" => Ok(WriteConstraint::WriteAsRead(
                    tree.get_child_bool(field.as_ref())?,
                )),
                "useEnumeratedValues" => Ok(WriteConstraint::UseEnumeratedValues(
                    tree.get_child_bool(field.as_ref())?,
                )),
                "range" => Ok(WriteConstraint::Range(WriteConstraintRange::parse(
                    tree.get_child_elem(field.as_ref())?,
                )?)),
                _ => Err(SVDError::UnknownWriteConstraint(tree.clone()).into()),
            }
        } else {
            Err(SVDError::MoreThanOneWriteConstraint(tree.clone()).into())
        }
    }
}

impl Encode for WriteConstraint {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let v = match *self {
            WriteConstraint::WriteAsRead(v) => {
                new_element("writeAsRead", Some(format!("{}", v))).build()
            }
            WriteConstraint::UseEnumeratedValues(v) => {
                new_element("useEnumeratedValues", Some(format!("{}", v))).build()
            }
            WriteConstraint::Range(v) => v.encode()?,
        };

        Ok(Element::builder("writeConstraint", NS).append(v).build())
    }
}

impl Parse for WriteConstraintRange {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        Ok(Self {
            min: tree.get_child_u64("minimum")?,
            max: tree.get_child_u64("maximum")?,
        })
    }
}

impl Encode for WriteConstraintRange {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        Ok(Element::builder("range", NS)
            .append(new_element("minimum", Some(format!("0x{:08.x}", self.min))))
            .append(new_element("maximum", Some(format!("0x{:08.x}", self.max))))
            .build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run_test;

    #[test]
    fn decode_encode() {
        let tests = vec![
            (
                WriteConstraint::WriteAsRead(true),
                "<writeConstraint><writeAsRead>true</writeAsRead></writeConstraint>"
            ),
            (
                WriteConstraint::UseEnumeratedValues(true),
                "<writeConstraint><useEnumeratedValues>true</useEnumeratedValues></writeConstraint>"
            ),
            (
                WriteConstraint::Range(WriteConstraintRange{min: 1, max: 10}),
                "<writeConstraint><range><minimum>0x00000001</minimum><maximum>0x0000000a</maximum></range></writeConstraint>"
            ),
        ];

        run_test::<WriteConstraint>(&tests[..]);
    }
}
