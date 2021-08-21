use std::io::Read;

pub unsafe fn str_from_u8_nul_utf8_checked(utf8_src: &[u8]) -> &str {
    let nul_range_end = utf8_src
        .iter()
        .position(|&c| c == b'\0')
        .unwrap_or_else(|| todo!()); // default to TODO if no `\0` present
    ::std::str::from_utf8_unchecked(&utf8_src[0..nul_range_end])
}

// used for ext to string
pub unsafe fn str_from_u8_nul_utf8_unchecked(utf8_src: &[u8]) -> &str {
    let nul_range_end = utf8_src
        .iter()
        .position(|&c| c == b'\0')
        .unwrap_or(utf8_src.len()); // default to length if no `\0` present
    ::std::str::from_utf8_unchecked(&utf8_src[0..nul_range_end])
}

pub fn string_from_buf<R: Read>(cursor: &mut R) -> String {
    let mut buf = [0u8; 1024];
    match cursor.read_exact(&mut buf) {
        Ok(_) => unsafe { str_from_u8_nul_utf8_checked(buf.as_ref()).to_owned() },
        _ => panic!(),
    }
}
