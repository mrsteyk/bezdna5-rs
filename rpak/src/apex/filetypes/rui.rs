use std::{
    cmp::max,
    io::{Read, Seek, SeekFrom},
};

use byteorder::{ReadBytesExt, LE};

use crate::FileEntry;

#[derive(Debug)]
pub struct ArgCluster {
    /// From which argument to start
    pub start_num: u16,
    /// How many arguments
    pub arg_num: u16,

    /// Hash mul param
    pub hash_mul: u8,
    /// Hash add param
    pub hash_add: u8,
}

impl ArgCluster {
    pub fn hash(&self, string: &str) -> u32 {
        self::hash(string, self.hash_mul, self.hash_add)
    }

    pub fn read<R: Read + Seek + ReadBytesExt>(cursor: &mut R) -> Result<Self, std::io::Error> {
        Ok(Self {
            start_num: cursor.read_u16::<LE>()?,
            arg_num: cursor.read_u16::<LE>()?,

            hash_mul: cursor.read_u8()?,
            hash_add: cursor.read_u8()?,
        })
    }
}

pub fn hash(string: &str, mul: u8, add: u8) -> u32 {
    let mut hash = 0u32;
    for i in string.as_bytes() {
        let v12 = hash >> 20;
        let v13 = (add as u32)
            .wrapping_add(hash.wrapping_mul(mul as u32))
            .wrapping_add((*i) as u32);
        hash = v12 ^ v13;
    }

    hash
}

#[derive(Debug)]
pub enum ArgType {
    Invalid(u8),
    // ???
    String, // 1

    AssetString, // 2
    Boolean,     // 3
    Int,         // 4
    Float,       // 5
    Float2,      // 6
    Float3,      // 7
    Float4,      // 8
    GameTime,    // 9
    WallTime,    // 10
    UIHandle,    // 11
    ImageString, // 12 String???
    FontFace,    // 13
    FontHash,    // 14
    Array,       // 15
}

#[derive(Debug)]
pub struct Arg {
    pub typ: ArgType,
    pub ro: bool,
    pub off: u16,
    pub unk: u16,
    pub hash16_shr4: u16,
}

impl Arg {
    pub fn read<R: Read + Seek + ReadBytesExt>(cursor: &mut R) -> Result<Self, std::io::Error> {
        Ok(Self {
            typ: match cursor.read_u8()? {
                1 => ArgType::String,
                //12 => ArgType::String2,
                2 => ArgType::AssetString,
                3 => ArgType::Boolean,
                4 => ArgType::Int,
                5 => ArgType::Float,        // 5
                6 => ArgType::Float2,       // 6
                7 => ArgType::Float3,       // 7
                8 => ArgType::Float4,       // 8
                9 => ArgType::GameTime,     // 9
                10 => ArgType::WallTime,    // 10
                11 => ArgType::UIHandle,    // 11
                12 => ArgType::ImageString, // 12 ???
                13 => ArgType::FontFace,    // 13
                14 => ArgType::FontHash,    // 14
                15 => ArgType::Array,       // 15
                v => ArgType::Invalid(v),
            },
            ro: cursor.read_u8()? != 0,
            off: cursor.read_u16::<LE>()?,
            unk: cursor.read_u16::<LE>()?,
            hash16_shr4: cursor.read_u16::<LE>()?,
        })
    }
}

#[derive(Debug)]
pub struct RUI {
    pub generic: super::FileGeneric,

    pub name: String,
    pub unk1: super::Descriptor,
    pub unk2: super::Descriptor,

    // 0x30
    pub arg_clusters: Vec<ArgCluster>,
    pub args: Vec<Arg>,
}

impl crate::FileEntry for RUI {
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
        None // we know for sure
    }
    fn get_desc_size(&self) -> usize {
        self.generic.get_desc_size()
    }

    fn get_name(&self) -> Option<&str> {
        Some(self.name.as_ref())
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
        "ui"
    }
}

impl RUI {
    pub fn ctor<R: Read + Seek + ReadBytesExt>(
        cursor: &mut R,
        seeks: &[u64],
        generic: super::FileGeneric,
    ) -> Result<Self, std::io::Error> {
        cursor.seek(SeekFrom::Start(generic.get_desc_off()))?;
        let name_seek = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        let unk1 = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            (id, off, seeks[id as usize] + off as u64)
        };
        let unk2 = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            (id, off, seeks[id as usize] + off as u64)
        };

        cursor.seek(SeekFrom::Start(generic.get_desc_off() + 0x30))?;
        let arg_cluster_seek = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };
        let arg_seek = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        cursor.seek(SeekFrom::Start(generic.get_desc_off() + 0x4e))?;
        let arg_cluster_cnt = cursor.read_u16::<LE>()?;
        if arg_cluster_cnt != 1 && arg_cluster_cnt != 0 {
            todo!("arg_cluser_cnt = {}", arg_cluster_cnt);
        }

        cursor.seek(SeekFrom::Start(name_seek))?;
        let name = crate::util::string_from_buf(cursor);

        let mut arg_cnt = 0usize;
        let mut arg_clusters = Vec::<ArgCluster>::with_capacity(arg_cluster_cnt as usize);
        for i in 0..arg_cluster_cnt {
            // TODO: move this seek out of the loop
            cursor.seek(SeekFrom::Start(arg_cluster_seek + (i as u64) * 18))?;
            let arg_cluster = ArgCluster::read(cursor)?;
            arg_cnt = max(
                arg_cnt,
                arg_cluster.start_num as usize + arg_cluster.arg_num as usize,
            );
            arg_clusters.push(arg_cluster);
        }

        let mut args = Vec::<Arg>::with_capacity(arg_cnt);
        cursor.seek(SeekFrom::Start(arg_seek))?;
        for _ in 0..arg_cnt {
            args.push(Arg::read(cursor)?)
        }

        Ok(Self {
            generic,

            name,

            unk1,
            unk2,

            arg_clusters,
            args,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::hash;

    #[test]
    fn announcement_quick_right_hash() {
        for i in [
            ("startTime", 4, 18893),
            ("messageText", 2, 8073),
            ("messageSubText", 3, 56142),
            ("duration", 1, 23028),
            ("eventColor", 0, 48850),
        ] {
            let hash = hash(i.0, 82, 0);
            assert_eq!(
                hash & 7,
                i.1,
                "Arg ID of hash {:x}({}) doesn't match id of {}",
                hash,
                i.0,
                i.1
            );
            assert_eq!(
                (hash >> 4) & 0xFFFF,
                i.2,
                "PartHash of hash {:x}({}) doesn't match {}",
                hash,
                i.0,
                i.2
            );
        }
    }
}
