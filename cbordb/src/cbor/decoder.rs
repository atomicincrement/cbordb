
use super::constants::*;
use super::CBOR;
use super::{Result, Error, Kind};
use std::fmt::Display;

pub trait Visitor {
    fn integer(&mut self, _val: i128) -> Result<()> { Ok(()) }
    fn bytes(&mut self, _val: &[u8]) -> Result<()> { Ok(()) }
    fn string(&mut self, _val: &str) -> Result<()> { Ok(()) }
    fn null(&mut self) -> Result<()> { Ok(()) }
    fn undefined(&mut self) -> Result<()> { Ok(()) }
    fn bool(&mut self, _val: bool) -> Result<()> { Ok(()) }
    fn float(&mut self, _val: f64) -> Result<()> { Ok(()) }
    fn simple(&mut self, _val: u64) -> Result<()> { Ok(()) }
    fn array_start(&mut self, _val: Option<u64>) -> Result<()> { Ok(()) }
    fn array_separator(&mut self) -> Result<()> { Ok(()) }
    fn array_end(&mut self) -> Result<()> { Ok(()) }
    fn map_start(&mut self, _val: Option<u64>) -> Result<()> { Ok(()) }
    fn map_colon(&mut self) -> Result<()> { Ok(()) }
    fn map_separator(&mut self) -> Result<()> { Ok(()) }
    fn map_end(&mut self) -> Result<()> { Ok(()) }
    fn tag_start(&mut self, _val: u64) -> Result<()> { Ok(()) }
    fn tag_end(&mut self) -> Result<()> { Ok(()) }
    fn big_number(&mut self, _val: &[u8], _is_negative: bool, _exp: i64, _base: u8) -> Result<()> { Ok(()) }
}

const KINDS : [Kind; 256] = [
    Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer,
    Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer,
    Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer,
    Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Invalid, Kind::Invalid, Kind::Invalid, Kind::Invalid,
    Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer,
    Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer,
    Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer,
    Kind::Integer, Kind::Integer, Kind::Integer, Kind::Integer, Kind::Invalid, Kind::Invalid, Kind::Invalid, Kind::Invalid,
    Kind::Bytes, Kind::Bytes, Kind::Bytes, Kind::Bytes, Kind::Bytes, Kind::Bytes, Kind::Bytes, Kind::Bytes,
    Kind::Bytes, Kind::Bytes, Kind::Bytes, Kind::Bytes, Kind::Bytes, Kind::Bytes, Kind::Bytes, Kind::Bytes,
    Kind::Bytes, Kind::Bytes, Kind::Bytes, Kind::Bytes, Kind::Bytes, Kind::Bytes, Kind::Bytes, Kind::Bytes,
    Kind::Bytes, Kind::Bytes, Kind::Bytes, Kind::Bytes, Kind::Invalid, Kind::Invalid, Kind::Invalid, Kind::Invalid,
    Kind::String, Kind::String, Kind::String, Kind::String, Kind::String, Kind::String, Kind::String, Kind::String,
    Kind::String, Kind::String, Kind::String, Kind::String, Kind::String, Kind::String, Kind::String, Kind::String,
    Kind::String, Kind::String, Kind::String, Kind::String, Kind::String, Kind::String, Kind::String, Kind::String,
    Kind::String, Kind::String, Kind::String, Kind::String, Kind::Invalid, Kind::Invalid, Kind::Invalid, Kind::Invalid,
    Kind::Array, Kind::Array, Kind::Array, Kind::Array, Kind::Array, Kind::Array, Kind::Array, Kind::Array,
    Kind::Array, Kind::Array, Kind::Array, Kind::Array, Kind::Array, Kind::Array, Kind::Array, Kind::Array,
    Kind::Array, Kind::Array, Kind::Array, Kind::Array, Kind::Array, Kind::Array, Kind::Array, Kind::Array,
    Kind::Array, Kind::Array, Kind::Array, Kind::Array, Kind::Invalid, Kind::Invalid, Kind::Invalid, Kind::Array,
    Kind::Map, Kind::Map, Kind::Map, Kind::Map, Kind::Map, Kind::Map, Kind::Map, Kind::Map,
    Kind::Map, Kind::Map, Kind::Map, Kind::Map, Kind::Map, Kind::Map, Kind::Map, Kind::Map,
    Kind::Map, Kind::Map, Kind::Map, Kind::Map, Kind::Map, Kind::Map, Kind::Map, Kind::Map,
    Kind::Map, Kind::Map, Kind::Map, Kind::Map, Kind::Invalid, Kind::Invalid, Kind::Invalid, Kind::Map,
    Kind::Time, Kind::Time, Kind::BigNumber, Kind::BigNumber, Kind::Tag, Kind::Tag, Kind::Tag, Kind::Tag,
    Kind::Tag, Kind::Tag, Kind::Tag, Kind::Tag, Kind::Tag, Kind::Tag, Kind::Tag, Kind::Tag,
    Kind::Tag, Kind::Tag, Kind::Tag, Kind::Tag, Kind::Tag, Kind::Tag, Kind::Tag, Kind::Tag,
    Kind::Tag, Kind::Tag, Kind::Tag, Kind::Tag, Kind::Invalid, Kind::Invalid, Kind::Invalid, Kind::Invalid,
    Kind::Simple, Kind::Simple, Kind::Simple, Kind::Simple, Kind::Simple, Kind::Simple, Kind::Simple, Kind::Simple,
    Kind::Simple, Kind::Simple, Kind::Simple, Kind::Simple, Kind::Simple, Kind::Simple, Kind::Simple, Kind::Simple,
    Kind::Simple, Kind::Simple, Kind::Simple, Kind::Simple, Kind::Bool, Kind::Bool, Kind::Null, Kind::Undefined,
    Kind::Simple, Kind::Float, Kind::Float, Kind::Float, Kind::Invalid, Kind::Invalid, Kind::Invalid, Kind::Break,
];

pub struct NullVisitor;
impl Visitor for NullVisitor {}
pub struct FmtVisitor<'r, 'fmt>(&'r mut std::fmt::Formatter<'fmt>);

impl<'r, 'fmt> Visitor for FmtVisitor<'r, 'fmt> {
    fn integer(&mut self, val: i128) -> Result<()> { Ok(Display::fmt(&val, self.0)?) }
    fn bytes(&mut self, val: &[u8]) -> Result<()> { Ok(write!(self.0, "h'{}'", hex::encode(val))?) }
    fn string(&mut self, val: &str) -> Result<()> { Ok(Display::fmt(&val, self.0)?) }
    fn null(&mut self) -> Result<()> { Ok(Display::fmt("null", self.0)?) }
    fn undefined(&mut self) -> Result<()> { Ok(Display::fmt("undefined", self.0)?) }
    fn bool(&mut self, val: bool) -> Result<()> { Ok(Display::fmt(&val, self.0)?) }
    fn float(&mut self, val: f64) -> Result<()> { Ok(Display::fmt(&val, self.0)?) }
    fn simple(&mut self, val: u64) -> Result<()> { Ok(write!(self.0, "simple({})", val)?) }
    fn array_start(&mut self, _val: Option<u64>) -> Result<()> { Ok(Display::fmt("[", self.0)?) }
    fn array_separator(&mut self) -> Result<()> { Ok(Display::fmt(", ", self.0)?) }
    fn array_end(&mut self) -> Result<()> { Ok(Display::fmt("]", self.0)?) }
    fn map_start(&mut self, _val: Option<u64>) -> Result<()> { Ok(Display::fmt("{", self.0)?) }
    fn map_end(&mut self) -> Result<()> { Ok(Display::fmt("}", self.0)?) }
    fn map_colon(&mut self) -> Result<()> { Ok(Display::fmt(": ", self.0)?) }
    fn map_separator(&mut self) -> Result<()> { Ok(Display::fmt(", ", self.0)?) }
    fn tag_start(&mut self, val: u64) -> Result<()> { Ok(write!(self.0, "{}(", val)?) }
    fn tag_end(&mut self) -> Result<()> { Ok(Display::fmt(")", self.0)?) }
    fn big_number(&mut self, _val: &[u8], _is_negative: bool, _exp: i64, _base: u8) -> Result<()> {
        Ok(())
    }
}

impl<'fmt, T : AsRef<[u8]>> std::fmt::Display for CBOR<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut visitor = FmtVisitor(f);
        let r = self.visit(&mut visitor);
        if let Err(Error::FormatError(e)) = r {
            Err(e)
        } else if let Err(e) = r {
            Ok(write!(f, "..Err({:?})", e)?)
        } else {
            Ok(())
        }
    }
}

impl<'fmt, T : AsRef<[u8]>> std::fmt::Debug for CBOR<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl<'bytes> CBOR<&'bytes [u8]> {
    pub fn next(&self) -> Result<CBOR<&'bytes [u8]>> {
        let (major, minor, mut next) = decode_header(self.0)?;
        match major {
            MAJOR_POSITIVE | MAJOR_NEGATIVE | MAJOR_SIMPLE => {
                minor.ok_or(Error::InvalidCode)?;
            }
            MAJOR_BYTES | MAJOR_STRING => {
                let minor = minor.ok_or(Error::InvalidCode)?;
                if next.len() < minor as usize {
                    return Err(Error::TooShort);
                }
                next = &next[minor as usize..];
            }
            MAJOR_ARRAY | MAJOR_MAP | MAJOR_TAG => {
                let count = match (major, minor) {
                    (MAJOR_ARRAY, Some(minor)) => minor,
                    (MAJOR_ARRAY, None) => u64::MAX,
                    (MAJOR_MAP, Some(minor)) => minor.saturating_mul(2),
                    (MAJOR_MAP, None) => u64::MAX,
                    _ => 1,
                };
                for _ in CBORIter::new(&mut self, count) {

                }
                for _ in 0..count {
                    if next.first() == Some(&0xff) {
                        break;
                    }
                    let size = CBOR(next).size()?;
                    next = &next[size..];
                }
            }
            _ => (),
        }
        Ok(CBOR(next))
    }

    pub fn size(&self) -> Result<usize> {
        let start = self.0.as_ref();
        Ok(unsafe { self.next()?.0.as_ptr().offset_from(start.as_ptr()) as usize })
    }
}

impl<T : AsRef<[u8]>> CBOR<T> {
    /// Return the appropriate Kind for this CBOR element.
    pub fn kind(&self) -> Kind {
        self.0.as_ref().first().map(|b| KINDS[*b as usize]).unwrap_or(Kind::Invalid)
    }

    pub fn visit<V : Visitor>(&self, visitor: &mut V) -> Result<usize> {
        let start = self.0.as_ref();
        let (major, minor, mut next) = decode_header(start)?;
        match major {
            MAJOR_POSITIVE => {
                visitor.integer(minor.ok_or(Error::InvalidCode)? as i128)?;
            }
            MAJOR_NEGATIVE => {
                visitor.integer(!(minor.ok_or(Error::InvalidCode)? as i128))?;
            }
            MAJOR_BYTES => {
                // TODO: indefinite bytes
                let minor = minor.ok_or(Error::InvalidCode)?;
                if next.len() < minor as usize {
                    return Err(Error::TooShort);
                }
                visitor.bytes(&next[0..minor as usize])?;
                next = &next[minor as usize..];
            }
            MAJOR_STRING => {
                // TODO: indefinite strings
                let minor = minor.ok_or(Error::InvalidCode)?;
                if next.len() < minor as usize {
                    return Err(Error::TooShort);
                }
                let s = std::str::from_utf8(&next[0..minor as usize]).map_err(|_| Error::NonUFT8String)?;
                visitor.string(s)?;
                next = &next[minor as usize..];
            }
            MAJOR_ARRAY => {
                visitor.array_start(minor)?;
                let is_breakable = minor.is_none();
                let minor = minor.unwrap_or(!0);
                for i in 0..minor {
                    if next.first() == Some(&0xff) && is_breakable {
                        next = &next[1..];
                        break;
                    }
                    if i != 0 { visitor.array_separator()? }
                    let len = CBOR(next).visit(visitor)?;
                    next = &next[len..];
                }
                visitor.array_end()?;
            }
            MAJOR_MAP => {
                visitor.map_start(minor)?;
                let is_breakable = minor.is_none();
                let minor = minor.unwrap_or(!0);
                for i in 0..minor {
                    if next.first() == Some(&0xff) && is_breakable {
                        next = &next[1..];
                        break;
                    }
                    if i != 0 { visitor.map_separator()? }
                    let len = CBOR(next).visit(visitor)?;
                    next = &next[len..];
                    visitor.map_colon()?;
                    let len = CBOR(next).visit(visitor)?;
                    next = &next[len..];
                }
                visitor.map_end()?;
            }
            MAJOR_TAG => {
                let minor = minor.ok_or(Error::InvalidCode)?;
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
                let minor = minor.ok_or(Error::InvalidCode)?;
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
                        SIMPLE_FALSE => visitor.bool(false)?,
                        SIMPLE_TRUE => visitor.bool(true)?,
                        SIMPLE_NULL => visitor.null()?,
                        SIMPLE_UNDEFINED => visitor.undefined()?,
                        _ => return Err(Error::InvalidCode)
                    }
                }
            }
            _ => unreachable!(),
        }
        Ok(unsafe { next.as_ptr().offset_from(start.as_ptr()) as usize })
    }

    /// For a definite length map of string keys, get a value.
    pub fn get<'key>(&self, key : &'key str) -> Option<CBOR<&[u8]>> {
        let start = self.0.as_ref();
        if let (MAJOR_MAP, Some(len), mut next) = decode_header(start).ok()? {
            while len != 0 {
                if let Ok(s) = <&str>::decode(&mut next) {
                    if s == key {
                    }
                }
            }
        }
        None
    }

    /// For an array get a value.
    pub fn index(&self, index : usize) -> Option<CBOR<&[u8]>> {
        None
    }
}

fn decode_header<'bytes>(b: &'bytes [u8]) -> Result<(u8, Option<u64>, &'bytes [u8])> {
    const CODE : [u8; 0x20] = [
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        1, 2, 3, 4, 5, 5, 5, 6,
    ];
    if b.is_empty() {
        Err(Error::TooShort)
    } else {
        let major = b[0] >> MAJOR_SHIFT;
        let code = CODE[(b[0] & 0x1f) as usize];
        match code {
            0 => Ok((major, Some((b[0] & 0x1f) as u64), &b[1..])),
            1 => {
                if b.len() >= 2 {
                    Ok((major, Some(u8::from_be_bytes(b[1..2].try_into().unwrap()) as u64), &b[2..]))
                } else {
                    Err(Error::TooShort)
                }
            }
            2 => {
                if b.len() >= 3 {
                    Ok((major, Some(u16::from_be_bytes(b[1..3].try_into().unwrap()) as u64), &b[3..]))
                } else {
                    Err(Error::TooShort)
                }
            }
            3 => {
                if b.len() >= 5 {
                    Ok((major, Some(u32::from_be_bytes(b[1..5].try_into().unwrap()) as u64), &b[5..]))
                } else {
                    Err(Error::TooShort)
                }
            }
            4 => {
                if b.len() >= 9 {
                    Ok((major, Some(u64::from_be_bytes(b[1..9].try_into().unwrap()) as u64), &b[9..]))
                } else {
                    Err(Error::TooShort)
                }
            }
            6 => Ok((major, None, &b[1..])),
            _ => Err(Error::InvalidCode),
        }
    }
}

pub struct CBORIter<'cbor, 'bytes> {
    cbor: &'cbor mut  CBOR<&'bytes [u8]>,
    items: Option<u64>,
}

impl<'cbor, 'bytes> CBORIter<'cbor, 'bytes> {
    pub fn new(cbor: &'cbor mut  CBOR<&'bytes [u8]>, items: Option<u64>) -> Self {
        Self { cbor, items }
    }

}

impl<'cbor, 'bytes> Iterator for CBORIter<'cbor, 'bytes> where 'cbor: 'bytes {
    type Item = CBOR<&'bytes [u8]>;
    /// Get the next cbor element in an array, map or tag.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(items) = self.items {
            if items == 0 {
                return None;
            }
            self.items = Some(items-1);
        }
        let res = self.cbor.0;
        let next = self.cbor.next().ok()?;
        self.cbor.0 = next.0;
        Some(CBOR(res))
    }
}

pub trait Decode<'s> : Sized {
    fn decode(src: &mut &'s [u8]) -> Result<Self>;
}

impl<'s> Decode<'s> for &'s str {
    fn decode(src: &mut &'s [u8]) -> Result<Self> {
        let (major, minor, next) = decode_header(src)?;
        let minor = minor.ok_or(Error::InvalidCode)? as usize;
        
        match major {
            MAJOR_STRING if next.len() >= minor => {
                *src = &next[minor..];
                Ok(std::str::from_utf8(&next[0..minor]).map_err(|_| Error::NonUFT8String)?)
            }
            _ => Err(Error::IncorrectType)
        }
    }
}

impl<'s> Decode<'s> for &'s [u8] {
    fn decode(src: &mut &'s [u8]) -> Result<Self> {
        let (major, minor, next) = decode_header(src)?;
        let minor = minor.ok_or(Error::InvalidCode)? as usize;
        
        match major {
            MAJOR_BYTES if next.len() >= minor => {
                *src = &next[minor..];
                Ok(&next[0..minor])
            }
            _ => Err(Error::IncorrectType)
        }
    }
}

impl<'s> Decode<'s> for i32 {
    fn decode(src: &mut &'s [u8]) -> Result<Self> {
        let (major, minor, next) = decode_header(src)?;
        let minor = minor.ok_or(Error::InvalidCode)? as usize;

        match major {
            MAJOR_POSITIVE => { *src = next; Ok(minor.try_into().map_err(|_| Error::NumberTooBig)?) }
            MAJOR_NEGATIVE => { *src = next; Ok((!minor).try_into().map_err(|_| Error::NumberTooBig)?) }
            _ => Err(Error::IncorrectType)
        }
    }
}

