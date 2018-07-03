extern crate flate2;
extern crate byteorder;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use byteorder::ReadBytesExt;
use byteorder::BigEndian;

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

enum NbtTag {
    Byte(u8),
    Short(i16),
    Int(i32),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    //List(Vec<T>),
    Compound(HashMap<String, NbtTag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>)
}

/*
impl NbtTag {


    fn read<T: ReadBytesExt>(reader: &mut T) -> NbtTag {
        match reader.read_u8().unwrap() {
            0xA => NbtTag::read_compound_tag(reader)
        }
    }

    fn read_compound_tag<T: ReadBytesExt>(reader: &mut T)  {
        let len = reader.read_i16::<BigEndian>().unwrap();
        let mut buf = Vec::with_capacity(len as usize);

        reader.read_exact(&mut buf);

    }
} */




///////////////////////

/*
pub struct NbtContainer {
    values: HashMap<String, NbtTag>
}

impl NbtContainer {

    pub fn from_file_uncompressed(file: File) -> NbtContainer {

    }
}*/




fn main() {
    let mut reader = File::open("test.nbt").unwrap();
    reader.read_u8();
    let len = reader.read_i16::<BigEndian>().unwrap();
    let mut buf = Vec::with_capacity(len as usize);

    read_slice(&mut reader, &mut buf, len);

    println!("{:?}", String::from_utf8(buf).unwrap());


}


fn read_slice<T: ReadBytesExt>(reader: &mut T, buf: &mut Vec<u8>, size: i16) {
    for i in 0..size {
        buf.push(reader.read_u8().unwrap());
    }
}