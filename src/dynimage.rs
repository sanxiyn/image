use std::io;
use std::io::{Read, Write, Seek, BufReader};

#[cfg(feature = "webp")]
use webp;
#[cfg(feature = "tiff")]
use tiff;

use buffer::{GrayImage, GrayAlphaImage, RgbImage, RgbaImage};
use image;
use image:: {
    GenericImage,
    ImageDecoder,
    ImageResult,
    ImageFormat,
};

/// A Dynamic Image
#[derive(Clone)]
pub enum DynamicImage {
    /// Each pixel in this image is 8-bit Luma
    ImageLuma8(GrayImage),

    /// Each pixel in this image is 8-bit Luma with alpha
    ImageLumaA8(GrayAlphaImage),

    /// Each pixel in this image is 8-bit Rgb
    ImageRgb8(RgbImage),

    /// Each pixel in this image is 8-bit Rgb with alpha
    ImageRgba8(RgbaImage),
}


/// Decodes an image and stores it into a dynamic image
pub fn decoder_to_image<I: ImageDecoder>(codec: I) -> ImageResult<DynamicImage> {
    let mut codec = codec;

    let buf    = try!(codec.read_image());
    let (w, h) = try!(codec.dimensions());

    Err(image::ImageError::DimensionError)
}

/// Create a new image from a Reader
pub fn load<R: Read+Seek>(r: R, format: ImageFormat) -> ImageResult<DynamicImage> {
    match format {
        #[cfg(feature = "webp")]
        image::ImageFormat::WEBP => decoder_to_image(webp::WebpDecoder::new(BufReader::new(r))),
        #[cfg(feature = "tiff")]
        image::ImageFormat::TIFF => decoder_to_image(try!(tiff::TIFFDecoder::new(r))),
        _ => Err(image::ImageError::UnsupportedError(format!("A decoder for {:?} is not available.", format))),
    }
}

static MAGIC_BYTES: [(&'static [u8], ImageFormat); 9] = [
    (b"\x89PNG\r\n\x1a\n", ImageFormat::PNG),
    (&[0xff, 0xd8, 0xff], ImageFormat::JPEG),
    (b"GIF89a", ImageFormat::GIF),
    (b"GIF87a", ImageFormat::GIF),
    (b"WEBP", ImageFormat::WEBP),
    (b"MM.*", ImageFormat::TIFF),
    (b"II*.", ImageFormat::TIFF),
    (b"BM", ImageFormat::BMP),
    (&[0, 0, 1, 0], ImageFormat::ICO),
];

/// Create a new image from a byte slice
///
/// Makes an educated guess about the image format.
/// TGA is not supported by this function.
pub fn load_from_memory(buffer: &[u8]) -> ImageResult<DynamicImage> {
    for &(signature, format) in MAGIC_BYTES.iter() {
        if buffer.starts_with(signature) {
            return load_from_memory_with_format(buffer, format)
        }
    }
    Err(image::ImageError::UnsupportedError(
        "Unsupported image format".to_string())
    )
}


/// Create a new image from a byte slice
#[inline(always)]
pub fn load_from_memory_with_format(buf: &[u8], format: ImageFormat) -> ImageResult<DynamicImage> {
    let b = io::Cursor::new(buf);
    load(b, format)
}
