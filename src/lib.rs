//! This crate provides native rust implementations of
//! image encoders and decoders and basic image manipulation
//! functions.

#![warn(missing_docs)]
#![warn(unused_qualifications)]
#![deny(missing_copy_implementations)]
#![cfg_attr(test, feature(test))]

extern crate byteorder;
extern crate num;
#[macro_use]
extern crate enum_primitive;
#[cfg(test)]
extern crate test;

pub use color::ColorType::{
    self,
    Gray,
    RGB,
    Palette,
    GrayA,
    RGBA
};

pub use color::{
    Luma,
    LumaA,
    Rgb,
    Rgba
};

pub use image::{
    ImageDecoder,
    ImageError,
    ImageResult,
    SubImage,
    GenericImage,
    // Iterators
    Pixels,
    MutPixels
};

pub use image::ImageFormat::{
    self,
    PNG,
    JPEG,
    GIF,
    WEBP,
    PPM,
    BMP,
    ICO
};

pub use buffer::{
    Pixel,
    ConvertBuffer,
    // Image types
    ImageBuffer,
    RgbImage,
    RgbaImage,
    GrayImage,
    GrayAlphaImage
};

// Traits
pub use traits::Primitive;

// Opening and loading images
pub use dynimage::{
    load,
    load_from_memory,
    load_from_memory_with_format,
};

pub use dynimage::DynamicImage::{
    self,
    ImageRgb8,
    ImageRgba8,
    ImageLuma8,
    ImageLumaA8
};

// Math utils
pub mod math;

// Image codecs
#[cfg(feature = "webp")]
pub mod webp;
#[cfg(feature = "tiff")]
pub mod tiff;

mod image;
mod utils;
mod dynimage;
mod color;
mod buffer;
mod traits;
