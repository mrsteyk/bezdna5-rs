use std::io::{Read, Seek, SeekFrom};

use byteorder::{ReadBytesExt, LE};

use crate::{util, FileEntry};

#[derive(Debug)]
pub struct AnimationRig {
    internal: super::FileGeneric,

    pub name: String,

    pub unk0: u64, // seek, IDST6
    // pub unk8: u64, // seek, name
    pub unk10: u32,
    pub unk14: u32,
    pub unk18: u64, // seek
    pub unk20: u64,
}

impl crate::FileEntry for AnimationRig {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_guid(&self) -> u64 {
        self.internal.guid
    }

    fn get_desc_off(&self) -> u64 {
        self.internal.desc.2
    }
    fn get_desc_size(&self) -> usize {
        self.internal.desc_size as usize
    }

    fn get_data_off(&self) -> Option<u64> {
        self.internal.get_data_off()
    }

    fn get_name(&self) -> Option<&str> {
        Some(self.name.as_str())
    }

    fn get_star_off(&self) -> Option<u64> {
        self.internal.get_star_off()
    }
    fn get_star_opt_off(&self) -> Option<u64> {
        self.internal.get_star_opt_off()
    }

    fn get_version(&self) -> u32 {
        self.internal.version
    }

    fn get_ext(&self) -> &str {
        "arig"
    }
}

impl AnimationRig {
    pub fn ctor<R: Read + Seek + ReadBytesExt>(
        cursor: &mut R,
        seeks: &[u64],
        generic: super::FileGeneric,
    ) -> Result<Self, std::io::Error> {
        cursor.seek(SeekFrom::Start(generic.get_desc_off()))?;

        let unk0 = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        let name_off = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        let unk10 = cursor.read_u32::<LE>()?;
        let unk14 = cursor.read_u32::<LE>()?;

        let unk18 = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        let unk20 = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        cursor.seek(SeekFrom::Start(name_off))?;
        let name = util::string_from_buf(cursor);

        Ok(Self {
            internal: generic,

            name,

            unk0,
            unk10,
            unk14,
            unk18,
            unk20, // ???
        })
    }
}
