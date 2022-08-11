
use super::constants::*;
use super::{Result, Error};

pub trait Encode {
    fn cbor_size(&self) -> usize;
    fn cbor_encode(&self, dest: &mut [u8]) -> Result<usize>;
}

const fn header_size(minor: u64) -> usize {
    if minor < 24 {
        1
    } else if minor < 0x100 {
        2
    } else if minor < 0x10000 {
        3
    } else if minor < 0x100000000 {
        5
    } else {
        9
    }
}

fn encode_header(dest: &mut [u8], major: u8, minor: u64) -> Result<usize> {
    if minor < 24 {
        if dest.len() < 1 { return Err(Error::TooShort); }
        dest[0] = major * 32 + minor as u8;
        Ok(1)
    } else if minor < 0x100 {
        if dest.len() < 2 { return Err(Error::TooShort); }
        dest[0] = major * 32 + MINOR_LEN1 as u8;
        dest[1..].copy_from_slice(&u8::to_be_bytes(minor as u8));
        Ok(2)
    } else if minor < 0x10000 {
        if dest.len() < 3 { return Err(Error::TooShort); }
        dest[0] = major * 32 + MINOR_LEN2 as u8;
        dest[1..].copy_from_slice(&u16::to_be_bytes(minor as u16));
        Ok(3)
    } else if minor < 0x100000000 {
        if dest.len() < 5 { return Err(Error::TooShort); }
        dest[0] = major * 32 + MINOR_LEN4 as u8;
        dest[1..].copy_from_slice(&u32::to_be_bytes(minor as u32));
        Ok(5)
    } else {
        if dest.len() < 9 { return Err(Error::TooShort); }
        dest[0] = major * 32 + MINOR_LEN8 as u8;
        dest[1..].copy_from_slice(&u64::to_be_bytes(minor));
        Ok(9)
    }
}

macro_rules! impl_from_uint {
    ($($type : ty)*) => {
        $(
            impl Encode for $type {
                fn cbor_size(&self) -> usize {
                    header_size(*self as u64)
                }
                fn cbor_encode(&self, dest: &mut [u8]) -> Result<usize> {
                    encode_header(dest, MAJOR_POSITIVE, *self as u64)
                }
            }
        )*
    }
}

impl_from_uint!{u8 u16 u32 u64}

macro_rules! impl_from_int {
    ($($type : ty)*) => {
        $(
            impl Encode for $type {
                fn cbor_size(&self) -> usize {
                    let val = if *self >= 0 {
                        *self
                    } else {
                        !*self
                    };
                    header_size(val as u64)
                }
                fn cbor_encode(&self, dest: &mut [u8]) -> Result<usize> {
                    let (val, maj) = if *self >= 0 {
                        (*self, MAJOR_POSITIVE)
                    } else {
                        (!*self, MAJOR_NEGATIVE)
                    };
                    encode_header(dest, maj, val as u64)
                }
            }
        )*
    }
}

impl_from_int!{i8 i16 i32 i64}

impl Encode for &str {
    fn cbor_size(&self) -> usize {
        header_size(self.len() as u64) + self.len()
    }
    fn cbor_encode(&self, dest: &mut [u8]) -> Result<usize> {
        let hdr_size = encode_header(dest, MAJOR_STRING, self.len() as u64)?;
        dest[hdr_size..].copy_from_slice(&self.as_bytes());
        Ok(self.cbor_size())
    }
}

impl Encode for &[u8] {
    fn cbor_size(&self) -> usize {
        header_size(self.len() as u64) + self.len()
    }
    fn cbor_encode(&self, dest: &mut [u8]) -> Result<usize> {
        let size = self.cbor_size();
        let hdr_size = encode_header(dest, MAJOR_STRING, self.len() as u64)?;
        dest[hdr_size..].copy_from_slice(&self);
        Ok(size)
    }
}

impl Encode for f32 {
    fn cbor_size(&self) -> usize {
        5
    }
    fn cbor_encode(&self, dest: &mut [u8]) -> Result<usize> {
        if dest.len() < self.cbor_size() { return Err(Error::TooShort); }
        dest[0] = MAJOR_SIMPLE * 32 + MINOR_LEN4 as u8;
        dest[1..].copy_from_slice(&self.to_be_bytes());
        Ok(self.cbor_size())
    }
}

impl Encode for f64 {
    fn cbor_size(&self) -> usize {
        9
    }
    fn cbor_encode(&self, dest: &mut [u8]) -> Result<usize> {
        if dest.len() < self.cbor_size() { return Err(Error::TooShort); }
        dest[0] = MAJOR_SIMPLE * 32 + MINOR_LEN8 as u8;
        dest[1..].copy_from_slice(&self.to_be_bytes());
        Ok(self.cbor_size())
    }
}

impl Encode for bool {
    fn cbor_size(&self) -> usize {
        1
    }
    fn cbor_encode(&self, dest: &mut [u8]) -> Result<usize> {
        if dest.len() < self.cbor_size() { return Err(Error::TooShort); }
        dest[0] = MAJOR_SIMPLE * 32 + SIMPLE_FALSE as u8 + *self as u8;
        Ok(self.cbor_size())
    }
}

impl Encode for () {
    fn cbor_size(&self) -> usize {
        1
    }
    fn cbor_encode(&self, dest: &mut [u8]) -> Result<usize> {
        if dest.len() < self.cbor_size() { return Err(Error::TooShort); }
        dest[0] = MAJOR_SIMPLE * 32 + SIMPLE_NULL as u8;
        Ok(self.cbor_size())
    }
}

impl<T : Encode> Encode for Option<T> {
    fn cbor_size(&self) -> usize {
        if let Some(val) = self {
            val.cbor_size()
        } else {
            ().cbor_size()
        }
    }

    fn cbor_encode(&self, dest: &mut [u8]) -> Result<usize> {
        assert!(dest.len() >= self.cbor_size());
        if let Some(val) = self {
            val.cbor_encode(dest)
        } else {
            ().cbor_encode(dest)
        }
    }
}


fn array_cbor_size<T : Encode, V : AsRef<[T]>>(vec: &V) -> usize {
    let hdr_size = header_size(vec.as_ref().len() as u64);
    hdr_size + vec.as_ref().iter().map(|val| val.cbor_size()).sum::<usize>()
}

fn array_cbor_encode<T : Encode, V : AsRef<[T]>>(vec: &V, dest: &mut [u8]) -> Result<usize> {
    let mut size = encode_header(dest, MAJOR_ARRAY, vec.as_ref().len() as u64)?;
    for val in vec.as_ref() {
        size += val.cbor_encode(&mut dest[size..])?;
    }
    Ok(size)
}

impl<T : Encode> Encode for Vec<T> {
    fn cbor_size(&self) -> usize {
        array_cbor_size(self)
    }

    fn cbor_encode(&self, dest: &mut [u8]) -> Result<usize> {
        array_cbor_encode(self, dest)
    }
}

impl<T : Encode, const N: usize> Encode for [T; N] {
    fn cbor_size(&self) -> usize {
        array_cbor_size(self)
    }

    fn cbor_encode(&self, dest: &mut [u8]) -> Result<usize> {
        array_cbor_encode(self, dest)
    }
}

impl<A : Encode, B: Encode> Encode for (A, B) {
    fn cbor_size(&self) -> usize {
        let hdr_size = header_size(2);
        hdr_size + self.0.cbor_size() + self.1.cbor_size()
    }

    fn cbor_encode(&self, dest: &mut [u8]) -> Result<usize> {
        let mut size = encode_header(dest, MAJOR_ARRAY, 2)?;
        size += self.0.cbor_encode(&mut dest[size..])?;
        size += self.1.cbor_encode(&mut dest[size..])?;
        Ok(size)
     }
}
