extern crate flate2;
extern crate byteorder;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use byteorder::ReadBytesExt;
use byteorder::BigEndian;
use std::io::Error;

/// The binray ID of each NBT tag.
enum NbtTagId {
    End = 0x0,
    Byte = 0x1,
    Short = 0x2,
    Int = 0x3,
    Float = 0x4,
    Double = 0x5,
    ByteArray = 0x6,
    String = 0x7,
    List = 0x8,
    Compound = 0xA,
    IntArray = 0xB,
    LongArray = 0xC
}

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
    LongArray(Vec<i64>)
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
}




///////////////////////

pub struct NbtReader {
    values: HashMap<String, NbtTag>
}

impl NbtContainer {

    pub fn from_file_uncompressed(file: File) -> NbtContainer {

    }

    fn read<R: ReadBytesExt>(reader: &mut R) -> Result<NbtTag, Error> {

        match reader.read_u8()? {
            0xA => read_compound_tag(reader)?
        }

    }

    fn read_compound_tag<T: ReadBytesExt>(reader: &mut T) -> Result<NbtTag, Error> {
        let mut tags = HashMap::new();

        // Read compound tag name
        let name_len = reader.read_i16::<BigEndian>()?;
        let mut buf = Vec::with_capacity(name_len as usize);
        read_slice(&mut reader, &mut buf, len);
        let tag_name = String::from_utf8(buf)?;

        loop {
            let id = reader.read_u8()?;
            if id == NbtTagId::End {
                break;
            }
            let name = "bla"; // TODO: read tag name
            let tag = NbtContainer::read(reader)?;
            tags.insert(name, tag)
        }

        Ok(NbtTag::Compound(TagInfo {
            name: tag_name,
            value: tags
        }))
    }
}




fn main() {
    let mut reader = File::open("test.nbt").unwrap();
    reader.read_u8();
    let len = reader.read_i16::<BigEndian>().unwrap();
    let mut buf = Vec::with_capacity(len as usize);

    read_slice(&mut reader, &mut buf, len);

    println!("{:?}", String::from_utf8(buf).unwrap());
}


fn read_slice<T: ReadBytesExt>(reader: &mut T, buf: &mut Vec<u8>, size: i16)  {
    for i in 0..size {
        buf.push(reader.read_u8().unwrap());
    }
}