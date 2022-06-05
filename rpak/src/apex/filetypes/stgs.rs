use std::io::{Read, Seek, SeekFrom};

use byteorder::{ReadBytesExt, LE};

use crate::{util, FileEntry};

#[derive(Debug)]
pub struct Settings {
    internal: super::FileGeneric,

    pub stlt_hash: u64, // STLT hash

    pub unk8: u64, // ptr
    //pub name_off: u64, // 0x10 - ptr
    pub unk18: u64, // 0x18 - why???

    pub unk20: u64, // ptr

    pub unk28: u64, // NOT ptr

    pub unk30: u64, // ptr
    pub unk38: u64, // ptr

    pub unk40: u64, // or 32???

    pub unk48: u32, // count?
    pub unk4C: u32, // ???

    name: String,

    mods: Vec<String>,
}

impl crate::FileEntry for Settings {
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
        assert_eq!(self.internal.data, None, "Конец света");
        None // ???
    }

    fn get_name(&self) -> Option<&str> {
        Some(self.name.as_str())
    }

    fn get_star_off(&self) -> Option<u64> {
        None
    }
    fn get_star_opt_off(&self) -> Option<u64> {
        None
    }

    fn get_version(&self) -> u32 {
        self.internal.version
    }

    fn get_ext(&self) -> &str {
        "stgs"
    }
}

impl Settings {
    pub fn ctor<R: Read + Seek + ReadBytesExt>(
        cursor: &mut R,
        seeks: &[u64],
        generic: super::FileGeneric,
    ) -> Result<Self, std::io::Error> {
        cursor.seek(SeekFrom::Start(generic.get_desc_off()))?;

        let unk0 = cursor.read_u64::<LE>().unwrap();

        let unk8 = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };
        let name_off = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };
        let name_off2 = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        let unk20 = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        let unk28 = cursor.read_u64::<LE>()?;

        let unk30 = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };
        let unk38 = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        let unk40 = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        let unk48 = cursor.read_u32::<LE>().unwrap();
        let unk4C = cursor.read_u32::<LE>().unwrap();

        cursor.seek(SeekFrom::Start(name_off))?;
        let name = util::string_from_buf(cursor);

        let mods = {
            let mut ret = Vec::<String>::with_capacity(unk48 as usize);

            let mut section_id = 0u32;
            for i in 0..unk48 {
                cursor.seek(SeekFrom::Start(unk30 as u64 + 8 * i as u64))?;
                let name_off = {
                    let id = cursor.read_u32::<LE>()?;
                    let off = cursor.read_u32::<LE>()?;

                    section_id = id;

                    seeks[id as usize] + off as u64
                };
                cursor.seek(SeekFrom::Start(name_off))?;
                let name = util::string_from_buf(cursor);
                ret.push(name);
            }

            if unk48 != 0 {
                assert_ne!(
                    section_id,
                    cursor.read_u32::<LE>()?,
                    "Ебанный бобанный {:x} | {}",
                    unk30,
                    unk48
                );
            }

            ret
        };

        Ok(Self {
            internal: generic,

            stlt_hash: unk0,
            unk8,
            //name_off,
            unk18: name_off2,
            unk20,
            unk28,

            unk30,
            unk38,
            unk40,
            unk48,
            unk4C,

            name,
            mods,
        })
    }
}
