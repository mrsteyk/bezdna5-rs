#include <cstdint>

#define _QWORD uint64_t
#define _DWORD uint32_t

struct __declspec(align(8)) rpak_decomp_state
{
  _QWORD input_buf;
  _QWORD out;
  _QWORD mask;
  _QWORD out_mask;
  _QWORD qword20;
  _QWORD decompressed_size;
  _QWORD qword30;
  _QWORD qword38;
  unsigned int unsigned40;
  _DWORD dword44;
  _QWORD qword48;
  _QWORD header_size;
  _QWORD len_needed;
  _QWORD qword60;
  _DWORD dword68;
  _DWORD dword6C;
  _QWORD qword70;
  _QWORD qword78;
  _QWORD qword80;
};