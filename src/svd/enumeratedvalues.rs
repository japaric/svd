use crate::elementext::ElementExt;
use crate::NS;
use minidom::Element;

use crate::encode::Encode;
use crate::error::*;

use crate::new_element;
use crate::parse;
use crate::svd::{enumeratedvalue::EnumeratedValue, usage::Usage};
use crate::types::Parse;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct EnumeratedValues {
    /// Identifier for the whole enumeration section
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,

    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub usage: Option<Usage>,

    /// Makes a copy from a previously defined enumeratedValues section.
    /// No modifications are allowed
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub derived_from: Option<String>,

    pub values: Vec<EnumeratedValue>,

    // Reserve the right to add more fields to this struct
    #[cfg_attr(feature = "serde", serde(skip))]
    _extensible: (),
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum EnumeratedValuesError {
    #[error("EnumeratedValues is empty")]
    Empty,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EnumeratedValuesBuilder {
    name: Option<String>,
    usage: Option<Usage>,
    derived_from: Option<String>,
    values: Option<Vec<EnumeratedValue>>,
}

impl EnumeratedValuesBuilder {
    pub fn name(mut self, value: Option<String>) -> Self {
        self.name = value;
        self
    }
    pub fn usage(mut self, value: Option<Usage>) -> Self {
        self.usage = value;
        self
    }
    pub fn derived_from(mut self, value: Option<String>) -> Self {
        self.derived_from = value;
        self
    }
    pub fn values(mut self, value: Vec<EnumeratedValue>) -> Self {
        self.values = Some(value);
        self
    }
    pub fn build(self) -> Result<EnumeratedValues> {
        (EnumeratedValues {
            name: self.name,
            usage: self.usage,
            derived_from: self.derived_from,
            values: self.values.unwrap_or_default(),
            _extensible: (),
        })
        .validate()
    }
}

impl EnumeratedValues {
    fn validate(self) -> Result<Self> {
        if let Some(name) = self.name.as_ref() {
            check_name(name, "name")?;
        }
        if let Some(dname) = self.derived_from.as_ref() {
            check_derived_name(dname, "derivedFrom")?;
            Ok(self)
        } else if self.values.is_empty() {
            Err(EnumeratedValuesError::Empty.into())
        } else {
            Ok(self)
        }
    }
    pub(crate) fn check_range(&self, range: core::ops::Range<u64>) -> Result<()> {
        for v in self.values.iter() {
            v.check_range(&range)?;
        }
        Ok(())
    }
}

impl Parse for EnumeratedValues {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        assert_eq!(tree.name(), "enumeratedValues");
        EnumeratedValuesBuilder::default()
            .name(tree.get_child_text_opt("name")?)
            .usage(parse::optional::<Usage>("usage", tree)?)
            .derived_from(tree.attr("derivedFrom").map(|s| s.to_owned()))
            .values({
                let values: Result<Vec<_>, _> = tree
                    .children()
                    .filter(|t| {
                        ["name", "headerEnumName", "usage"]
                            .iter()
                            .all(|s| &t.name() != s)
                    })
                    .enumerate()
                    .map(|(e, t)| {
                        if t.name() == "enumeratedValue" {
                            EnumeratedValue::parse(t)
                                .with_context(|| format!("Parsing enumerated value #{}", e))
                        } else {
                            Err(
                                SVDError::NotExpectedTag(t.clone(), "enumeratedValue".to_string())
                                    .into(),
                            )
                        }
                    })
                    .collect();
                values?
            })
            .build()
    }
}

impl Encode for EnumeratedValues {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let mut e = Element::builder("enumeratedValues", NS);

        if let Some(d) = &self.name {
            e = e.append(new_element("name", Some((*d).clone())));
        };

        if let Some(v) = &self.usage {
            e = e.append(v.encode()?);
        };

        if let Some(v) = &self.derived_from {
            e = e.attr("derivedFrom", v);
        }

        for v in &self.values {
            e = e.append(v.encode()?);
        }

        Ok(e.build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::svd::enumeratedvalue::EnumeratedValueBuilder;

    #[test]
    fn decode_encode() {
        let example =
            "
            <enumeratedValues xmlns=\"".to_string() + NS + "\" derivedFrom=\"fake_derivation\">
                <enumeratedValue>
                    <name>WS0</name>
                    <description>Zero wait-states inserted in fetch or read transfers</description>
                    <isDefault>true</isDefault>
                </enumeratedValue>
                <enumeratedValue>
                    <name>WS1</name>
                    <description>One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details</description>
                    <value>0x00000001</value>
                </enumeratedValue>
            </enumeratedValues>
        ";

        let expected = EnumeratedValuesBuilder::default()
            .derived_from(Some("fake_derivation".to_string()))
            .values(vec![
                EnumeratedValueBuilder::default()
                    .name("WS0".to_string())
                    .description(Some(
                        "Zero wait-states inserted in fetch or read transfers".to_string()
                    ))
                    .is_default(Some(true))
                    .build()
                    .unwrap(),
                EnumeratedValueBuilder::default()
                    .name("WS1".to_string())
                    .description(Some(
                        "One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details".to_string()
                    ))
                    .value(Some(1))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        // TODO: move to test! macro
        let tree1: Element = example.parse().unwrap();

        let parsed = EnumeratedValues::parse(&tree1).unwrap();
        assert_eq!(parsed, expected, "Parsing tree failed");

        let tree2 = parsed.encode().unwrap();
        assert_eq!(tree1, tree2, "Encoding value failed");
    }

    #[test]
    fn valid_children() {
        fn parse(contents: String) -> Result<EnumeratedValues> {
            let example = "<enumeratedValues xmlns=\"".to_string()
                + NS
                + "\">"
                + &contents
                + "</enumeratedValues>";
            let tree: Element = example.parse().unwrap();
            EnumeratedValues::parse(&tree)
        }

        // `enumeratedValue` occurrence: 1..*
        parse("".into()).expect_err("must contain at least one <enumeratedValue>");

        let value = String::from(
            "
            <enumeratedValue>
                <name>WS0</name>
                <description>Zero wait-states inserted in fetch or read transfers</description>
                <value>0x00000000</value>
            </enumeratedValue>",
        );

        // Valid tags
        parse(value.clone() + "<name>foo</name>").expect("<name> is valid");
        parse(value.clone() + "<headerEnumName>foo</headerEnumName>")
            .expect("<headerEnumName> is valid");
        parse(value.clone() + "<usage>read</usage>").expect("<usage> is valid");

        // Invalid tags
        parse(value.clone() + "<enumerateValue></enumerateValue>")
            .expect_err("<enumerateValue> in invalid here");
        parse(value.clone() + "<enumeratedValues></enumeratedValues>")
            .expect_err("<enumeratedValues> in invalid here");
    }
}
