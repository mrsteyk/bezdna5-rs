use std::io::{Read, Seek, SeekFrom};

use byteorder::{ReadBytesExt, LE};

use crate::{util::string_from_buf, FileEntry};

#[derive(Debug)]
pub enum MaterialError {
    IOError(std::io::Error),
}

impl From<std::io::Error> for MaterialError {
    fn from(v: std::io::Error) -> Self {
        Self::IOError(v)
    }
}

impl From<MaterialError> for crate::RPakError {
    fn from(item: MaterialError) -> Self {
        Self::FileTypeParseError((
            "mtrl",
            Box::new(match item {
                MaterialError::IOError(v) => Self::IOError(v),
                _ => crate::RPakError::Shiz("Unknown error!".to_string()),
            }),
        ))
    }
}

#[derive(Debug)]
pub struct Material {
    pub generic: super::FileGeneric,

    pub guid: u64,
    pub name: String,
    pub surface_props: String,

    pub texture_guids: Vec<u64>,
}

impl crate::FileEntry for Material {
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
        Some(self.generic.get_data_off().unwrap())
    }
    fn get_desc_size(&self) -> usize {
        self.generic.get_desc_size()
    }

    fn get_name(&self) -> Option<&str> {
        Some(&self.name)
    }

    fn get_star_off(&self) -> Option<u64> {
        assert_eq!(self.generic.get_star_off(), None);
        None
    }
    fn get_star_opt_off(&self) -> Option<u64> {
        assert_eq!(self.generic.get_star_opt_off(), None);
        None
    }

    fn get_ext(&self) -> &str {
        "matl"
    }
}

impl Material {
    pub fn ctor<R: Read + Seek + ReadBytesExt>(
        cursor: &mut R,
        seeks: &[u64],
        generic: super::FileGeneric,
    ) -> Result<Self, MaterialError> {
        cursor.seek(SeekFrom::Start(generic.get_desc_off()))?;

        let _unk0 = cursor.read_u64::<LE>()?;
        assert_eq!(_unk0, 0, "pad0 isn't 0!");
        let _unk8 = cursor.read_u64::<LE>()?;
        assert_eq!(_unk8, 0, "pad8 isn't 0!");

        let guid = cursor.read_u64::<LE>()?;

        let name_seek = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        let mat_name_seek = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        cursor.seek(SeekFrom::Start(name_seek))?;
        let name = string_from_buf(cursor);
        cursor.seek(SeekFrom::Start(mat_name_seek))?;
        let surface_props = string_from_buf(cursor);

        cursor.seek(SeekFrom::Start(generic.get_desc_off() + 0x60))?;

        let unk60_seek = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        let unk68_seek = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        let refcnt = (unk68_seek - unk60_seek) / 8;

        let texture_guids = {
            let mut ret = Vec::<u64>::with_capacity(refcnt as usize);

            cursor.seek(SeekFrom::Start(unk60_seek))?;

            for _ in 0..refcnt {
                ret.push(cursor.read_u64::<LE>()?);
            }

            ret
        };

        Ok(Self {
            generic,

            guid,
            name,
            surface_props,

            texture_guids,
        })
    }
}
