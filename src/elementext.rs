//! SVD Element Extensions.
//! This module is extends minidom::Element objects with convenience methods

use minidom::Element;

use crate::types::{BoolParse, Parse};

use crate::error::*;

/// Defines extensions for implementation over minidom::Element
pub trait ElementExt {
    fn get_child_text_opt<K>(&self, k: K) -> Result<Option<String>>
    where
        String: PartialEq<K>,
        K: AsRef<str>;
    fn get_child_text<K>(&self, k: K) -> Result<String>
    where
        String: PartialEq<K>,
        K: core::fmt::Display + Clone + AsRef<str>;

    fn get_text(&self) -> Result<String>;

    fn get_child_elem<'a>(&'a self, n: &str) -> Result<&'a Element>;
    fn get_child_u32(&self, n: &str) -> Result<u32>;
    fn get_child_u64(&self, n: &str) -> Result<u64>;
    fn get_child_bool(&self, n: &str) -> Result<bool>;

    fn merge(&mut self, n: &Self);

    fn debug(&self);
}

/// Implements extensions for minidom::Element
impl ElementExt for Element {
    fn get_child_text_opt<K>(&self, k: K) -> Result<Option<String>>
    where
        String: PartialEq<K>,
        K: AsRef<str>,
    {
        if let Some(child) = self.get_child(k, "") {
            match child.get_text() {
                Err(e) => match e.downcast_ref() {
                    // if tag is empty just ignore it
                    Some(SVDError::EmptyTag(_, _)) => Ok(None),
                    _ => return Err(e),
                },
                Ok(s) => Ok(Some(s.to_owned())),
            }
        } else {
            Ok(None)
        }
    }
    fn get_child_text<K>(&self, k: K) -> Result<String>
    where
        String: PartialEq<K>,
        K: core::fmt::Display + Clone + AsRef<str>,
    {
        self.get_child_text_opt(k.clone())?
            .ok_or_else(|| SVDError::MissingTag(self.clone(), format!("{}", k)).into())
    }

    /// Get text contained by an XML Element
    fn get_text(&self) -> Result<String> {
        match self.texts().next() {
            Some(s) => Ok(s.to_string()),
            // FIXME: Doesn't look good because SVDError doesn't format by itself. We already
            // capture the element and this information can be used for getting the name
            // This would fix ParseError
            None => Err(SVDError::EmptyTag(self.clone(), self.name().to_string()).into()),
        }
    }

    /// Get a named child element from an XML Element
    fn get_child_elem<'a>(&'a self, n: &str) -> Result<&'a Element> {
        match self.get_child(n, "") {
            Some(s) => Ok(s),
            None => Err(SVDError::MissingTag(self.clone(), n.to_string()).into()),
        }
    }

    /// Get a u32 value from a named child element
    fn get_child_u32(&self, n: &str) -> Result<u32> {
        let s = self.get_child_elem(n)?;
        u32::parse(&s).context(SVDError::ParseError(self.clone()))
    }

    /// Get a u64 value from a named child element
    fn get_child_u64(&self, n: &str) -> Result<u64> {
        let s = self.get_child_elem(n)?;
        u64::parse(&s).context(SVDError::ParseError(self.clone()))
    }

    /// Get a bool value from a named child element
    fn get_child_bool(&self, n: &str) -> Result<bool> {
        let s = self.get_child_elem(n)?;
        BoolParse::parse(s)
    }

    // Merges the children of two elements, maintaining the name and description of the first
    fn merge(&mut self, r: &Self) {
        for c in r.children() {
            self.append_child(c.clone());
        }
    }

    fn debug(&self) {
        let name = self.name();
        println!("<{}>", name);
        for c in self.children() {
            println!("{}: {:?}", c.name(), c.text())
        }
        println!("</{}>", name);
    }
}
