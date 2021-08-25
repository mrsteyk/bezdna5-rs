use byteorder::ReadBytesExt;

pub unsafe fn str_from_u8_nul_utf8_checked(utf8_src: &[u8]) -> &str {
    let nul_range_end = utf8_src.iter().position(|&c| c == b'\0').unwrap();
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

pub fn string_from_buf<R: ReadBytesExt>(cursor: &mut R) -> String {
    let mut buf = [0u8; 1024];
    match cursor.read_exact(&mut buf) {
        Ok(_) => unsafe { str_from_u8_nul_utf8_checked(buf.as_ref()).to_owned() },
        Err(v) => {
            if v.kind() == std::io::ErrorKind::UnexpectedEof {
                unsafe { str_from_u8_nul_utf8_checked(buf.as_ref()).to_owned() }
            } else {
                panic!("{:?}", v)
            }
        }
    }
}

pub fn string_from_buf_slow<R: ReadBytesExt>(cursor: &mut R) -> String {
    let mut buf = Vec::<u8>::with_capacity(1024);
    loop {
        let tmp = cursor.read_u8();
        match tmp {
            Ok(v) => {
                if v == 0 {
                    break;
                } else {
                    buf.push(v)
                }
            }
            Err(v) => {
                if v.kind() == std::io::ErrorKind::UnexpectedEof {
                    break;
                } else {
                    panic!("{:?}", v)
                }
            }
        }
    }
    unsafe { ::std::str::from_utf8_unchecked(&buf[0..buf.len()]).to_owned() }
}
