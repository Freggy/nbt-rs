extern crate flate2;
extern crate byteorder;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use byteorder::ReadBytesExt;
use byteorder::BigEndian;
use std::io::Error;
use std::io::ErrorKind;

const TAG_END       : u8 = 0x0;
const TAG_BYTE      : u8 = 0x1;
const TAG_SHORT     : u8 = 0x2;
const TAG_INT       : u8 = 0x3;
const TAG_LONG      : u8 = 0x4;
const TAG_FLOAT     : u8 = 0x5;
const TAG_DOUBLE    : u8 = 0x6;
const TAG_BYTE_ARRAY: u8 = 0x7;
const TAG_STRING    : u8 = 0x8;
const TAG_LIST      : u8 = 0x9;
const TAG_COMPOUND  : u8 = 0xA;
const TAG_INT_ARRAY : u8 = 0xB;
const TAG_LONG_ARRAY: u8 = 0xC;

/// Represents an NBT tag.
pub enum NbtTag {
    Byte(u8),
    Short(i16),
    Int(i32),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    List(Vec<NbtTag>),
    Compound(HashMap<String, NbtTag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

pub enum Compression {
    Gzip,
    Zlib,
    None,
}

impl NbtTag {
    pub fn as_byte(&self) -> Option<u8> {
        match *self {
            NbtTag::Byte(val) => Some(val),
            _ => None
        }
    }

    pub fn as_short(&self) -> Option<i16> {
        match *self {
            NbtTag::Short(val) => Some(val),
            _ => None
        }
    }

    pub fn as_int(&self) -> Option<i32> {
        match *self {
            NbtTag::Int(val) => Some(val),
            _ => None
        }
    }

    pub fn as_float(&self) -> Option<f32> {
        match *self {
            NbtTag::Float(val) => Some(val),
            _ => None
        }
    }

    pub fn as_double(&self) -> Option<f64> {
        match *self {
            NbtTag::Double(val) => Some(val),
            _ => None
        }
    }

    pub fn as_byte_arr(&self) -> Option<Vec<u8>> {
        match *self {
            NbtTag::ByteArray(val) => Some(val),
            _ => None
        }
    }

    pub fn as_list(&self) -> Option<Vec<NbtTag>> {
        match *self {
            NbtTag::List(val) => Some(val),
            _ => None
        }
    }

    pub fn as_compound(&self) -> Option<HashMap<String, NbtTag>> {
        match *self {
            NbtTag::Compound(val) => Some(val),
            _ => None
        }
    }

    pub fn as_int_arr(&self) -> Option<Vec<i32>> {
        match *self {
            NbtTag::IntArray(val) => Some(val),
            _ => None
        }
    }

    pub fn as_long_arr(&self) -> Option<Vec<i64>> {
        match *self {
            NbtTag::LongArray(val) => Some(val),
            _ => None
        }
    }

    pub fn read(file: File, comp: Compression) {
        // TODO: use flate2 for decompression
        match comp {
            Compression::Gzip => (),
            Compression::Zlib => (),
            Compression::None => (),
        }
    }
}

///////////////////////

pub struct NbtContainer {
    values: HashMap<String, NbtTag>
}

impl NbtContainer {
    pub fn from_file_uncompressed(file: File)  {}

    fn read<R: ReadBytesExt>(reader: &mut R) -> Result<NbtTag, Error> {
        match reader.read_u8()? {
            TAG_COMPOUND => NbtContainer::read_compound_tag(reader),
            _ => Err(Error::new(ErrorKind::Other, "Unknown NBT identifier"))
        }
    }

    fn read_compound_tag<T: ReadBytesExt>(reader: &mut T) -> Result<NbtTag, Error> {
        let mut tags = HashMap::new();
        let tag_name = NbtContainer::read_utf8_string(reader)?;
        loop {
            let id = reader.read_u8()?;
            if id == TAG_END {
                break;
            }
            let name = NbtContainer::read_utf8_string(reader)?;
            let tag = NbtContainer::read(reader)?;
            tags.insert(name, tag);
        }
        Ok(NbtTag::Compound(tags))
    }

    /// Reads a UTF8 string.
    fn read_utf8_string<T: ReadBytesExt>(reader: &mut T) -> Result<String, Error> {
        let len = reader.read_i16::<BigEndian>()?;
        let mut buf = Vec::with_capacity(len as usize);
        NbtContainer::read_slice(reader, &mut buf, len);
        Ok(String::from_utf8(buf).unwrap())
    }

    /// Reads a slice of bytes of given length from reader and saves it in a given buffer.
    fn read_slice<T: ReadBytesExt>(reader: &mut T, buf: &mut Vec<u8>, size: i16) {
        for i in 0..size {
            buf.push(reader.read_u8().unwrap());
        }
    }
}


fn main() {}
