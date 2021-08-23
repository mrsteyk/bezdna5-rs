use std::io::{Read, Seek, SeekFrom};

use byteorder::{ReadBytesExt, LE};

use crate::{util, FileEntry};

#[derive(Debug)]
pub struct Model {
    internal: super::FileGeneric,

    pub name: String,

    pub unk0: u64, // seek, IDST6
    //pub unk10: u64, // seek, a name?
    pub unk20: u64, // seek
    pub unk28: u64, // seek, size unk44 | unk28 == unk38???
    pub unk30: u64, // seek, size unk48
    pub unk38: u64, // seek | unk28 == unk38???
    pub unk40: u32,
    pub unk44: u32, // size for unk28
    pub unk48: u32, // size for unk30

    pub pads: Vec<u32>, // paddings...
}

impl crate::FileEntry for Model {
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

    fn get_ext(&self) -> &str {
        "mdl_"
    }
}

impl Model {
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
        assert_eq!(cursor.read_u64::<LE>()?, 0, "Reserved != 0");

        let name_off = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };
        assert_eq!(cursor.read_u64::<LE>()?, 0, "Reserved != 0");

        let unk20 = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };
        let unk28 = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

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
        // assert_eq!(cursor.read_u64::<LE>()?, 0, "Reserved != 0");

        let unk40 = cursor.read_u32::<LE>()?;
        let unk44 = cursor.read_u32::<LE>()?;
        let unk48 = cursor.read_u32::<LE>()?;

        // todo: the rest?
        // for i in 0..(0x28/4) {
        //     assert_eq!(cursor.read_u32::<LE>()?, 0, "Padding[{}] != 0", i);
        // }

        let pads: Vec<u32> = (0..(0x28 / 4))
            .map(|_| cursor.read_u32::<LE>().unwrap())
            .collect();

        cursor.seek(SeekFrom::Start(name_off))?;
        let name = util::string_from_buf(cursor);

        Ok(Self {
            internal: generic,

            name,

            unk0,
            // unk10,
            unk20,
            unk28,
            unk30,
            unk38,
            unk40,
            unk44,
            unk48,

            pads,
        })
    }
}
