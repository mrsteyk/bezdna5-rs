#include <array>
#include <cstring>
#include <cstdint>

using _DWORD = uint32_t;

#ifndef _MSC_VER

using __int64 = int64_t;
using __int16 = int16_t;

#include <cassert>

static inline
unsigned int __native_popcnt(unsigned int x)
{ 
    x -=  ((x >> 1) & 0x55555555U);
    x  = (((x >> 2) & 0x33333333U) + (x & 0x33333333U));
    x  = (((x >> 4) + x) & 0x0F0F0F0FU);
    x +=   (x >> 8);
    x +=   (x >> 16);
    x  = x & 0x0000003FU;
    assert(x >= 0 && x <= 32);
    return x;
}

static inline
unsigned int __native_clz(unsigned int x)
{
    x |= (x >> 1);
    x |= (x >> 2);
    x |= (x >> 4);
    x |= (x >> 8);
    x |= (x >> 16);
    return (32 - __native_popcnt(x));
}

static inline
unsigned char
__builtin_BitScanReverse(unsigned long * index, unsigned long mask)
{
    assert(index != nullptr);
    unsigned int leading_zeros;
#if defined(__has_builtin_clz)
    leading_zeros = __builtin_clz((unsigned int)mask);
#else
    leading_zeros = __native_clz((unsigned int)mask);
#endif
    *index = 32 - leading_zeros;
    return (unsigned char)(mask != 0);
}

#define _BitScanReverse __builtin_BitScanReverse
#endif

extern "C" uint64_t hash_string(_DWORD* a1)
{
    _DWORD* v1; // r8
    uint64_t v2; // r10
    int v3; // er11
    unsigned int v4; // er9
    unsigned int i; // edx
    __int64 v6; // rcx
    int v7; // er9
    int v8; // edx
    int v9; // eax
    unsigned int v10; // er8
    int v12; // ecx

    v1 = a1;
    v2 = 0LL;
    v3 = 0;
    v4 = (*a1 - 45 * ((~(*a1 ^ 0x5C5C5C5Cu) >> 7) & (((*a1 ^ 0x5C5C5C5Cu) - 0x1010101) >> 7) & 0x1010101)) & 0xDFDFDFDF;
    for (i = ~*a1 & (*a1 - 0x1010101) & 0x80808080; !i; i = v8 & 0x80808080)
    {
        v6 = v4;
        v7 = v1[1];
        ++v1;
        v3 += 4;
        v2 = ((((uint64_t)(0xFB8C4D96501LL * v6) >> 24) + 0x633D5F1 * v2) >> 61) ^ (((uint64_t)(0xFB8C4D96501LL * v6) >> 24)
            + 0x633D5F1 * v2);
        v8 = ~v7 & (v7 - 0x1010101);
        v4 = (v7 - 45 * ((~(v7 ^ 0x5C5C5C5Cu) >> 7) & (((v7 ^ 0x5C5C5C5Cu) - 0x1010101) >> 7) & 0x1010101)) & 0xDFDFDFDF;
    }
    v9 = -1;
    v10 = (i & -(signed)i) - 1;
    if (_BitScanReverse((unsigned long*)&v12, v10))
        v9 = v12;
    return 0x633D5F1 * v2
        + ((0xFB8C4D96501LL * (uint64_t)(v4 & v10)) >> 24)
        - 0xAE502812AA7333LL * (unsigned int)(v3 + v9 / 8);
}

// LUTs with length checks cuz modern C++ and we don't want to shit ourselves accidentaly from pasting from IDA
std::array<uint8_t, 512> LUT_0{
    4, 254, 252, 8, 4, 239, 17, 249, 4, 253, 252, 7, 4, 5, 255, 244, 4, 254, 252, 16, 4, 239, 17, 246, 4, 253, 252, 251, 4, 6, 255, 11, 4, 254, 252, 8, 4, 239, 17, 248, 4, 253, 252, 12, 4, 5, 255, 247, 4, 254, 252, 16, 4, 239, 17, 245, 4, 253, 252, 250, 4, 6, 255, 243, 4, 254, 252, 8, 4, 239, 17, 249, 4, 253, 252, 7, 4, 5, 255, 244, 4, 254, 252, 16, 4, 239, 17, 246, 4, 253, 252, 251, 4, 6, 255, 14, 4, 254, 252, 8, 4, 239, 17, 248, 4, 253, 252, 12, 4, 5, 255, 9, 4, 254, 252, 16, 4, 239, 17, 245, 4, 253, 252, 250, 4, 6, 255, 241, 4, 254, 252, 8, 4, 239, 17, 249, 4, 253, 252, 7, 4, 5, 255, 244, 4, 254, 252, 16, 4, 239, 17, 246, 4, 253, 252, 251, 4, 6, 255, 13, 4, 254, 252, 8, 4, 239, 17, 248, 4, 253, 252, 12, 4, 5, 255, 247, 4, 254, 252, 16, 4, 239, 17, 245, 4, 253, 252, 250, 4, 6, 255, 242, 4, 254, 252, 8, 4, 239, 17, 249, 4, 253, 252, 7, 4, 5, 255, 244, 4, 254, 252, 16, 4, 239, 17, 246, 4, 253, 252, 251, 4, 6, 255, 15, 4, 254, 252, 8, 4, 239, 17, 248, 4, 253, 252, 12, 4, 5, 255, 10, 4, 254, 252, 16, 4, 239, 17, 245, 4, 253, 252, 250, 4, 6, 255, 240, 4, 5, 4, 6, 4, 5, 4, 7, 4, 5, 4, 6, 4, 5, 4, 17, 4, 5, 4, 6, 4, 5, 4, 8, 4, 5, 4, 6, 4, 5, 4, 12, 4, 5, 4, 6, 4, 5, 4, 7, 4, 5, 4, 6, 4, 5, 4, 9, 4, 5, 4, 6, 4, 5, 4, 8, 4, 5, 4, 6, 4, 5, 4, 14, 4, 5, 4, 6, 4, 5, 4, 7, 4, 5, 4, 6, 4, 5, 4, 17, 4, 5, 4, 6, 4, 5, 4, 8, 4, 5, 4, 6, 4, 5, 4, 11, 4, 5, 4, 6, 4, 5, 4, 7, 4, 5, 4, 6, 4, 5, 4, 10, 4, 5, 4, 6, 4, 5, 4, 8, 4, 5, 4, 6, 4, 5, 4, 16, 4, 5, 4, 6, 4, 5, 4, 7, 4, 5, 4, 6, 4, 5, 4, 17, 4, 5, 4, 6, 4, 5, 4, 8, 4, 5, 4, 6, 4, 5, 4, 12, 4, 5, 4, 6, 4, 5, 4, 7, 4, 5, 4, 6, 4, 5, 4, 9, 4, 5, 4, 6, 4, 5, 4, 8, 4, 5, 4, 6, 4, 5, 4, 15, 4, 5, 4, 6, 4, 5, 4, 7, 4, 5, 4, 6, 4, 5, 4, 17, 4, 5, 4, 6, 4, 5, 4, 8, 4, 5, 4, 6, 4, 5, 4, 13, 4, 5, 4, 6, 4, 5, 4, 7, 4, 5, 4, 6, 4, 5, 4, 10, 4, 5, 4, 6, 4, 5, 4, 8, 4, 5, 4, 6, 4, 5, 4, 255
};
std::array<uint8_t, 512> LUT_200{
    2, 4, 3, 5, 2, 4, 4, 6, 2, 4, 3, 6, 2, 5, 4, 6, 2, 4, 3, 5, 2, 4, 4, 6, 2, 4, 3, 6, 2, 5, 4, 8, 2, 4, 3, 5, 2, 4, 4, 6, 2, 4, 3, 6, 2, 5, 4, 7, 2, 4, 3, 5, 2, 4, 4, 6, 2, 4, 3, 6, 2, 5, 4, 8, 2, 4, 3, 5, 2, 4, 4, 6, 2, 4, 3, 6, 2, 5, 4, 6, 2, 4, 3, 5, 2, 4, 4, 6, 2, 4, 3, 6, 2, 5, 4, 8, 2, 4, 3, 5, 2, 4, 4, 6, 2, 4, 3, 6, 2, 5, 4, 8, 2, 4, 3, 5, 2, 4, 4, 6, 2, 4, 3, 6, 2, 5, 4, 8, 2, 4, 3, 5, 2, 4, 4, 6, 2, 4, 3, 6, 2, 5, 4, 6, 2, 4, 3, 5, 2, 4, 4, 6, 2, 4, 3, 6, 2, 5, 4, 8, 2, 4, 3, 5, 2, 4, 4, 6, 2, 4, 3, 6, 2, 5, 4, 7, 2, 4, 3, 5, 2, 4, 4, 6, 2, 4, 3, 6, 2, 5, 4, 8, 2, 4, 3, 5, 2, 4, 4, 6, 2, 4, 3, 6, 2, 5, 4, 6, 2, 4, 3, 5, 2, 4, 4, 6, 2, 4, 3, 6, 2, 5, 4, 8, 2, 4, 3, 5, 2, 4, 4, 6, 2, 4, 3, 6, 2, 5, 4, 8, 2, 4, 3, 5, 2, 4, 4, 6, 2, 4, 3, 6, 2, 5, 4, 8, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3, 1, 2, 1, 6, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3, 1, 2, 1, 7, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3, 1, 2, 1, 7, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3, 1, 2, 1, 8, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3, 1, 2, 1, 6, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3, 1, 2, 1, 8, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3, 1, 2, 1, 7, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3, 1, 2, 1, 8, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3, 1, 2, 1, 6, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3, 1, 2, 1, 7, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3, 1, 2, 1, 7, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3, 1, 2, 1, 8, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3, 1, 2, 1, 6, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3, 1, 2, 1, 8, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3, 1, 2, 1, 7, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3, 1, 2, 1, 8
};

std::array<uint8_t, 0x40> LUT_400{
    0, 8, 0, 4, 0, 8, 0, 6, 0, 8, 0, 1, 0, 8, 0, 11, 0, 8, 0, 12, 0, 8, 0, 9, 0, 8, 0, 3, 0, 8, 0, 14, 0, 8, 0, 4, 0, 8, 0, 7, 0, 8, 0, 2, 0, 8, 0, 13, 0, 8, 0, 12, 0, 8, 0, 10, 0, 8, 0, 5, 0, 8, 0, 15
};
std::array<uint8_t, 0x40> LUT_440{
    1, 2, 1, 5, 1, 2, 1, 6, 1, 2, 1, 6, 1, 2, 1, 6, 1, 2, 1, 5, 1, 2, 1, 6, 1, 2, 1, 6, 1, 2, 1, 6, 1, 2, 1, 5, 1, 2, 1, 6, 1, 2, 1, 6, 1, 2, 1, 6, 1, 2, 1, 5, 1, 2, 1, 6, 1, 2, 1, 6, 1, 2, 1, 6
};

std::array<uint32_t, 16> LUT_480{
    74, 106, 138, 170, 202, 234, 266, 298, 330, 362, 394, 426, 938, 1450, 9642, 140714
};

std::array<uint8_t, 16> LUT_4C0{
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 9, 9, 13, 17, 21
};

std::array<uint8_t, 8> LUT_4D0{
    0, 0, 2, 4, 6, 8, 10, 42
};
std::array<uint8_t, 8> LUT_4D8{
    0, 1, 1, 1, 1, 1, 5, 5
};

std::array<uint8_t, 32> LUT_4E0{
    17, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
};

#define LOBYTE(x)   (*((_BYTE*)&(x)))   // low byte
#define LOWORD(x)   (*((_WORD*)&(x)))   // low word
#define LODWORD(x)  (*((_DWORD*)&(x)))  // low dword
#define HIBYTE(x)   (*((_BYTE*)&(x)+1))
#define HIWORD(x)   (*((_WORD*)&(x)+1))
#define HIDWORD(x)  (*((_DWORD*)&(x)+1))
#define BYTEn(x, n)   (*((_BYTE*)&(x)+n))
#define WORDn(x, n)   (*((_WORD*)&(x)+n))
#define BYTE1(x)   BYTEn(x,  1)         // byte 1 (counting from 0)
#define BYTE2(x)   BYTEn(x,  2)
#define BYTE3(x)   BYTEn(x,  3)
#define BYTE4(x)   BYTEn(x,  4)
#define BYTE5(x)   BYTEn(x,  5)
#define BYTE6(x)   BYTEn(x,  6)
#define BYTE7(x)   BYTEn(x,  7)
#define BYTE8(x)   BYTEn(x,  8)
#define BYTE9(x)   BYTEn(x,  9)
#define BYTE10(x)  BYTEn(x, 10)
#define BYTE11(x)  BYTEn(x, 11)
#define BYTE12(x)  BYTEn(x, 12)
#define BYTE13(x)  BYTEn(x, 13)
#define BYTE14(x)  BYTEn(x, 14)
#define BYTE15(x)  BYTEn(x, 15)
#define WORD1(x)   WORDn(x,  1)
#define WORD2(x)   WORDn(x,  2)         // third word of the object, unsigned
#define WORD3(x)   WORDn(x,  3)
#define WORD4(x)   WORDn(x,  4)
#define WORD5(x)   WORDn(x,  5)
#define WORD6(x)   WORDn(x,  6)
#define WORD7(x)   WORDn(x,  7)

using _QWORD = uint64_t;
using _DWORD = uint32_t;
using _WORD = uint16_t;
using _BYTE = uint8_t;

extern "C" char decompress_rpak(__int64* a1, uint64_t a2, uint64_t a3)
{
    uint64_t v3; // r15
    char result; // al
    __int64 v6; // r14
    unsigned int v7; // er10
    uint64_t v8; // rbx
    int v9; // ebp
    uint64_t v10; // rsi
    __int64 v11; // r12
    uint64_t v12; // r13
    uint64_t v13; // rdi
    int v14; // ecx
    uint64_t v15; // rax
    unsigned int v16; // ebp
    uint64_t v17; // rsi
    char v18; // cl
    unsigned int v19; // er12
    int v20; // edi
    unsigned int v21; // ecx
    __int64 v22; // r9
    int v23; // er10
    unsigned int v24; // er8
    int v25; // edi
    __int64 v26; // rax
    unsigned int v27; // er15
    __int64 v28; // rdi
    _QWORD* v29; // rax
    char* v30; // rdi
    __int64 v31; // rcx
    uint64_t v32; // r8
    __int64 v33; // rcx
    uint64_t v34; // rdi
    uint64_t v35; // rax
    uint64_t v36; // rax
    __int64 v37; // rax
    __int64 v38; // rdx
    uint64_t v39; // rcx
    uint64_t v40; // rdx
    uint64_t v41; // rsi
    uint64_t v42; // rax
    char v43; // cl
    uint64_t v44; // rsi
    __int64 v45; // rcx
    uint64_t v46; // r8
    int v47; // er10
    uint8_t v48; // r9
    uint64_t v49; // rcx
    __int64 v50; // rcx
    __int64 v51; // r8
    __int64 v52; // rdi
    __int64 v53; // rdx
    __int64 v54; // rcx
    unsigned int v55; // er8
    __int64 v56; // rdi
    __int64 v57; // rdx
    char v58; // cl
    int v59; // eax
    __int64 v60; // rdi
    unsigned int v61; // er8
    __int64* v62; // rdx
    char v63; // al
    uint64_t v64; // rsi
    __int64 v65; // rax
    uint64_t v66; // r9
    int v67; // er10
    uint8_t v68; // cl
    uint64_t v69; // rax
    unsigned int v70; // er8
    unsigned int i; // ecx
    __int64 v72; // rax
    int v73; // eax
    __int16 v74; // ax
    uint64_t v75; // rdi
    __int64 v76; // rax
    uint64_t v77; // rsi
    uint64_t v78; // rax
    int v79; // ebp
    __int64 v80; // [rsp+0h] [rbp-58h]
    int v81; // [rsp+60h] [rbp+8h]
    uint64_t v83; // [rsp+70h] [rbp+18h]
    __int64 v84; // [rsp+78h] [rbp+20h]

    v83 = a3;
    v3 = a3;
    if (a2 < a1[11])
        return 0;
    v6 = a1[10];
    if (a3 < a1[7] + (v6 & (uint64_t)~a1[7]) + 1 && a3 < a1[5])
        return 0;
    v7 = *((_DWORD*)a1 + 27);
    v8 = a1[9];
    v9 = *((_DWORD*)a1 + 26);
    v10 = a1[12];
    v11 = *a1;
    v12 = a1[14];
    v80 = a1[1];
    v84 = *a1;
    if (a1[15] < v12)
        v12 = a1[15];
    while (1)
    {
        v13 = (uint64_t)v7 << 8;
        v14 = LUT_200[v13 + (uint8_t)v10];
        v15 = v13 + (uint8_t)v10;
        v16 = v14 + v9;
        v17 = v10 >> v14;
        v18 = LUT_0[v15];
        if (v18 < 0)
        {
            v59 = LUT_4E0[v7];
            v60 = v80 + (v6 & a1[3]);
            v61 = -v18;
            v7 = 1;
            v62 = (__int64*)(v11 + (v8 & a1[2]));
            v81 = 1;
            if (v61 == v59)
            {
                if ((~v8 & a1[6]) < 0xF || (a1[7] & (uint64_t)~v6) < 0xF || (uint64_t)(a1[5] - v6) < 0x10)
                    v61 = 1;
                v63 = v17;
                v64 = v17 >> 3;
                v65 = v63 & 7;
                v66 = v64;
                if (v65)
                {
                    v67 = LUT_4D0[v65];
                    v68 = LUT_4D8[v65];
                }
                else
                {
                    v66 = v64 >> 4;
                    v69 = v64 & 0xF;
                    v16 += 4;
                    v67 = LUT_480[v69];
                    v68 = LUT_4C0[v69];
                }
                v16 += v68 + 3;
                v17 = v66 >> v68;
                v70 = v67 + (v66 & ((1 << v68) - 1)) + v61;
                for (i = v70 >> 3; i; --i)
                {
                    v72 = *v62;
                    v60 += 8LL;
                    ++v62;
                    *(_QWORD*)(v60 - 8) = v72;
                }
                if ((v70 & 4) != 0)
                {
                    v73 = *(_DWORD*)v62;
                    v60 += 4LL;
                    v62 = (__int64*)((char*)v62 + 4);
                    *(_DWORD*)(v60 - 4) = v73;
                }
                if ((v70 & 2) != 0)
                {
                    v74 = *(_WORD*)v62;
                    v60 += 2LL;
                    v62 = (__int64*)((char*)v62 + 2);
                    *(_WORD*)(v60 - 2) = v74;
                }
                if ((v70 & 1) != 0)
                    *(_BYTE*)v60 = *(_BYTE*)v62;
                v8 += v70;
                v6 += v70;
                goto LABEL_12;
            }
            *(_QWORD*)v60 = *v62;
            *(_QWORD*)(v60 + 8) = v62[1];
            v8 += v61;
            v6 += v61;
        }
        else
        {
            v19 = v18;
            v20 = v17 & 0xF;
            v81 = 0;
            v21 = ((unsigned int)(v20 - 31) >> 3) & 6;
            v22 = ((unsigned int)v17 >> v21) & 0x3F;
            v23 = v20 + ((v17 >> 4) & ((24 * (((unsigned int)(v20 - 31) >> 3) & 2)) >> 4));
            v24 = v21 + LUT_440[v22];
            LOBYTE(v21) = v20 + ((v17 >> 4) & ((24 * (((unsigned int)(v20 - 31) >> 3) & 2)) >> 4));
            v25 = (1 << v21) + (((1 << v21) - 1) & (v17 >> v24));
            v16 += v24 + v23;
            v26 = a1[3];
            v17 >>= (uint8_t)v24 + (uint8_t)v23;
            v27 = 16 * v25 - 16 + LUT_400[v22];
            v28 = v26 & (v6 - v27);
            v29 = (_QWORD*)(v80 + (v6 & v26));
            v30 = (char*)(v80 + v28);
            if (v19 != 17)
            {
                v31 = v19;
                v11 = v84;
                v6 += v31;
                *v29 = *(_QWORD*)v30;
                v29[1] = *((_QWORD*)v30 + 1);
            LABEL_11:
                v3 = v83;
            LABEL_12:
                v7 = v81;
                goto LABEL_13;
            }
            v11 = v84;
            v43 = v17;
            v44 = v17 >> 3;
            v45 = v43 & 7;
            v46 = v44;
            if (v45)
            {
                v47 = LUT_4D0[v45];
                v48 = LUT_4D8[v45];
            }
            else
            {
                v46 = v44 >> 4;
                v16 += 4;
                v49 = v44 & 0xF;
                v47 = LUT_480[v49];
                v48 = LUT_4C0[v49];
                if (v84 && v16 + v48 >= 0x3D)
                {
                    v50 = v8++ & a1[2];
                    v46 |= (uint64_t)*(uint8_t*)(v50 + v84) << (61 - (uint8_t)v16);
                    v16 -= 8;
                }
            }
            v16 += v48 + 3;
            v17 = v46 >> v48;
            v51 = ((unsigned int)v46 & ((1 << v48) - 1)) + v47 + 17;
            v6 += v51;
            if (v27 >= 8)
            {
                //++dword_18004324C; // wtf?
                if ((_DWORD)v51)
                {
                    v52 = v30 - (char*)v29;
                    v53 = ((unsigned int)(v51 - 1) >> 3) + 1;
                    do
                    {
                        v54 = *(_QWORD*)((char*)v29++ + v52);
                        *(v29 - 1) = v54;
                        --v53;
                    }           while (v53);
                }
                goto LABEL_11;
            }
            v55 = v51 - 13;
            v6 -= 13LL;
            if (v27 != 1)
            {
                //++dword_180043254; // wtf?
                if (v55)
                {
                    v56 = v30 - (char*)v29;
                    v57 = v55;
                    do
                    {
                        v58 = *((_BYTE*)v29 + v56);
                        v29 = (_QWORD*)((char*)v29 + 1);
                        *((_BYTE*)v29 - 1) = v58;
                        --v57;
                    }           while (v57);
                }
                goto LABEL_11;
            }
            //++dword_180043250; // wtf?
            v7 = 0;
            v3 = v83;
            if (v55)
                //memset64(v29, 0x101010101010101LL * (uint8_t)*v30, ((v55 - 1) >> 3) + 1);
                memset((void*)v29, (uint8_t)*v30, v55);
        }
    LABEL_13:
        if (v8 < v12)
            goto LABEL_27;
        if (v6 == a1[16])
        {
            v32 = a1[5];
            if (v6 == v32)
            {
                result = 1;
                goto LABEL_67;
            }
            v33 = a1[6];
            v34 = *((unsigned int*)a1 + 16);
            v17 >>= 1;
            ++v16;
            v35 = v33 & -(__int64)v8;
            if (v34 > v35)
            {
                v8 += v35;
                v36 = a1[14];
                if (v8 > v36)
                    a1[14] = v33 + v36 + 1;
            }
            v37 = v8 & a1[2];
            v8 += v34;
            v38 = *(_QWORD*)(v37 + v11) & ((1LL << (8 * (uint8_t)v34)) - 1);
            v39 = v6 + a1[7] + 1;
            a1[11] += v38;
            a1[15] += v38;
            if (v39 >= v32)
            {
                v39 = v32;
                a1[15] += v34;
            }
            a1[16] = v39;
            if (a2 < a1[11] || v3 < v39)
                break;
        }
        v40 = a1[14];
        if (v8 >= v40)
        {
            v8 = ~a1[6] & (v8 + 7);
            a1[14] = a1[6] + v40 + 1;
        }
        v12 = a1[14];
        if (a1[15] < v12)
            v12 = a1[15];
    LABEL_27:
        v41 = (*(_QWORD*)((v8 & a1[2]) + v11) << (64 - (uint8_t)v16)) | v17;
        v42 = v16;
        v9 = v16 & 7;
        v8 += v42 >> 3;
        v10 = (0xFFFFFFFFFFFFFFFFuLL >> v9) & v41;
    }
    v75 = a1[14];
    if (v8 >= v75)
    {
        v8 = ~a1[6] & (v8 + 7);
        a1[14] = a1[6] + v75 + 1;
    }
    v76 = *(_QWORD*)((v8 & a1[2]) + v11);
    *((_DWORD*)a1 + 27) = v7;
    v77 = (v76 << (64 - (uint8_t)v16)) | v17;
    v78 = v16;
    v79 = v16 & 7;
    *((_DWORD*)a1 + 26) = v79;
    v8 += v78 >> 3;
    result = 0;
    a1[12] = v77 & (0xFFFFFFFFFFFFFFFFuLL >> v79);
LABEL_67:
    a1[10] = v6;
    a1[9] = v8;
    return result;
}

extern "C" __int64 get_decompressed_size(__int64 params, uint8_t* file_buf, __int64 some_magic_shit, __int64 file_size, __int64 off_without_header_qm, __int64 header_size)
{
    //__int64 params = __int64(aparams);

    __int64 v8; // r9
    uint64_t v9; // r11
    char v10; // r8
    int v11; // er8
    __int64 v12; // rbx
    unsigned int v13; // ebp
    uint64_t v14; // rbx
    __int64 v15; // rax
    unsigned int v16; // er9
    uint64_t v17; // r12
    uint64_t v18; // r11
    uint64_t v19; // r10
    uint64_t v20; // rax
    int v21; // ebp
    uint64_t v22; // r10
    unsigned int v23; // er9
    __int64 v24; // rax
    __int64 v25; // rsi
    __int64 v26; // rdx
    __int64 result; // rax
    __int64 v28; // rdx
    __int64 v29; // [rsp+48h] [rbp+18h]

    v29 = some_magic_shit;
    *(_QWORD*)params = _QWORD(file_buf);
    *(_QWORD*)(params + 32) = off_without_header_qm + file_size;
    *(_QWORD*)(params + 8) = 0LL;
    *(_QWORD*)(params + 24) = 0LL;
    *(_DWORD*)(params + 68) = 0;
    *(_QWORD*)(params + 16) = some_magic_shit;
    v8 = off_without_header_qm + header_size + 8;
    v9 = *(_QWORD*)&file_buf[some_magic_shit & (off_without_header_qm + header_size)];
    *(_QWORD*)(params + 80) = header_size;
    *(_QWORD*)(params + 72) = v8;
    v10 = v9;
    v9 >>= 6;
    v11 = v10 & 0x3F;
    *(_QWORD*)(params + 40) = (1LL << v11) | v9 & ((1LL << v11) - 1);
    v12 = *(_QWORD*)&file_buf[some_magic_shit & v8] << (64 - ((uint8_t)v11 + 6));
    *(_QWORD*)(params + 72) = v8 + ((uint64_t)(unsigned int)(v11 + 6) >> 3);
    v13 = ((v11 + 6) & 7) + 13;
    v14 = (0xFFFFFFFFFFFFFFFFuLL >> ((v11 + 6) & 7)) & ((v9 >> v11) | v12);
    v15 = v29 & *(_QWORD*)(params + 72);
    v16 = (((_BYTE)v14 - 1) & 0x3F) + 1;
    v17 = 0xFFFFFFFFFFFFFFFFuLL >> (64 - (uint8_t)v16);
    *(_QWORD*)(params + 48) = v17;
    v18 = 0xFFFFFFFFFFFFFFFFuLL >> (64 - ((((v14 >> 6) - 1) & 0x3F) + 1));
    *(_QWORD*)(params + 56) = v18;
    v19 = (v14 >> 13) | (*(_QWORD*)&file_buf[v15] << (64 - (uint8_t)v13));
    v20 = v13;
    v21 = v13 & 7;
    *(_QWORD*)(params + 72) += v20 >> 3;
    v22 = (0xFFFFFFFFFFFFFFFFuLL >> v21) & v19;
    if (v17 == -1LL)
    {
        *(_DWORD*)(params + 64) = 0;
        *(_QWORD*)(params + 88) = file_size;
    }
    else
    {
        v23 = v16 >> 3;
        v24 = v29 & *(_QWORD*)(params + 72);
        *(_DWORD*)(params + 64) = v23 + 1;
        v25 = *(_QWORD*)&file_buf[v24] & ((1LL << (8 * ((uint8_t)v23 + 1))) - 1);
        *(_QWORD*)(params + 72) += v23 + 1;
        *(_QWORD*)(params + 88) = v25;
    }
    *(_QWORD*)(params + 88) += off_without_header_qm;
    v26 = *(_QWORD*)(params + 88);
    *(_QWORD*)(params + 96) = v22;
    *(_DWORD*)(params + 104) = v21;
    *(_QWORD*)(params + 112) = v17 + off_without_header_qm - 6;
    result = *(_QWORD*)(params + 40);
    *(_DWORD*)(params + 108) = 0;
    *(_QWORD*)(params + 120) = v26;
    *(_QWORD*)(params + 128) = result;
    if ((((uint8_t)(v14 >> 6) - 1) & 0x3F) != -1LL && result - 1 > v18)
    {
        v28 = v26 - *(unsigned int*)(params + 64);
        *(_QWORD*)(params + 128) = v18 + 1;
        *(_QWORD*)(params + 120) = v28;
    }
    return result;
}