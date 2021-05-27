use super::{elementext::ElementExt, Context, Element, Parse, Result, SVDError};
use crate::svd::Interrupt;

fn parse_interrupt(tree: &Element, name: String) -> Result<Interrupt> {
    Ok(Interrupt {
        name,
        description: tree.get_child_text_opt("description")?,
        value: tree.get_child_u32("value")?,
    })
}

impl Parse for Interrupt {
    type Object = Self;
    type Error = anyhow::Error;

    fn parse(tree: &Element) -> Result<Self> {
        if tree.name != "interrupt" {
            return Err(SVDError::NotExpectedTag(tree.clone(), "interrupt".to_string()).into());
        }
        let name = tree.get_child_text("name")?;
        parse_interrupt(tree, name.clone()).with_context(|| format!("In interrupt `{}`", name))
    }
}
