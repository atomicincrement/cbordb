
pub mod encoder;
pub mod decoder;
pub mod error;
pub mod constants;

pub use error::{Error, Result};

#[derive(PartialEq)]
pub struct CBOR<T : AsRef<[u8]>>(pub T);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Kind {
    Integer,
    Bytes,
    String,
    Null,
    Undefined,
    Bool,
    Float,
    Simple,
    Array,
    Map,
    Tag,
    Time,
    BigNumber,
    Invalid,
    Break,
}
