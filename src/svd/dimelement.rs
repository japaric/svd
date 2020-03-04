use xmltree::Element;

use crate::types::{parse_optional, DimIndex, Parse};

use crate::elementext::ElementExt;
#[cfg(feature = "unproven")]
use crate::encode::Encode;
#[cfg(feature = "unproven")]
use crate::new_element;

use crate::error::*;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct DimElement {
    /// Defines the number of elements in an array or list
    pub dim: u32,

    /// Specify the address increment between two neighboring array or list members in the address map
    pub dim_increment: u32,

    /// Specify the strings that substitue the placeholder `%s` within `name` and `displayName`.
    /// By default, <dimIndex> is a value starting with 0
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub dim_index: Option<Vec<String>>,

    // Reserve the right to add more fields to this struct
    #[cfg_attr(feature = "serde", serde(skip))]
    _extensible: (),
}

impl DimElement {
    /// Produce vector of names of elements (fields/registers/clusters) in array
    pub fn array_item_names(&self, name: &str) -> Vec<String> {
        if let Some(dim_index) = &self.dim_index {
            dim_index.iter().map(|i| name.replace("%s", &i)).collect()
        } else {
            (0..self.dim)
                .map(|i| name.replace("%s", &format!("{}", i)))
                .collect()
        }
    }
}

impl Parse for DimElement {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        Ok(Self {
            dim: tree.get_child_u32("dim")?,
            dim_increment: tree.get_child_u32("dimIncrement")?,
            dim_index: parse_optional::<DimIndex>("dimIndex", tree)?,
            _extensible: (),
        })
    }
}

#[cfg(feature = "unproven")]
impl Encode for DimElement {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        let mut e = new_element("dimElement", None);

        e.children
            .push(new_element("dim", Some(format!("{}", self.dim))));
        e.children.push(new_element(
            "dimIncrement",
            Some(format!("{}", self.dim_increment)),
        ));

        if let Some(di) = &self.dim_index {
            e.children.push(new_element("dimIndex", Some(di.join(","))));
        }

        Ok(e)
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
            DimElement {
                dim: 100,
                dim_increment: 4,
                dim_index: Some(vec!["10".to_owned(), "20".to_owned()]),
                _extensible: (),
            },
            "<dimElement>
                <dim>100</dim>
                <dimIncrement>4</dimIncrement>
                <dimIndex>10,20</dimIndex>
            </dimElement>
            ",
        )];

        run_test::<DimElement>(&tests[..]);
    }
}
