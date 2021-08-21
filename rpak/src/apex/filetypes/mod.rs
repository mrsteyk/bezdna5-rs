use std::io::{Read, Seek};

use byteorder::{ReadBytesExt, LE};

type Descriptor = (u32, u32, u64);

pub mod dtbl;
pub mod matl;
pub mod rui;
pub mod stgs;
pub mod stlt;
pub mod txtr;
pub mod uimg;
pub mod rmdl;
pub mod arig;

#[derive(Debug)]
pub struct FileGeneric {
    pub guid: u64,
    pub desc: Descriptor,
    pub data: Option<Descriptor>,

    pub starpak: Option<u64>,
    pub starpak_opt: Option<u64>,

    pub desc_size: u32,

    pub extension: String,
}

impl crate::FileEntry for FileGeneric {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_guid(&self) -> u64 {
        self.guid
    }

    fn get_desc_off(&self) -> u64 {
        self.desc.2
    }
    fn get_desc_size(&self) -> usize {
        self.desc_size as usize
    }

    fn get_data_off(&self) -> Option<u64> {
        self.data.map(|val| val.2)
    }

    fn get_name(&self) -> Option<&str> {
        None
    }

    fn get_star_off(&self) -> Option<u64> {
        self.starpak
    }
    fn get_star_opt_off(&self) -> Option<u64> {
        self.starpak_opt
    }

    fn get_ext(&self) -> &str {
        self.extension.as_ref()
    }
}

impl FileGeneric {
    pub fn read<R: Read + Seek + ReadBytesExt>(
        cursor: &mut R,
        seeks: &[u64],
    ) -> Result<Self, std::io::Error> {
        //let start_pos = cursor.stream_position()?;

        let guid = cursor.read_u64::<LE>()?;
        let _name_pad = cursor.read_u64::<LE>()?;

        let description_id = cursor.read_u32::<LE>()?;
        let description_offset = cursor.read_u32::<LE>()?;
        let description_seek = if description_id as usize >= seeks.len() {
            0u64
        } else {
            seeks[description_id as usize] + description_offset as u64
        };

        let data = {
            let data_id = cursor.read_u32::<LE>()?;
            let data_offset = cursor.read_u32::<LE>()?;
            match data_id {
                u32::MAX => None,
                data_id => {
                    if data_id as usize >= seeks.len() {
                        None
                    } else {
                        let data_seek = seeks[data_id as usize] + data_offset as u64;
                        Some((data_id, data_offset, data_seek))
                    }
                }
            }
        };

        let starpak = match cursor.read_u64::<LE>()? {
            u64::MAX => None,
            val => Some(val),
        };
        let starpak_opt = match cursor.read_u64::<LE>()? {
            u64::MAX => None,
            val => Some(val),
        };

        let _unk30 = cursor.read_u16::<LE>()?;
        let _unk32 = cursor.read_u16::<LE>()?;

        let _unk34 = cursor.read_u32::<LE>()?;
        let _start_idx = cursor.read_u32::<LE>()?;
        let _unk3c = cursor.read_u32::<LE>()?;
        let _count = cursor.read_u32::<LE>()?;

        let desc_size = cursor.read_u32::<LE>()?;
        let _description_align = cursor.read_u32::<LE>()?;

        let mut extension_raw = [0u8; 4];
        cursor.read_exact(&mut extension_raw)?;
        let extension = if description_seek == 0 && data == None {
            "BORK".to_owned()
        } else {
            unsafe { crate::util::str_from_u8_nul_utf8_unchecked(&extension_raw).to_owned() }
        };

        Ok(Self {
            guid,
            desc: (description_id, description_offset, description_seek),
            data,

            starpak,
            starpak_opt,

            desc_size,
            extension,
        })
    }
}
