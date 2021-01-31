use crate::NS;
use minidom::Element;

use crate::error::*;
use crate::new_element;
use crate::types::Parse;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BitRange {
    /// Value defining the position of the least significant bit of the field within the register
    pub offset: u32,

    /// Value defining the bit-width of the bitfield within the register
    pub width: u32,

    pub range_type: BitRangeType,
}

impl BitRange {
    pub fn lsb(&self) -> u32 {
        self.offset
    }
    pub fn msb(&self) -> u32 {
        self.offset + self.width - 1
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BitRangeType {
    BitRange,
    OffsetWidth,
    MsbLsb,
}

impl Parse for BitRange {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        let (end, start, range_type): (u32, u32, BitRangeType) = if let Some(range) =
            tree.get_child("bitRange", NS)
        {
            let text = range.text();
            //let text = range
            //    .text
            //    .as_ref()
            //    .ok_or_else(|| SVDError::InvalidBitRange(tree.clone(), InvalidBitRange::Empty))?;
            if !text.starts_with('[') {
                return Err(
                    SVDError::InvalidBitRange(tree.clone(), InvalidBitRange::Syntax).into(),
                );
                // TODO: Maybe have a MissingOpen/MissingClosing variant
            }
            if !text.ends_with(']') {
                return Err(
                    SVDError::InvalidBitRange(tree.clone(), InvalidBitRange::Syntax).into(),
                );
                // TODO: Maybe have a MissingOpen/MissingClosing variant
            }

            let mut parts = text[1..text.len() - 1].split(':');
            (
                parts
                    .next()
                    .ok_or_else(|| {
                        SVDError::InvalidBitRange(tree.clone(), InvalidBitRange::Syntax)
                    })?
                    .parse::<u32>()
                    .with_context(|| {
                        SVDError::InvalidBitRange(tree.clone(), InvalidBitRange::ParseError)
                    })?,
                parts
                    .next()
                    .ok_or_else(|| {
                        SVDError::InvalidBitRange(tree.clone(), InvalidBitRange::Syntax)
                    })?
                    .parse::<u32>()
                    .with_context(|| {
                        SVDError::InvalidBitRange(tree.clone(), InvalidBitRange::ParseError)
                    })?,
                BitRangeType::BitRange,
            )
        // TODO: Consider matching instead so we can say which of these tags are missing
        } else if let (Some(lsb), Some(msb)) =
            (tree.get_child("lsb", NS), tree.get_child("msb", NS))
        {
            (
                // TODO: `u32::parse` should not hide it's errors
                u32::parse(msb).with_context(|| {
                    SVDError::InvalidBitRange(tree.clone(), InvalidBitRange::MsbLsb)
                })?,
                u32::parse(lsb).with_context(|| {
                    SVDError::InvalidBitRange(tree.clone(), InvalidBitRange::MsbLsb)
                })?,
                BitRangeType::MsbLsb,
            )
        } else if let (Some(offset), Some(width)) = (
            tree.get_child("bitOffset", NS),
            tree.get_child("bitWidth", NS),
        ) {
            // Special case because offset and width are directly provided
            // (ie. do not need to be calculated as in the final step)
            return Ok(BitRange {
                // TODO: capture that error comes from offset/width tag
                // TODO: `u32::parse` should not hide it's errors
                offset: u32::parse(offset).with_context(|| {
                    SVDError::InvalidBitRange(tree.clone(), InvalidBitRange::ParseError)
                })?,
                width: u32::parse(width).with_context(|| {
                    SVDError::InvalidBitRange(tree.clone(), InvalidBitRange::ParseError)
                })?,
                range_type: BitRangeType::OffsetWidth,
            });
        } else {
            return Err(SVDError::InvalidBitRange(tree.clone(), InvalidBitRange::Syntax).into());
        };

        Ok(Self {
            offset: start,
            width: end - start + 1,
            range_type,
        })
    }
}

impl BitRange {
    // TODO: Encode method differs from Encode trait as it acts on a set of possible children, create an interface or decide how to better do this
    pub fn encode(&self) -> Result<Vec<Element>> {
        match self.range_type {
            BitRangeType::BitRange => Ok(vec![new_element(
                "bitRange",
                Some(format!("[{}:{}]", self.msb(), self.lsb())),
            )
            .build()]),
            BitRangeType::MsbLsb => Ok(vec![
                new_element("lsb", Some(format!("{}", self.lsb()))).build(),
                new_element("msb", Some(format!("{}", self.msb()))).build(),
            ]),
            BitRangeType::OffsetWidth => Ok(vec![
                new_element("bitOffset", Some(format!("{}", self.offset))).build(),
                new_element("bitWidth", Some(format!("{}", self.width))).build(),
            ]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NS;

    #[test]
    fn decode_encode() {
        let types = vec![
            (
                BitRange {
                    offset: 16,
                    width: 4,
                    range_type: BitRangeType::BitRange,
                },
                "
                <fake xmlns=\""
                    .to_string()
                    + NS
                    + "\"><bitRange>[19:16]</bitRange></fake>
            ",
            ),
            (
                BitRange {
                    offset: 16,
                    width: 4,
                    range_type: BitRangeType::OffsetWidth,
                },
                "
                <fake xmlns=\""
                    .to_string()
                    + NS
                    + "\"><bitOffset>16</bitOffset><bitWidth>4</bitWidth></fake>
            ",
            ),
            (
                BitRange {
                    offset: 16,
                    width: 4,
                    range_type: BitRangeType::MsbLsb,
                },
                "
                <fake xmlns=\""
                    .to_string()
                    + NS
                    + "\"><lsb>16</lsb><msb>19</msb></fake>
            ",
            ),
        ];

        for (a, s) in types {
            let tree1: Element = s.parse().unwrap();
            let value = BitRange::parse(&tree1).unwrap();
            assert_eq!(value, a, "Parsing `{}` expected `{:?}`", s, a);
            let mut tree2 = Element::builder("fake", NS)
                .append_all(value.encode().unwrap())
                .build();
            assert_eq!(tree1, tree2, "Encoding {:?} expected {}", a, s);
        }
    }
}
