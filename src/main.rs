extern crate flate2;
extern crate byteorder;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use byteorder::ReadBytesExt;
use byteorder::BigEndian;
use std::io::Error;
use std::io::ErrorKind;
use byteorder::ByteOrder;
use std::marker::PhantomData;

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

pub enum Compression {
    Gzip,
    Zlib,
    None,
}


//-------------------------//


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

impl NbtTag {
    pub fn as_byte(&self) -> Option<&u8> {
        match *self {
            NbtTag::Byte(ref val) => Some(val),
            _ => None
        }
    }

    pub fn as_short(&self) -> Option<&i16> {
        match *self {
            NbtTag::Short(ref val) => Some(val),
            _ => None
        }
    }

    pub fn as_int(&self) -> Option<&i32> {
        match *self {
            NbtTag::Int(ref val) => Some(val),
            _ => None
        }
    }

    pub fn as_float(&self) -> Option<&f32> {
        match *self {
            NbtTag::Float(ref val) => Some(val),
            _ => None
        }
    }

    pub fn as_double(&self) -> Option<&f64> {
        match *self {
            NbtTag::Double(ref val) => Some(val),
            _ => None
        }
    }

    pub fn as_byte_arr(&self) -> Option<&Vec<u8>> {
        match *self {
            NbtTag::ByteArray(ref val) => Some(val),
            _ => None
        }
    }

    pub fn as_list(&self) -> Option<&Vec<NbtTag>> {
        match *self {
            NbtTag::List(ref val) => Some(val),
            _ => None
        }
    }

    pub fn as_compound(&self) -> Option<&HashMap<String, NbtTag>> {
        match *self {
            NbtTag::Compound(ref val) => Some(val),
            _ => None
        }
    }

    pub fn as_int_arr(&self) -> Option<&Vec<i32>> {
        match *self {
            NbtTag::IntArray(ref val) => Some(val),
            _ => None
        }
    }

    pub fn as_long_arr(&self) -> Option<&Vec<i64>> {
        match *self {
            NbtTag::LongArray(ref val) => Some(val),
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


//-------------------------//


/// Builder for NBT tags.
pub struct NbtBuilder {
    values: HashMap<String, NbtTag>,
    root_tag_name: String
}

impl NbtBuilder {

    /// Creates a new instance of NbtBuilder. The root tag name will be empty.
    pub fn new() -> NbtBuilder {
        NbtBuilder {
            values: HashMap::new(),
            root_tag_name: String::new()
        }
    }

    /// Creates a new instance of NbtBuilder with the given root tag name.
    pub fn with_root_tag(root_tag_name: String) -> NbtBuilder {
        NbtBuilder {
            values: HashMap::new(),
            root_tag_name
        }
    }

    /// Adds an i32 to the compound.
    pub fn add_i32(mut self, key: &str, val: i32) -> NbtBuilder {
        self.values.insert(key.to_string(), NbtTag::Int(val));
        self
    }

    // TODO: Return object that contains root tag
    pub fn build(self) -> NbtTag {
        NbtTag::Compound(self.values)
    }
}


//-------------------------//


/// Provides functions for reading NBT data from compressed and uncompressed files.
pub struct NbtReader<F: ByteOrder> {
    phantom: PhantomData<F>
}

impl <F: ByteOrder> NbtReader<F> {

    /// Reads the content of the given uncompressed file and creates NBT compound tag.
    pub fn from_uncompressed_file(file: File) /*-> Result<NbtTag, Error> */{}

    fn read<R: ReadBytesExt>(&self, reader: &mut R) -> Result<NbtTag, Error> {
        match reader.read_u8()? {
            TAG_COMPOUND => self.read_compound_tag(reader),
            _ => Err(Error::new(ErrorKind::Other, "Unknown NBT identifier"))
        }
    }

    /// Reads a compound tag.
    fn read_compound_tag<R: ReadBytesExt>(&self, reader: &mut R) -> Result<NbtTag, Error> {
        let mut tags = HashMap::new();
        let tag_name = self.read_utf8_string(reader)?;
        loop {
            let id = reader.read_u8()?;
            if id == TAG_END {
                break;
            }
            let name = self.read_utf8_string(reader)?;
            let tag = self.read(reader)?;
            tags.insert(name, tag);
        }
        Ok(NbtTag::Compound(tags))
    }

    /// Reads a UTF8 string.
    fn read_utf8_string<R: ReadBytesExt>(&self, reader: &mut R) -> Result<String, Error> {
        let len = reader.read_i16::<F>()?;
        let mut buf = Vec::with_capacity(len as usize);
        self.read_slice(reader, &mut buf, len);
        Ok(String::from_utf8(buf).unwrap())
    }

    /// Reads a slice of bytes of given length from reader and saves it in a given buffer.
    fn read_slice<R: ReadBytesExt>(&self, reader: &mut R, buf: &mut Vec<u8>, size: i16) {
        for i in 0..size {
            buf.push(reader.read_u8().unwrap());
        }
    }
}

fn main() {
    let tag = NbtBuilder::new().add_i32("hallo", 2).add_i32("hello", 3).build();
    println!("{:?}", tag.as_compound().unwrap().get("hallo").unwrap().as_int());
    println!("{:?}", tag.as_compound().unwrap().get("hello").unwrap().as_int());
}
