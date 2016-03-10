use std::io::Read;
use std::default::Default;

use image;
use image::ImageResult;
use image::ImageDecoder;

use super::vp8::Frame;



/// A Representation of a Webp Image format decoder.
pub struct WebpDecoder<R> {
    r: R,
    frame: Frame,
    have_frame: bool,
    decoded_rows: u32,
}

impl<R: Read> WebpDecoder<R> {
    /// Create a new WebpDecoder from the Reader ```r```.
    /// This function takes ownership of the Reader.
    pub fn new(r: R) -> WebpDecoder<R> {
        let f: Frame = Default::default();

        WebpDecoder {
            r: r,
            have_frame: false,
            frame: f,
            decoded_rows: 0
        }
    }

    fn read_metadata(&mut self) -> ImageResult<()> {
        Ok(())
    }
}

impl<R: Read> ImageDecoder for WebpDecoder<R> {
    fn dimensions(&mut self) -> ImageResult<(u32, u32)> {
        let _ = try!(self.read_metadata());

        Ok((self.frame.width as u32, self.frame.height as u32))
    }

    fn read_image(&mut self) -> ImageResult<image::DecodingResult> {
        let _ = try!(self.read_metadata());

        Ok(image::DecodingResult::U8(self.frame.ybuf.clone()))
    }
}
