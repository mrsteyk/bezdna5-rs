use std::io::{Read, Seek, SeekFrom};

use byteorder::{ReadBytesExt, LE};

use crate::FileEntry;

#[derive(Debug)]
pub enum TextureError {
    IOError(std::io::Error),
}

impl From<std::io::Error> for TextureError {
    fn from(v: std::io::Error) -> Self {
        Self::IOError(v)
    }
}

impl From<TextureError> for crate::RPakError {
    fn from(item: TextureError) -> Self {
        Self::FileTypeParseError((
            "txtr",
            Box::new(match item {
                TextureError::IOError(v) => Self::IOError(v),
                _ => crate::RPakError::Shiz("Unknown error!".to_string()),
            }),
        ))
    }
}

#[repr(C)]
pub struct TextureHeaderBSS13 {
    pub name: (u32, u32),
    pub format: u16,
    pub width: u16,
    pub height: u16,
    pub pad: u16, // always 0???
    pub array_size: u8,
    pub misc_flags: u8,
    _pad: u8,
    pub usage: u8,
    _pad2: u8,
    _pad3: u16,
    _pad4: u8,
    pub mips_1: u8, // static?
    pub mips_2: u8, // starpak?
    pub mips_3: u8, // opt?
}

pub const TEXTURE_ALGOS: [&str; 64] = [
    "DXT1",    // 0
    "DXT1",    // 1
    "UNKNOWN", // 2
    "UNKNOWN", // 3
    "UNKNOWN", // 4
    "UNKNOWN", // 5
    "BC4U",    // 6
    "UNKNOWN", // 7
    "BC5U",    // 8
    "UNKNOWN", // 9
    "BC6H",    // 10 // DDS DX10?
    "UNKNOWN", // 11
    "UNKNOWN", // 12
    "BC7U",    // 13 // DDS DX10 0x62
    "UNKNOWN", // 14
    "UNKNOWN", // 15
    "UNKNOWN", // 16
    "UNKNOWN", // 17
    "UNKNOWN", // 18
    "UNKNOWN", // 19
    "UNKNOWN", // 20
    "UNKNOWN", // 21
    "UNKNOWN", // 22
    "UNKNOWN", // 23
    "UNKNOWN", // 24
    "UNKNOWN", // 25
    "UNKNOWN", // 26
    "UNKNOWN", // 27
    "UNKNOWN", // 28
    "UNKNOWN", // 29
    "UNKNOWN", // 30
    "UNKNOWN", // 31
    "UNKNOWN", // 32
    "UNKNOWN", // 33
    "UNKNOWN", // 34
    "UNKNOWN", // 35
    "UNKNOWN", // 36
    "UNKNOWN", // 37
    "UNKNOWN", // 38
    "UNKNOWN", // 39
    "UNKNOWN", // 40
    "UNKNOWN", // 41
    "UNKNOWN", // 42
    "UNKNOWN", // 43
    "UNKNOWN", // 44 // ??? no fourcc; DDPF_ALPHAPIXELS | DDPF_LUMINANCE
    "UNKNOWN", // 45
    "UNKNOWN", // 46
    "UNKNOWN", // 47
    "UNKNOWN", // 48
    "UNKNOWN", // 49
    "UNKNOWN", // 50
    "UNKNOWN", // 51
    "UNKNOWN", // 52
    "UNKNOWN", // 53
    "UNKNOWN", // 54
    "UNKNOWN", // 55
    "UNKNOWN", // 56
    "UNKNOWN", // 57
    "UNKNOWN", // 58
    "UNKNOWN", // 59
    "UNKNOWN", // 60
    "UNKNOWN", // 61
    "UNKNOWN", // 62
    "UNKNOWN", // 63
];

const TEXTURE_SKIPS: [(u32, u32, u32); 64] = [
    (8, 4, 4),
    (8, 4, 4),
    (16, 4, 4),
    (16, 4, 4),
    (16, 4, 4),
    (16, 4, 4),
    (8, 4, 4),
    (8, 4, 4),
    (16, 4, 4),
    (16, 4, 4),
    (16, 4, 4),
    (16, 4, 4),
    (16, 4, 4),
    (16, 4, 4),
    (16, 1, 1),
    (16, 1, 1),
    (16, 1, 1),
    (12, 1, 1),
    (12, 1, 1),
    (12, 1, 1),
    (8, 1, 1),
    (8, 1, 1),
    (8, 1, 1),
    (8, 1, 1),
    (8, 1, 1),
    (8, 1, 1),
    (8, 1, 1),
    (8, 1, 1),
    (4, 1, 1),
    (4, 1, 1),
    (4, 1, 1),
    (4, 1, 1),
    (4, 1, 1),
    (4, 1, 1),
    (4, 1, 1),
    (4, 1, 1),
    (4, 1, 1),
    (4, 1, 1),
    (4, 1, 1),
    (4, 1, 1),
    (4, 1, 1),
    (4, 1, 1),
    (4, 1, 1),
    (4, 1, 1),
    (2, 1, 1),
    (2, 1, 1),
    (2, 1, 1),
    (2, 1, 1),
    (2, 1, 1),
    (2, 1, 1),
    (2, 1, 1),
    (2, 1, 1),
    (2, 1, 1),
    (1, 1, 1),
    (1, 1, 1),
    (1, 1, 1),
    (1, 1, 1),
    (1, 1, 1),
    (4, 1, 1),
    (4, 1, 1),
    (4, 1, 1),
    (2, 1, 1),
    (16, 4, 4),
    (16, 5, 4),
];

#[derive(Debug, PartialEq, Eq)]
pub enum MipMapType {
    RPak,
    StarPak,
    StarPakOpt,
}

#[derive(Debug)]
pub struct MipMap {
    pub typ: MipMapType,
    pub off: u64,

    pub width: u16,
    pub height: u16,

    pub size: u64,
}

#[derive(Debug)]
pub struct Texture {
    pub generic: super::FileGeneric,

    pub guid: u64,
    // No name in Apex...
    pub width: u16,
    pub height: u16,

    pub texture_type: u16,
    pub layers_count: u8,

    //pub starpak_mipmaps: u8, // mandatory rpak
    //pub starpak_opt_mipmaps: u8,
    //pub rpak_mipmaps: u8

    //pub starpak_mipmaps: Vec<MipMap>,
    //pub starpak_opt_mipmaps: Vec<MipMap>,
    //pub rpak_mipmaps: Vec<MipMap>,
    pub mipmaps: Vec<MipMap>,
    pub total_size: u32,
}

impl crate::FileEntry for Texture {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_guid(&self) -> u64 {
        self.guid
    }

    fn get_desc_off(&self) -> u64 {
        self.generic.get_desc_off()
    }
    fn get_data_off(&self) -> Option<u64> {
        self.generic.get_data_off()
    }
    fn get_desc_size(&self) -> usize {
        self.generic.get_desc_size()
    }

    fn get_name(&self) -> Option<&str> {
        None
    }

    fn get_star_off(&self) -> Option<u64> {
        self.generic.get_star_off()
    }
    fn get_star_opt_off(&self) -> Option<u64> {
        self.generic.get_star_opt_off()
    }

    fn get_version(&self) -> u32 {
        self.generic.version
    }

    fn get_ext(&self) -> &str {
        "txtr"
    }
}

impl Texture {
    pub fn ctor<R: Read + Seek + ReadBytesExt>(
        cursor: &mut R,
        _seeks: &[u64],
        generic: super::FileGeneric,
    ) -> Result<Self, TextureError> {
        cursor.seek(SeekFrom::Start(generic.get_desc_off()))?;

        let start_pos = cursor.stream_position()?;

        if generic.version != 9 {
            let guid = cursor.read_u64::<LE>()?;
            assert_eq!(generic.get_guid(), guid, "пакер еблан");

            let _name_pad = cursor.read_u64::<LE>()?;
            // well, technically it should be 0 but we are dumb...

            let width = cursor.read_u16::<LE>()?;
            let height = cursor.read_u16::<LE>()?;

            let _unk14 = cursor.read_u16::<LE>()?;

            let texture_type = cursor.read_u16::<LE>()?;

            let total_size = cursor.read_u32::<LE>()?;
            let _unk1c = cursor.read_u8()?;

            let starpak_opt_mipmaps_num = cursor.read_u8()?;

            let layers_count = cursor.read_u8()?;
            let _unk19 = cursor.read_u8()?; // sometimes not 0 and that fucked up my calculations for size...
            let _unk20 = cursor.read_u8()?;

            let rpak_mipmaps_num = cursor.read_u8()?;
            let starpak_mipmaps_num = cursor.read_u8()?;

            // 0x23-0x38 ergh?

            let mipmaps_num = (starpak_opt_mipmaps_num as u32)
                + (starpak_mipmaps_num as u32)
                + (rpak_mipmaps_num as u32);

            let mut total_size_check = 0u64;

            let mipmaps = {
                // leftover logic from c+p
                let unk1e = if layers_count == 0 { 1 } else { layers_count };

                let mut rpak_off = generic.get_data_off().unwrap(); // or panic...
                let mut starpak_off = generic.get_star_off().unwrap_or(0u64);
                let mut starpak_opt_off = generic.get_star_opt_off().unwrap_or(0u64);

                let mut ret = Vec::<MipMap>::with_capacity(mipmaps_num as usize);
                //for i in mipmaps_num..=0 {
                for i in (0..mipmaps_num).rev() {
                    let typ = if i < (starpak_opt_mipmaps_num as u32) {
                        MipMapType::StarPakOpt
                    } else if i < (starpak_opt_mipmaps_num as u32 + starpak_mipmaps_num as u32) {
                        MipMapType::StarPak
                    } else {
                        MipMapType::RPak
                    };

                    let (v15, v14, v16) = TEXTURE_SKIPS[texture_type as usize];

                    let v17 = if (width >> i) > 1 { width >> i } else { 1 };
                    let v22 = if (height >> i) > 1 { height >> i } else { 1 };

                    let v21 = (v14 + v17 as u32 - 1) as u32 / v14;
                    let v23 = v21 * ((v16 + v22 as u32 - 1) / v16);
                    let v25 = v15 * v23;

                    let size = ((v25 + 15) & 0xFFFFFFF0) as u64;
                    let skip_size = unk1e as u64 * size;
                    //let skip_size = size;

                    let off = match typ {
                        MipMapType::RPak => {
                            let ret = rpak_off;
                            rpak_off += skip_size;
                            ret
                        }
                        MipMapType::StarPak => {
                            let ret = starpak_off;
                            starpak_off += skip_size;
                            ret
                        }
                        MipMapType::StarPakOpt => {
                            let ret = starpak_opt_off;
                            starpak_opt_off += skip_size;
                            ret
                        }
                    };

                    total_size_check += skip_size;

                    ret.push(MipMap {
                        typ,
                        off,
                        width: v17,
                        height: v22,
                        size,
                    });
                }
                ret
            };

            assert_eq!(
                cursor.stream_position()? - start_pos,
                0x23,
                "я еблан в текстурках"
            );

            let ret = Self {
                generic,

                guid,
                width,
                height,
                texture_type,
                layers_count,

                mipmaps,
                total_size,
            };

            assert!(
                (mipmaps_num == 1) || (total_size as u64 == total_size_check),
                "Пакер втройне еблан? {} != {} | {:?} | {:#?}",
                total_size,
                total_size_check,
                (
                    rpak_mipmaps_num,
                    starpak_mipmaps_num,
                    starpak_opt_mipmaps_num
                ),
                ret
            );

            Ok(ret)
        } else {
            let _name_pad = cursor.read_u64::<LE>()?;

            let format = cursor.read_u16::<LE>()?;
            let width = cursor.read_u16::<LE>()?;
            let height = cursor.read_u16::<LE>()?;

            let _pad = cursor.read_u16::<LE>()?;
            #[cfg(debug_assertions)]
            assert_eq!(_pad, 0, "Zero pad of txtr v9 isn't 0! @ {:X}", start_pos);

            let array_size = cursor.read_u8()?;
            let _misc_flags = cursor.read_u8()?;

            let _pad1 = cursor.read_u8()?;
            let _usage = cursor.read_u8()?;
            let _pad2 = cursor.read_u8()?;
            let _pad3 = cursor.read_u16::<LE>()?;
            let _pad4 = cursor.read_u8()?;

            let rpak_mipmaps_num = cursor.read_u8()?;
            let starpak_mipmaps_num = cursor.read_u8()?;
            let starpak_opt_mipmaps_num = cursor.read_u8()?;

            let mipmaps_num = (starpak_opt_mipmaps_num as u32)
                + (starpak_mipmaps_num as u32)
                + (rpak_mipmaps_num as u32);

            let mut total_size_check = 0u64;

            let mipmaps = {
                // leftover logic from c+p
                let unk1e = if array_size == 0 { 1 } else { array_size };

                let mut rpak_off = generic.get_data_off().unwrap(); // or panic...
                let mut starpak_off = generic.get_star_off().unwrap_or(0u64);
                let mut starpak_opt_off = generic.get_star_opt_off().unwrap_or(0u64);

                let mut ret = Vec::<MipMap>::with_capacity(mipmaps_num as usize);
                //for i in mipmaps_num..=0 {
                for i in (0..mipmaps_num).rev() {
                    let typ = if i < (starpak_opt_mipmaps_num as u32) {
                        MipMapType::StarPakOpt
                    } else if i < (starpak_opt_mipmaps_num as u32 + starpak_mipmaps_num as u32) {
                        MipMapType::StarPak
                    } else {
                        MipMapType::RPak
                    };

                    let (v15, v14, v16) = TEXTURE_SKIPS[format as usize];

                    let v17 = if (width >> i) > 1 { width >> i } else { 1 };
                    let v22 = if (height >> i) > 1 { height >> i } else { 1 };

                    let v21 = (v14 + v17 as u32 - 1) as u32 / v14;
                    let v23 = v21 * ((v16 + v22 as u32 - 1) / v16);
                    let v25 = v15 * v23;

                    let size = ((v25 + 15) & 0xFFFFFFF0) as u64;
                    let skip_size = unk1e as u64 * size;
                    //let skip_size = size;

                    let off = match typ {
                        MipMapType::RPak => {
                            let ret = rpak_off;
                            rpak_off += skip_size;
                            ret
                        }
                        MipMapType::StarPak => {
                            let ret = starpak_off;
                            starpak_off += skip_size;
                            ret
                        }
                        MipMapType::StarPakOpt => {
                            let ret = starpak_opt_off;
                            starpak_opt_off += skip_size;
                            ret
                        }
                    };

                    total_size_check += skip_size;

                    ret.push(MipMap {
                        typ,
                        off,
                        width: v17,
                        height: v22,
                        size,
                    });
                }
                ret
            };

            Ok(Self {
                generic,

                guid: 0,
                width,
                height,
                texture_type: format,
                layers_count: array_size,

                mipmaps,
                total_size: 0,
            })
        }
    }
}
