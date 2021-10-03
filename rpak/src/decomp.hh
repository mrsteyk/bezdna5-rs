#pragma once

using uint8_t = unsigned char;

#ifndef _MSC_VER
using __int64 = int64_t;
#endif

extern "C" {
    char decompress_rpak(__int64* a1, unsigned __int64 a2, unsigned __int64 a3);
    // next line has incorrect definition but who the fuck cares...
    __int64 get_decompressed_size(__int64* aparams, uint8_t* file_buf, __int64 some_magic_shit, __int64 file_size, __int64 off_without_header_qm, __int64 header_size);
    unsigned __int64 hash_string(unsigned int* a1);
}