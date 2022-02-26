use crate::binding;

fn get_decompressed_size(a1: &mut [u64; 17], file_buf: &mut Vec<u8>, header_size: usize) -> usize {
    unsafe {
        binding::get_decompressed_size(
            a1 as *mut _ as *mut i64,
            file_buf.as_mut_ptr(),
            -1,
            file_buf.len() as i64,
            0,
            header_size as i64,
        ) as usize
    }
}

fn decompress_rpak(
    a1: &mut [u64; 17],
    file_buf: &mut Vec<u8>,
    decompressed_size: usize,
) -> Option<Vec<u8>> {
    unsafe {
        let mut out = vec![0u8; decompressed_size];

        a1[1] = out.as_mut_ptr() as u64;
        a1[3] = u64::MAX;

        if binding::decompress_rpak(
            a1 as *mut _ as *mut i64,
            file_buf.len() as u64,
            decompressed_size as u64,
        ) == 1
        {
            Some(out)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidDecompressedSize,
    DecompressionError,
}

/// Decompresses RPak file, if invalid - `rpak::decomp::Error` is returned with the type.
pub fn decompress(
    file_buf: &mut Vec<u8>,
    decompressed_size_expected: usize,
    header_size: usize,
) -> std::result::Result<Vec<u8>, Error> {
    let mut a1 = [0u64; 17];
    let decompressed_size = get_decompressed_size(&mut a1, file_buf, header_size);
    unsafe {
        let state = std::mem::transmute::<_, binding::rpak_decomp_state>(a1);
        eprintln!("{:#?}", state);
    }
    if decompressed_size_expected == decompressed_size {
        match decompress_rpak(&mut a1, file_buf, decompressed_size) {
            Some(ret) => {
                unsafe {
                    let state = std::mem::transmute::<_, binding::rpak_decomp_state>(a1);
                    eprintln!("{:#?}", state);
                }
                Ok(ret)
            }
            None => Err(Error::DecompressionError),
        }
    } else {
        Err(Error::InvalidDecompressedSize)
    }
}
