//! All IO functionality needed for TIFF decoding

use std::io;
use std::io::{Read, Seek};
use byteorder::{self, ReadBytesExt, BigEndian, LittleEndian};

/// Byte order of the TIFF file.
#[derive(Clone, Copy, Debug)]
pub enum ByteOrder {
    /// little endian byte order
    LittleEndian,
    /// big endian byte order
    BigEndian
}


/// Reader that is aware of the byte order.
pub trait EndianReader: Read {
    /// Byte order that should be adhered to
    fn byte_order(&self) -> ByteOrder;

    /// Reads an u16
    #[inline(always)]
    fn read_u16(&mut self) -> Result<u16, byteorder::Error> {
        match self.byte_order() {
            ByteOrder::LittleEndian => <Self as ReadBytesExt>::read_u16::<LittleEndian>(self),
            ByteOrder::BigEndian => <Self as ReadBytesExt>::read_u16::<BigEndian>(self)
        }
    }

    /// Reads an u32
    #[inline(always)]
    fn read_u32(&mut self) -> Result<u32, byteorder::Error> {
        match self.byte_order() {
            ByteOrder::LittleEndian => <Self as ReadBytesExt>::read_u32::<LittleEndian>(self),
            ByteOrder::BigEndian => <Self as ReadBytesExt>::read_u32::<BigEndian>(self)
        }
    }
}
