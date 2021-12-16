#![allow(non_snake_case)]
#![allow(deref_nullptr)]

extern "C" {
    pub fn decompress_rpak(
        a1: *mut ::std::os::raw::c_longlong,
        a2: ::std::os::raw::c_ulonglong,
        a3: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_char;
}
extern "C" {
    pub fn get_decompressed_size(
        aparams: *mut ::std::os::raw::c_longlong,
        file_buf: *mut u8,
        some_magic_shit: ::std::os::raw::c_longlong,
        file_size: ::std::os::raw::c_longlong,
        off_without_header_qm: ::std::os::raw::c_longlong,
        header_size: ::std::os::raw::c_longlong,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn hash_string(a1: *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_ulonglong;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rpak_decomp_state {
    pub input_buf: u64,
    pub out: u64,
    pub mask: u64,
    pub out_mask: u64,
    pub file_len_total: u64,
    pub decompressed_size: u64,
    pub inv_mask_in: u64,
    pub inv_mask_out: u64,
    pub header_skip_bytes_bs: u32, // some bs alignment or something, idk
    pub dword44: u32,
    pub input_byte_pos: u64,
    pub decompressed_position: u64,
    pub len_needed: u64, // or a better name would be stream_file_len/stream_len_needed?
    pub byte: u64, // cursor, prepend stream_?
    pub byte_bit_offset: u32, // cursor bit offset, MUST BE < 8, prepend stream_?
    pub dword6C: u32, // unknown bs, either 0 or 1, controls some LUT_200 offset (0/256)
    pub qword70: u64, // initially (inv_mask_in-6), then can be (inv_mask_in + (inv_mask_in & -input_byte_pos) + 1)
    pub stream_compressed_size: u64,
    pub stream_decompressed_size: u64,
}
#[test]
fn bindgen_test_layout_rpak_decomp_state() {
    assert_eq!(
        ::std::mem::size_of::<rpak_decomp_state>(),
        136usize,
        concat!("Size of: ", stringify!(rpak_decomp_state))
    );
    assert_eq!(
        ::std::mem::align_of::<rpak_decomp_state>(),
        8usize,
        concat!("Alignment of ", stringify!(rpak_decomp_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpak_decomp_state>())).input_buf as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rpak_decomp_state),
            "::",
            stringify!(input_buf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpak_decomp_state>())).out as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rpak_decomp_state),
            "::",
            stringify!(out)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpak_decomp_state>())).mask as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rpak_decomp_state),
            "::",
            stringify!(mask)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpak_decomp_state>())).out_mask as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rpak_decomp_state),
            "::",
            stringify!(out_mask)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpak_decomp_state>())).file_len_total as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rpak_decomp_state),
            "::",
            stringify!(qword20)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rpak_decomp_state>())).decompressed_size as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rpak_decomp_state),
            "::",
            stringify!(decompressed_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpak_decomp_state>())).inv_mask_in as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rpak_decomp_state),
            "::",
            stringify!(qword30)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpak_decomp_state>())).inv_mask_out as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rpak_decomp_state),
            "::",
            stringify!(qword38)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpak_decomp_state>())).header_skip_bytes_bs as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rpak_decomp_state),
            "::",
            stringify!(unsigned40)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpak_decomp_state>())).dword44 as *const _ as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(rpak_decomp_state),
            "::",
            stringify!(dword44)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpak_decomp_state>())).input_byte_pos as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(rpak_decomp_state),
            "::",
            stringify!(qword48)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpak_decomp_state>())).decompressed_position as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(rpak_decomp_state),
            "::",
            stringify!(header_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpak_decomp_state>())).len_needed as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(rpak_decomp_state),
            "::",
            stringify!(len_needed)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpak_decomp_state>())).byte as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(rpak_decomp_state),
            "::",
            stringify!(qword60)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpak_decomp_state>())).byte_bit_offset as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(rpak_decomp_state),
            "::",
            stringify!(dword68)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpak_decomp_state>())).dword6C as *const _ as usize },
        108usize,
        concat!(
            "Offset of field: ",
            stringify!(rpak_decomp_state),
            "::",
            stringify!(dword6C)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpak_decomp_state>())).qword70 as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(rpak_decomp_state),
            "::",
            stringify!(qword70)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpak_decomp_state>())).stream_compressed_size as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(rpak_decomp_state),
            "::",
            stringify!(qword78)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rpak_decomp_state>())).stream_decompressed_size as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(rpak_decomp_state),
            "::",
            stringify!(qword80)
        )
    );
}