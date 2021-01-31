use crate::elementext::ElementExt;
use crate::new_element;

use minidom::Element;

use crate::types::Parse;

use crate::encode::Encode;
use crate::error::*;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ModifiedWriteValues {
    OneToClear,
    OneToSet,
    OneToToggle,
    ZeroToClear,
    ZeroToSet,
    ZeroToToggle,
    Clear,
    Set,
    Modify,
}

impl Parse for ModifiedWriteValues {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        use self::ModifiedWriteValues::*;
        let text = tree.get_text()?;

        Ok(match text.as_ref() {
            "oneToClear" => OneToClear,
            "oneToSet" => OneToSet,
            "oneToToggle" => OneToToggle,
            "zeroToClear" => ZeroToClear,
            "zeroToSet" => ZeroToSet,
            "zeroToToggle" => ZeroToToggle,
            "clear" => Clear,
            "set" => Set,
            "modify" => Modify,
            s => return Err(SVDError::InvalidModifiedWriteValues(tree.clone(), s.into()).into()),
        })
    }
}

impl Encode for ModifiedWriteValues {
    type Error = anyhow::Error;

    fn encode(&self) -> Result<Element> {
        use self::ModifiedWriteValues::*;
        let v = match *self {
            OneToClear => "oneToClear",
            OneToSet => "oneToSet",
            OneToToggle => "oneToToggle",
            ZeroToClear => "zeroToClear",
            ZeroToSet => "zeroToSet",
            ZeroToToggle => "zeroToToggle",
            Clear => "clear",
            Set => "set",
            Modify => "modify",
        };

        Ok(new_element("modifiedWriteValues", Some(v.into())).build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run_test;

    #[test]
    fn decode_encode() {
        // FIXME: Do we need a more extensive test?
        let tests = vec![(
            ModifiedWriteValues::OneToToggle,
            "<modifiedWriteValues>oneToToggle</modifiedWriteValues>",
        )];

        run_test::<ModifiedWriteValues>(&tests[..]);
    }
}
