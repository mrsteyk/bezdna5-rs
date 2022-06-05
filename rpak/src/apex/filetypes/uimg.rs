use std::io::{Read, Seek, SeekFrom};

use byteorder::{ReadBytesExt, LE};

use crate::FileEntry;

#[derive(Debug)]
pub enum UIMGError {
    IOError(std::io::Error),
}

impl From<std::io::Error> for UIMGError {
    fn from(v: std::io::Error) -> Self {
        Self::IOError(v)
    }
}

impl From<UIMGError> for crate::RPakError {
    fn from(item: UIMGError) -> Self {
        Self::FileTypeParseError((
            "uimg",
            Box::new(match item {
                UIMGError::IOError(v) => Self::IOError(v),
                _ => crate::RPakError::Shiz("Unknown error!".to_string()),
            }),
        ))
    }
}

// desc
// dims: (u16, u16) // 0x8 - total dimensions
// textures_num: u16 // 0xC - textures num?
// textures_offsets: Descriptor // 0x10 - [f32; 8?] times 0xC
// textures_dims_desc: Descriptor // 0x18 - WxH: (u16, u16) times 0xC
// unk20: u32 // 0x20 - used in substraction
// texture_hashes_disc: Descriptor // 0x28 - hash:u32 times 0xC?
// txtr_guid: u64 // 0x38 - texture's GUID

#[derive(Debug)]
pub struct UImgTexture {
    pub offset: [f32; 8],
    pub width: u16,
    pub height: u16,
    pub hash: u32,
}

pub fn hash(string: String) -> u32 {
    let r = crate::hash(string);
    let l = (r & 0xFFFFFFFF) as u32;
    let h = (r >> 32) as u32;

    l ^ h
}

#[derive(Debug)]
pub struct UImg {
    pub generic: super::FileGeneric,

    pub dims: (u16, u16),
    pub texture_file: u64,
    pub textures: Vec<UImgTexture>,
}

impl crate::FileEntry for UImg {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_guid(&self) -> u64 {
        self.generic.get_guid()
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
        None // we know for sure
    }
    fn get_star_opt_off(&self) -> Option<u64> {
        None // we know for sure
    }

    fn get_version(&self) -> u32 {
        self.generic.version
    }

    fn get_ext(&self) -> &str {
        "uimg"
    }
}

impl UImg {
    pub fn ctor<R: Read + Seek + ReadBytesExt>(
        cursor: &mut R,
        seeks: &[u64],
        generic: super::FileGeneric,
    ) -> Result<Self, UIMGError> {
        cursor.seek(SeekFrom::Start(generic.get_desc_off()))?;

        let start_pos = cursor.stream_position()?;

        let _unk0 = cursor.read_u64::<LE>()?;
        let dims = (cursor.read_u16::<LE>()?, cursor.read_u16::<LE>()?);
        // Я ебал в рот эту парашу лол это самое ужасное что я когда-либо видел...
        let textures_offsets_num = cursor.read_u16::<LE>()?;
        let textures_num = match cursor.read_u16::<LE>()? {
            0 => 1u16,
            val => val,
        };

        let textures_offsets_desc = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            (id, off, seeks[id as usize] + off as u64)
        };
        let textures_dims_desc = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            (id, off, seeks[id as usize] + off as u64)
        };

        let _unk20 = cursor.read_u32::<LE>()?;
        let _unk24 = cursor.read_u32::<LE>()?;

        let texture_hashes_disc = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            (id, off, seeks[id as usize] + off as u64)
        };

        let _unk30 = cursor.read_u64::<LE>()?;

        let texture_guid = cursor.read_u64::<LE>()?;

        assert_eq!(cursor.stream_position()? - start_pos, 0x40, "я еблан");

        // Rust :TM:
        let textures = {
            let texture_offsets = {
                let mut ret = Vec::<[f32; 8]>::with_capacity(textures_offsets_num as usize);
                cursor.seek(SeekFrom::Start(textures_offsets_desc.2))?;
                for _ in 0..textures_offsets_num {
                    let mut tmp = [0f32; 8];
                    cursor.read_f32_into::<LE>(&mut tmp)?;
                    ret.push(tmp);
                }
                ret
            };

            let texture_dims = {
                let mut ret = Vec::<(u16, u16)>::with_capacity(textures_num as usize);
                cursor.seek(SeekFrom::Start(textures_dims_desc.2))?;
                for _ in 0..textures_num {
                    let mut tmp = [0u16; 2];
                    cursor.read_u16_into::<LE>(&mut tmp)?;
                    ret.push((tmp[0], tmp[1]));
                }
                ret
            };

            let texture_hashes = {
                let mut ret = Vec::<u32>::with_capacity(textures_num as usize);
                cursor.seek(SeekFrom::Start(texture_hashes_disc.2))?;
                for _ in 0..textures_num {
                    ret.push(cursor.read_u32::<LE>()?);
                }
                ret
            };

            let mut ret = Vec::<UImgTexture>::with_capacity(textures_num as usize);
            for i in 0..textures_num as usize {
                ret.push(UImgTexture {
                    width: texture_dims[i].0,
                    height: texture_dims[i].1,
                    offset: texture_offsets[i],
                    hash: texture_hashes[i],
                })
            }
            ret
        };

        Ok(Self {
            generic,

            dims,
            texture_file: texture_guid,
            textures,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::hash;

    #[test]
    fn it_works() {
        for i in &[
            (
                "loadscreens/mp_rr_desertlands_64k_x_64k_tt_widescreen",
                0xBB80EA2E,
            ),
            (
                "loadscreens/mp_rr_desertlands_64k_x_64k_nx_widescreen",
                0x6ddde368,
            ),
        ] {
            assert_eq!(hash(i.0.to_owned()), i.1);
        }
    }
}
