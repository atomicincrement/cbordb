
use std::convert::{TryFrom, TryInto};

pub enum Error {
    IncorrectType,
    TooShort,
    InvalidCode,
    Infinite,
    NonUFT8String,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub struct CBOR<T : AsRef<[u8]>>(T);

pub mod constants {
    // https://www.rfc-editor.org/rfc/rfc8949.html
    pub const MAJOR_SHIFT : usize = 5;

    pub const MAJOR_POSITIVE : u8 = 0;
    pub const MAJOR_NEGATIVE : u8 = 1;
    pub const MAJOR_BYTES : u8 = 2;
    pub const MAJOR_STRING : u8 = 3;
    pub const MAJOR_ARRAY : u8 = 4;
    pub const MAJOR_MAP : u8 = 5;
    pub const MAJOR_TAG : u8 = 6;
    pub const MAJOR_SIMPLE : u8 = 7;
}

use constants::*;

pub trait Visitor {
    fn integer(&self, val: u64, is_negative: bool) -> Result<()> { Ok(()) }
    fn bytes(&self, val: &[u8]) -> Result<()> { Ok(()) }
    fn string(&self, val: &str) -> Result<()> { Ok(()) }
    fn null(&self) -> Result<()> { Ok(()) }
    fn undefined(&self) -> Result<()> { Ok(()) }
    fn bool(&self, val: bool) -> Result<()> { Ok(()) }
    fn float(&self, val: f64) -> Result<()> { Ok(()) }
    fn simple(&self, val: u64) -> Result<()> { Ok(()) }
    fn array_start(&self, val: u64) -> Result<()> { Ok(()) }
    fn array_end(&self) -> Result<()> { Ok(()) }
    fn map_start(&self, val: u64) -> Result<()> { Ok(()) }
    fn map_end(&self) -> Result<()> { Ok(()) }
    fn tag_start(&self, val: u64) -> Result<()> { Ok(()) }
    fn tag_end(&self) -> Result<()> { Ok(()) }
    fn big_number(&self, val: &[u8], is_negative: bool, exp: i64, base: u8) -> Result<()> { Ok(()) }
}

pub struct NullVisitor;
impl Visitor for NullVisitor {}

const fn small_val(major: u8, minor: usize) -> u8 {
    assert!(minor < 24);
    major << MAJOR_SHIFT | minor as u8
}

macro_rules! impl_from_uint {
    ($([$type : ty, $size : expr])*) => {
        $(
            impl From<$type> for CBOR<[u8; $size]> {
                fn from(val: $type) -> Self {
                    let mut bytes = [0; $size];
                    bytes[0] = small_val(MAJOR_POSITIVE, MINOR_LEN1);
                    bytes[1..].copy_from_slice(&val.to_be_bytes());
                    Self(bytes)
                }
            }
        )*
    };
}

impl_from_uint!{[u8, 2] [u16, 3] [u32, 5] [u64, 9]}

macro_rules! impl_from_int {
    ($([$type : ty, $size : expr])*) => {
        $(
            impl From<$type> for CBOR<[u8; $size]> {
                fn from(val: $type) -> Self {
                    let mut bytes = [0; $size];
                    let (val, maj) = if val >= 0 {
                        (val as u16, MAJOR_POSITIVE)
                    } else {
                        (!val as u16, MAJOR_NEGATIVE)
                    };
                    bytes[0] = small_val(maj, MINOR_LEN1);
                    bytes[1..].copy_from_slice(&val.to_be_bytes());
                    Self(bytes)
                }
            }
            
        )*
    };
}

impl_from_int!{[i8, 2] [i16, 3] [i32, 5] [i64, 9]}

fn decode_header<T : AsRef<[u8]>>(value: &CBOR<T>) -> Result<(u8, u64, &[u8])> {
    const CODE : [u8; 0x20] = [
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        1, 2, 3, 4, 5, 5, 5, 6,
    ];
    let b = value.0.as_ref();
    if b.is_empty() {
        Err(Error::TooShort)
    } else {
        let major = b[0] >> MAJOR_SHIFT;
        let code = CODE[(b[0] & 0x1f) as usize];
        match code {
            0 => Ok((major, b[0] as u64, &b[1..])),
            1 => {
                if b.len() >= 2 {
                    Ok((major, u8::from_be_bytes(b[1..2].try_into().unwrap()) as u64, &b[2..]))
                } else {
                    Err(Error::TooShort)
                }
            }
            2 => {
                if b.len() >= 3 {
                    Ok((major, u16::from_be_bytes(b[1..3].try_into().unwrap()) as u64, &b[3..]))
                } else {
                    Err(Error::TooShort)
                }
            }
            3 => {
                if b.len() >= 5 {
                    Ok((major, u32::from_be_bytes(b[1..5].try_into().unwrap()) as u64, &b[5..]))
                } else {
                    Err(Error::TooShort)
                }
            }
            4 => {
                if b.len() >= 9 {
                    Ok((major, u64::from_be_bytes(b[1..9].try_into().unwrap()) as u64, &b[9..]))
                } else {
                    Err(Error::TooShort)
                }
            }
            6 => Ok((major, u64::MAX, &b[1..])),
            _ => Err(Error::InvalidCode),
        }
    }
}

macro_rules! impl_to_uint {
    ($([$type : ty, $size : expr])*) => {
        $(
            impl<T : AsRef<[u8]>> TryFrom<&CBOR<T>> for $type {
                type Error = Error;
                fn try_from(value: &CBOR<T>) -> Result<Self> {
                    let (major, minor, _next) = decode_header(value)?;
                    match major {
                        MAJOR_POSITIVE if minor as $type as u64 == minor => Ok(minor as $type),
                        _ => Err(Error::IncorrectType)
                    }
                }
            }
        )*
    };
}

impl_to_uint!{[u8, 2] [u16, 3] [u32, 5] [u64, 9]}

macro_rules! impl_to_int {
    ($([$type : ty, $size : expr])*) => {
        $(
            impl<T : AsRef<[u8]>> TryFrom<&CBOR<T>> for $type {
                type Error = Error;
                fn try_from(value: &CBOR<T>) -> Result<Self> {
                    let (major, minor, _next) = decode_header(value)?;
                    match major {
                        MAJOR_POSITIVE if minor as $type as u64 == minor => Ok(minor as $type),
                        MAJOR_NEGATIVE if !minor as $type as u64 == !minor => Ok(!minor as $type),
                        _ => Err(Error::IncorrectType)
                    }
                }
            }
        )*
    };
}

impl<T : AsRef<[u8]>> CBOR<T> {
    pub fn size(&self) -> Result<usize> {
        let v = NullVisitor;
        self.visit(v)
        // let start = self.0.as_ref();
        // let (major, minor, mut next) = decode_header(self)?;
        // match major {
        //     MAJOR_BYTES | MAJOR_STRING => {
        //         next = &next[minor as usize..];
        //     }
        //     MAJOR_ARRAY | MAJOR_MAP | MAJOR_TAG => {
        //         let count = match major {
        //             MAJOR_ARRAY => minor,
        //             MAJOR_MAP => minor*2,
        //             _ => 1,
        //         };
        //         for _ in 0..count {
        //             let size = CBOR(next).size()?;
        //             next = &next[size..];
        //         }
        //     }
        //     _ => (),
        // }
        // Ok(unsafe { next.as_ptr().offset_from(start.as_ptr()) as usize })
    }

    pub fn visit<V : Visitor>(&self, visitor: V) -> Result<usize> {
        let start = self.0.as_ref();
        let (major, minor, mut next) = decode_header(self)?;
        match major {
            MAJOR_POSITIVE => {
                visitor.integer(minor, false)?;
            }
            MAJOR_NEGATIVE => {
                visitor.integer(minor, true)?;
            }
            MAJOR_BYTES => {
                if next.len() < minor as usize {
                    return Err(Error::TooShort);
                }
                visitor.bytes(&next[0..minor as usize])?;
                next = &next[minor as usize..];
            }
            MAJOR_STRING => {
                if next.len() < minor as usize {
                    return Err(Error::TooShort);
                }
                let s = std::str::from_utf8(&next[0..minor as usize]).map_err(|_| Error::NonUFT8String)?;
                visitor.string(s)?;
                next = &next[minor as usize..];
            }
            MAJOR_ARRAY => {
                visitor.array_start(minor)?;
                for _ in 0..minor {
                    if next.first() == Some(&0xff) {
                        next = &next[1..];
                        break;
                    }
                    let len = CBOR(next).visit(visitor)?;
                    next = &next[len..];
                }
                visitor.array_end()?;
            }
            MAJOR_MAP => {
                visitor.map_start(minor)?;
                for _ in 0..minor {
                    if next.first() == Some(&0xff) {
                        next = &next[1..];
                        break;
                    }
                    let len = CBOR(next).visit(visitor)?;
                    next = &next[len..];
                    let len = CBOR(next).visit(visitor)?;
                    next = &next[len..];
                }
                visitor.map_end()?;
            }
            MAJOR_TAG => {
                match minor {
                    _ => {
                        visitor.tag_start(minor)?;
                        let len = CBOR(next).visit(visitor)?;
                        next = &next[len..];
                        visitor.tag_end()?;
                    }
                }
            }
            MAJOR_SIMPLE => {
                let hdr_len = unsafe { next.as_ptr().offset_from(start.as_ptr()) as usize };
                match hdr_len {
                    2 => {
                        if minor < 32 {
                            return Err(Error::InvalidCode);
                        } else {
                            visitor.simple(minor)?
                        }
                    }
                    3 => return Err(Error::InvalidCode), // fp16
                    5 => visitor.float(f32::from_bits(minor as u32) as f64)?,
                    9 => visitor.float(f64::from_bits(minor))?,
                    _ => match minor {
                        0..=19 => visitor.simple(minor)?,
                        20 => visitor.bool(false)?,
                        21 => visitor.bool(true)?,
                        22 => visitor.null()?,
                        23 => visitor.undefined()?,
                        _ => return Err(Error::InvalidCode)
                    }
                }
            }
        }
        Ok(unsafe { next.as_ptr().offset_from(start.as_ptr()) as usize })
    }
}
