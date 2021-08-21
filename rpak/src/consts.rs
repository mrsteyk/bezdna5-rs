use std::convert::From;

/// All RPak versions known to date, alongside invalid value for """error-handling"""
#[derive(Debug)]
pub enum RPakVersion {
    Invalid = 0,
    TF2 = 7,
    APEX = 8,
}

#[derive(Debug)]
pub enum RPakError {
    InvalidMagic(u32),
    InvalidVersion(u16),
    IOError(std::io::Error),
    DecompError(crate::decomp::Error),
    Shiz(String),
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

/// Header size for TF2(ver 7) based games
pub const HEADER_SIZE_TF2: usize = 88;
/// Header size for Apex(ver 8) based games
pub const HEADER_SIZE_APEX: usize = 0x80;
