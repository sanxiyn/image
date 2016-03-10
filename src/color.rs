use std::ops::{ Index, IndexMut };
use std::mem;

use buffer::Pixel;
use traits::Primitive;

/// An enumeration over supported color types and their bit depths
#[derive(Copy, PartialEq, Eq, Debug, Clone)]
pub enum ColorType {
    /// Pixel is grayscale
    Gray(u8),

    /// Pixel contains R, G and B channels
    RGB(u8),

    /// Pixel is an index into a color palette
    Palette(u8),

    /// Pixel is grayscale with an alpha channel
    GrayA(u8),

    /// Pixel is RGB with an alpha channel
    RGBA(u8)
}

macro_rules! define_colors {
    {$(
        $ident:ident,
        $channels: expr,
        $alphas: expr,
        $interpretation: expr,
        $color_type: ident,
        #[$doc:meta];
    )*} => {

$( // START Structure definitions

#[$doc]
#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct $ident<T: Primitive> { pub data: [T; $channels] }
#[allow(non_snake_case, missing_docs)]
pub fn $ident<T: Primitive>(data: [T; $channels]) -> $ident<T> {
    $ident {
        data: data
    }
}

impl<T: Primitive + 'static> Pixel for $ident<T> {

    type Subpixel = T;

    fn channel_count() -> u8 {
        $channels
    }
    fn color_model() -> &'static str {
        $interpretation
    }
    fn color_type() -> ColorType {
        ColorType::$color_type(mem::size_of::<T>() as u8 * 8)
    }
    #[inline(always)]
    fn channels(&self) -> &[T] {
        &self.data
    }
    #[inline(always)]
    fn channels_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    fn from_channels(a: T, b: T, c: T, d: T,) -> $ident<T> {
        *<$ident<T> as Pixel>::from_slice(&[a, b, c, d][..$channels])
    }

    fn from_slice<'a>(slice: &'a [T]) -> &'a $ident<T> {
        assert_eq!(slice.len(), $channels);
        unsafe { mem::transmute(slice.as_ptr()) }
    }
    fn from_slice_mut<'a>(slice: &'a mut [T]) -> &'a mut $ident<T> {
        assert_eq!(slice.len(), $channels);
        unsafe { mem::transmute(slice.as_ptr()) }
    }
}

impl<T: Primitive> Index<usize> for $ident<T> {
    type Output = T;
    #[inline(always)]
    fn index<'a>(&'a self, _index: usize) -> &'a T {
        &self.data[_index]
    }
}

impl<T: Primitive> IndexMut<usize> for $ident<T> {
    #[inline(always)]
    fn index_mut<'a>(&'a mut self, _index: usize) -> &'a mut T {
        &mut self.data[_index]
    }
}

)* // END Structure definitions

    }
}

define_colors! {
    Rgb, 3, 0, "RGB", RGB, #[doc = "RGB colors"];
    Luma, 1, 0, "Y", Gray, #[doc = "Grayscale colors"];
    Rgba, 4, 1, "RGBA", RGBA, #[doc = "RGB colors + alpha channel"];
    LumaA, 2, 1, "YA", GrayA, #[doc = "Grayscale colors + alpha channel"];
}


/// Provides color conversions for the different pixel types.
pub trait FromColor<Other> {
    /// Changes `self` to represent `Other` in the color space of `Self`
    fn from_color(&mut self, &Other);
}

// Self->Self: just copy
impl<A: Copy> FromColor<A> for A {
    fn from_color(&mut self, other: &A) {
        *self = *other;
    }
}
