use crate::binding::hash_string;

/// Internal hashing function used for the GUID generation
pub fn hash(string: String) -> u64 {
    unsafe {
        let tmp = std::ffi::CString::new(string).unwrap();
        hash_string(tmp.as_ptr() as *mut u32)
    }
}

#[cfg(test)]
mod tests {
    use super::hash;

    #[test]
    fn it_works() {
        assert_eq!(hash("suck_my_dick".to_owned()), 0x9a129792efd9bd12);
    }
}
