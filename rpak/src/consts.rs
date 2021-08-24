use std::convert::From;

/// All RPak versions known to date, alongside invalid value for """error-handling"""
#[derive(Debug)]
pub enum RPakVersion {
    Invalid = 0,
    APEX = 8,
}

#[derive(Debug)]
pub enum RPakError {
    InvalidMagic(u32),
    InvalidVersion(u16),
    IOError(std::io::Error),
    DecompError(crate::decomp::Error),
    Shiz(String),

    FileTypeParseError((&'static str, Box<RPakError>)),
}

impl From<std::io::Error> for RPakError {
    fn from(item: std::io::Error) -> Self {
        Self::IOError(item)
    }
}

impl From<crate::decomp::Error> for RPakError {
    fn from(item: crate::decomp::Error) -> Self {
        Self::DecompError(item)
    }
}

/// Header size for Apex(ver 8) based games
pub const HEADER_SIZE_APEX: usize = 0x80;
