use std::io::{Read, Seek, SeekFrom};

use byteorder::{ReadBytesExt, LE};

use crate::{util, FileEntry};

// 8 bytes exactly...
#[derive(Debug)]
pub struct SettingsItem {
    pub typ: u16,
    //pub st_off: u16,
    pub name: String,
    pub unk4: u32,
}

#[derive(Debug)]
pub struct SettingsLayout {
    internal: super::FileGeneric,

    //pub name_off: u64, // 0x0 - ptr
    pub name: String,

    //pub items_seek: u64, // 0x8 - ptr
    pub unk10: u64, // ptr

    pub hash_and: u32, // 0x18 - AND for hash
    //pub items_count: u32, // 0x1C - count?
    pub unk20: u32,

    // Typical fucking Respawn...
    pub hash_mul: u32, // 0x24
    pub hash_add: u32, // 0x28

    pub unk2C: u32, // 0x2C
    pub unk30: u32, // 0x30

    pub unk34: u32,

    pub string_table_off: u64, // ptr
    pub unk40: u64,            // ptr

    // ---
    pub items: Vec<SettingsItem>,
}

impl crate::FileEntry for SettingsLayout {
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
        assert_eq!(self.internal.data, None, "Конец света 2");
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

    fn get_ext(&self) -> &str {
        "stlt"
    }
}

impl SettingsLayout {
    pub fn ctor<R: Read + Seek + ReadBytesExt>(
        cursor: &mut R,
        seeks: &[u64],
        generic: super::FileGeneric,
    ) -> Result<Self, std::io::Error> {
        cursor.seek(SeekFrom::Start(generic.get_desc_off()))?;

        let name_off = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        let items_seek = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };
        let unk10 = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        let hash_and = cursor.read_u32::<LE>()?;
        let _items_count = cursor.read_u32::<LE>()?;

        let unk20 = cursor.read_u32::<LE>()?;

        let hash_mul = cursor.read_u32::<LE>()?;
        let hash_add = cursor.read_u32::<LE>()?;

        let unk2C = cursor.read_u32::<LE>()?;
        let unk30 = cursor.read_u32::<LE>()?;

        let unk34 = cursor.read_u32::<LE>()?;

        let string_table_off = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };
        let unk40 = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        // ---

        cursor.seek(SeekFrom::Start(name_off))?;
        let name = util::string_from_buf(cursor);

        let items = {
            let mut ret = Vec::<SettingsItem>::with_capacity(hash_and as usize);
            for i in 0..hash_and {
                cursor.seek(SeekFrom::Start(items_seek + i as u64 * 8))?;
                let unk0 = cursor.read_u16::<LE>()?;
                let name_off = cursor.read_u16::<LE>()?;
                let unk4 = cursor.read_u32::<LE>()?;

                cursor.seek(SeekFrom::Start(string_table_off + name_off as u64))?;
                let name = util::string_from_buf(cursor);

                ret.push(SettingsItem {
                    typ: unk0,
                    unk4,
                    name,
                });
            }

            ret
        };

        Ok(Self {
            internal: generic,

            name,

            //items_seek,
            unk10,

            hash_and,
            //items_count,
            unk20,

            hash_mul,
            hash_add,

            unk2C,
            unk30,

            unk34,

            string_table_off,
            unk40,

            items,
        })
    }
}
