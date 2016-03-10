use std::io::{self, Read, Seek};

use image::{
    ImageError,
    ImageResult,
    ImageDecoder,
    DecodingResult,
};

use self::ifd::Directory;

use self::stream::{
    ByteOrder,
    EndianReader,
};

mod ifd;
mod stream;

/// The representation of a TIFF decoder
///
/// Currently does not support decoding of interlaced images
#[derive(Debug)]
pub struct TIFFDecoder<R> where R: Read + Seek {
    reader: R,
    byte_order: ByteOrder,
    next_ifd: Option<u32>,
    ifd: Option<Directory>,
    width: u32,
    height: u32,
    bits_per_sample: Vec<u8>,
    samples: u8,
}

impl<R: Read + Seek> TIFFDecoder<R> {
    /// Create a new decoder that decodes from the stream ```r```
    pub fn new(r: R) -> ImageResult<TIFFDecoder<R>> {
        TIFFDecoder {
            reader: r,
            byte_order: ByteOrder::LittleEndian,
            next_ifd: None,
            ifd: None,
            width: 0,
            height: 0,
            bits_per_sample: vec![1],
            samples: 1,
        }.init()
    }

    fn read_header(&mut self) -> ImageResult<()> {
        Ok(())
    }

    /// Initializes the decoder.
    pub fn init(self) -> ImageResult<TIFFDecoder<R>> {
        self.next_image()
    }

    /// Reads in the next image.
    /// If there is no further image in the TIFF file a format error is return.
    /// To determine whether there are more images call `TIFFDecoder::more_images` instead.
    pub fn next_image(mut self) -> ImageResult<TIFFDecoder<R>> {
        try!(self.read_header());
        Ok(self)
    }
}

impl<R: Read + Seek> ImageDecoder for TIFFDecoder<R> {
    fn dimensions(&mut self) -> ImageResult<(u32, u32)> {
        Ok((self.width, self.height))

    }

    fn read_image(&mut self) -> ImageResult<DecodingResult> {
        let buffer_size =
            self.width  as usize
            * self.height as usize
            * self.bits_per_sample.iter().count();
        let mut result = match (self.bits_per_sample.iter()
                                               .map(|&x| x)
                                               .max()
                                               .unwrap_or(8) as f32/8.0).ceil() as u8 {
            n if n <= 8 => DecodingResult::U8(Vec::with_capacity(buffer_size)),
            n if n <= 16 => DecodingResult::U16(Vec::with_capacity(buffer_size)),
            n => return Err(
                ImageError::UnsupportedError(
                    format!("{} bits per channel not supported", n)
                )
            )
        };
        Ok(result)
    }
}
