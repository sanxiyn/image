//! Function for reading TIFF tags

use std::collections::{HashMap};

macro_rules! tags {
    {$(
        $tag:ident
        $val:expr;
    )*} => {

        /// TIFF tag
        #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
        pub enum Tag {
            $($tag,)*
            Unknown(u16)
        }
    }
}

// Note: These tags appear in the order they are mentioned in the TIFF reference
tags!{
    // Baseline tags:
    Artist 315; // TODO add support
}

enum_from_primitive! {
#[derive(Clone, Copy, Debug)]
pub enum Type {
    BYTE = 1,
    ASCII = 2,
    SHORT = 3,
    LONG = 4,
    RATIONAL = 5,
}
}


pub struct Entry {
    type_: Type,
    count: u32,
    offset: [u8; 4],
}

impl ::std::fmt::Debug for Entry {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        fmt.write_str(&format!("Entry {{ type_: {:?}, count: {:?}, offset: {:?} }}",
            self.type_,
            self.count,
            &self.offset
        ))
    }
}

/// Type representing an Image File Directory
pub type Directory = HashMap<Tag, Entry>;
