use std::fmt;
use failure::{Error, Fail};

#[derive(Debug, Fail, PartialEq, Eq)]
pub enum TagError {
    #[fail(display = "Expected {} in `<{}>` element, found none",content, name)]
    EmptyTag {
        name: String,
        content: XmlContent, 
    },
    #[fail(display = "Expected a `<{}>` tag but found none", name)]
    MissingTag {
        name: String,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum XmlContent {
    Text,
    Element,
    Unknown,
}

impl fmt::Display for XmlContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &XmlContent::Text => write!(f, "text content"),
            &XmlContent::Element => write!(f, "element contents"),
            &XmlContent::Unknown => write!(f, "contents"),
        }
    }
}

#[derive(Debug, Fail, PartialEq, Eq)]
pub enum PeripheralError {
    #[fail(display = "Peripheral #{} has no name", _0)]
    UnnamedPeripheral(usize, #[cause] TagError),
    #[fail(display = "In peripheral \"{}\"", _0)]
    NamedPeripheral(String),
}

impl PeripheralError {
    pub fn from_cause(f: Error, i: usize) -> Error {
        let res = f.downcast::<Named>();
        if let Ok(regname) = res {
            let name = regname.0.clone();
            return regname.1.context(PeripheralError::NamedPeripheral(name)).into()
        }
        let res = res.unwrap_err().downcast::<TagError>();
        if let Ok(tagerror) = res {
            return PeripheralError::UnnamedPeripheral(i,tagerror).into()
        }
        println!("\"{}\"", res.unwrap_err());
        unimplemented!()
    }
}

#[derive(Debug, Fail)]
pub enum RegisterClusterError {
    #[fail(display = "Cluster #{} has no name", _0)]
    UnnamedCluster(usize, #[cause] TagError),
    #[fail(display = "In cluster \"{}\"", _0)]
    NamedCluster(String),
    #[fail(display = "Register #{} has no name", _0)]
    UnnamedRegister(usize, #[cause] TagError),
    #[fail(display = "In register \"{}\"", _0)]
    NamedRegister(String),
}

impl RegisterClusterError {
    pub fn from_cause(f: Error, i: usize) -> Error {
        let res = f.downcast::<Named>();
        if let Ok(regname) = res {
            let name = regname.0.clone();
            return regname.1.context(RegisterClusterError::NamedRegister(name)).into()
        }
        let res = res.unwrap_err().downcast::<TagError>();
        if let Ok(tagerror) = res {
            return RegisterClusterError::UnnamedRegister(i,tagerror).into()
        }
        let res = res.unwrap_err().downcast::<ClusterError>();
        if let Ok(clustererror) = res {
            let res = clustererror.1.downcast::<Named>();
            if let Ok(clusname) = res {
                let name = clusname.0.clone();
                return clusname.1.context(RegisterClusterError::NamedCluster(name)).into()
            }
            let res = res.unwrap_err().downcast::<TagError>();
            if let Ok(tagerror) = res {
                return RegisterClusterError::UnnamedCluster(i,tagerror).into()
            }
            //return RegisterClusterError::UnnamedCluster(i).context(format_err!("e")).into()
        println!("\"{:?}\"", res.unwrap_err());
            unimplemented!("Unknown error on cluster")
        }
        let res = res.unwrap_err().downcast::<RegisterClusterArrayParseError>();
        if let Ok(dimerror) = res {
            return dimerror.1.context(RegisterClusterError::NamedRegister(dimerror.0)).into()
            //return RegisterClusterError::NamedRegister(dimerror.0).context(dimerror.1).into()
        }
        print_causes(res.unwrap_err().cause());
        unimplemented!("Unknown error/Unhandled error on RegisterClusterError")
    }
}

#[derive(Debug, Fail)]
#[fail(display = "ClusterError")]
pub(crate) struct ClusterError(pub usize, pub Error);

#[derive(Debug, Fail)]
#[fail(display = "RegisterClusterArrayParseError")]
pub struct RegisterClusterArrayParseError(pub String, pub Error);

impl RegisterClusterArrayParseError {
    pub fn no_placeholder(name: String) -> Error {
        RegisterClusterArrayParseError(name, DimensionError::NoPlaceHolder.into()).into()
    }
    pub fn dimensions_mismatch(name: String) -> Error {
        RegisterClusterArrayParseError(name, DimensionError::DimensionsMismatch.into()).into()
    }
}

#[derive(Debug,Fail)]
pub enum DimensionError {
    //#[fail(display = "An error occured while parsing `<dim>`")]
    //ParseError(#[cause] ::std::num::ParseIntError),
    #[fail(display = "<name> was expected to contain a [%s] placeholder, found none")]
    NoPlaceHolder,
    #[fail(display = "Dimensions of register/cluster doesn't match the specified dimensions")]
    DimensionsMismatch,
}
#[derive(Debug, Fail)]
#[fail(display = "")]
/// Internal, only to capture name
pub(crate) struct Named(pub String, pub Error);

// TODO: Put all *Error that relates to inner levels of device (and device) into one enum

#[derive(Debug, Fail)]
pub enum FieldError {
    #[fail(display = "Field #{} has no name", _0)]
    UnnamedField(usize, #[cause] TagError),
    #[fail(display = "In field \"{}\"", _1)]
    NamedField(usize,String),
}

impl FieldError {
    pub fn from_cause(f: Error, i: usize) -> Error {
        let res = f.downcast::<Named>();
        if let Ok(regname) = res {
            let name = regname.0.clone();
            return regname.1.context(FieldError::NamedField(i,name)).into()
        }
        let res = res.unwrap_err().downcast::<TagError>();
        if let Ok(tagerror) = res {
            return FieldError::UnnamedField(i,tagerror).into()
        }
        println!("\"{}\"", res.unwrap_err());
        unimplemented!()
    }
}

#[derive(Debug, Fail)]
pub enum BitRangeError {
    #[fail(display = "While parsing `<bitRange>`")]
    BitRange,
    // No specific error needed since the only possible error is a parsing error
    #[fail(display = "While parsing `<msb>` and `<lsb>`")]
    MsbLsb,
    #[fail(display = "While parsing `<bitOffset>` and `<bitWidth>`")]
    BitOffsetWidth,
}

impl BitRangeError {
    pub fn from_cause(f: Error) -> Error {
        let res = f.downcast::<BitRangeParseError>();
        if let Ok(err) = res {
            return err.context(BitRangeError::BitRange).into()
            //return regname.1.context(RegisterError::NamedRegister(i,name)).into()
        }
        let res = res.unwrap_err().downcast::<BitRangeError>();
        if let Ok(err) = res {
            match err {
                _ => unimplemented!("BitRangeError::from_cause self match")
            }
            //return regname.1.context(RegisterError::NamedRegister(i,name)).into()
        }
        //let res = f.unwrap_err().downcast::<TagError>();
        //if let Ok(tagerror) = res {
        //}
        let res = res.unwrap_err().downcast::<::std::num::ParseIntError>();
        if let Ok(err) = res {
            match err {
                _ => unimplemented!("BitRangeError::from_cause unexpected error type")
            }
            //return regname.1.context(RegisterError::NamedRegister(i,name)).into()
        }
        res.unwrap_err()
    }
}

#[derive(Debug, Fail)]
pub enum EnumeratedValueError {
    // FIXME: error message is misleading, this displayed value is not necessarily the same as
    // the one in the svd. It also doesn't dismiss non enumeratedValue tags 
    #[fail(display = "Enumerated value at place #{} has no name", _0)]
    UnnamedEnumeratedValue(usize, #[cause] TagError),
    #[fail(display = "In enumerated value \"{}\"", _1)]
    NamedEnumeratedValue(usize,String),
}

impl EnumeratedValueError {
    pub fn from_cause(f: Error, i: usize) -> Error {
        let res = f.downcast::<Named>();
        if let Ok(regname) = res {
            let name = regname.0.clone();
            return regname.1.context(EnumeratedValueError::NamedEnumeratedValue(i,name)).into()
        }
        let res = res.unwrap_err().downcast::<TagError>();
        if let Ok(tagerror) = res {
            return EnumeratedValueError::UnnamedEnumeratedValue(i,tagerror).into()
        }
        println!("\"{}\"", res.unwrap_err());
        unimplemented!()
    }
}
// TODO: Unite variant errors
#[derive(Debug, Fail)]
#[fail(display = "Unknown <access> variant: {}", _0)]
pub struct AccessVariantError(pub String);

#[derive(Debug, Fail)]
#[fail(display = "Unknown <endian> variant: {}", _0)]
pub struct EndianVariantError(pub String);

#[derive(Debug, Fail)]
#[fail(display = "Unknown <usage> variant: {}", _0)]
pub struct UsageVariantError(pub String);

#[derive(Debug, Fail)]
pub enum WriteConstraintError {
    #[fail(display = "Unknown <writeConstrain> variant: {}", _0)]
    Variant(String),
    #[fail(display = "found more than one <WriteConstraint> element")]
    TooManyElements,
}

#[derive(Debug,Fail)]
pub enum BitRangeParseError {
    #[fail(display = "Missing [")]
    MissingOpen,
    #[fail(display = "Missing ]")]
    MissingClose,
    #[fail(display = "An error occured while parsing")]
    ParseError(#[cause] ::std::num::ParseIntError),
    #[fail(display = "Invalid Syntax")] // FIXME: proper msg
    Syntax,
}

#[derive(Debug,Fail)]
#[fail(display = "While parsing `<{}>` ({}) as {}", tagname, text, conv)]
pub struct ParseError {
    pub tagname: String,
    pub conv: ConvType,
    // TODO: Is this worth it?
    pub text: String,
}

#[derive(Debug)]
pub enum ConvType {
    Bool,
    U32,
    DimIndex,
}

impl fmt::Display for ConvType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ConvType::Bool => write!(f, "bool"),
            &ConvType::U32 => write!(f, "u32"),
            &ConvType::DimIndex => write!(f, "dim indices"),
        }
    }
}

/// Used for debugging errors
fn print_causes(mut fail: &Fail) {
    println!("{}", &fail);
    while let Some(cause) = fail.cause() {
        fail = cause;
        println!("{}", &fail);
    }
}
