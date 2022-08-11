
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

pub const MINOR_LEN1 : usize = 24;
pub const MINOR_LEN2 : usize = 25;
pub const MINOR_LEN4 : usize = 26;
pub const MINOR_LEN8 : usize = 27;

pub const MINOR_INDEFINITE : u64 = 31;

pub const SIMPLE_FALSE : u64 = 20;
pub const SIMPLE_TRUE : u64 = 21;
pub const SIMPLE_NULL : u64 = 22;
pub const SIMPLE_UNDEFINED : u64 = 23;
