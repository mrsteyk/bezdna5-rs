use std::io::{Read, Seek, SeekFrom};

use byteorder::{ReadBytesExt, LE};

use crate::{util, FileEntry};

#[derive(Debug)]
pub enum PatchError {
    IOError(std::io::Error),
}

impl From<std::io::Error> for PatchError {
    fn from(v: std::io::Error) -> Self {
        Self::IOError(v)
    }
}

impl From<PatchError> for crate::RPakError {
    fn from(item: PatchError) -> Self {
        Self::FileTypeParseError((
            "Ptch",
            Box::new(match item {
                PatchError::IOError(v) => Self::IOError(v),
                _ => crate::RPakError::Shiz("Unknown error!".to_string()),
            }),
        ))
    }
}

#[derive(Debug)]
pub struct PatchEntry {
    pub rpak: String,
    pub num: u8,
}

#[derive(Debug)]
pub struct Patch {
    pub generic: super::FileGeneric,

    pub unk: u32,
    pub patches: Vec<PatchEntry>,
}

impl crate::FileEntry for Patch {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_guid(&self) -> u64 {
        self.generic.guid
    }

    fn get_desc_off(&self) -> u64 {
        self.generic.desc.2
    }
    fn get_desc_size(&self) -> usize {
        self.generic.desc_size as usize
    }

    fn get_data_off(&self) -> Option<u64> {
        assert_eq!(self.generic.get_data_off(), None, "Patch's Data != None");
        None
    }

    fn get_name(&self) -> Option<&str> {
        None
    }

    fn get_star_off(&self) -> Option<u64> {
        assert_eq!(self.generic.get_star_off(), None, "Patch's Star != None");
        None
    }
    fn get_star_opt_off(&self) -> Option<u64> {
        assert_eq!(
            self.generic.get_star_opt_off(),
            None,
            "Patch's StarOpt != None"
        );
        None
    }

    fn get_ext(&self) -> &str {
        "Ptch"
    }
}

impl Patch {
    pub fn ctor<R: Read + Seek + ReadBytesExt>(
        cursor: &mut R,
        seeks: &[u64],
        generic: super::FileGeneric,
    ) -> Result<Self, PatchError> {
        cursor.seek(SeekFrom::Start(generic.get_desc_off()))?;

        let start_pos = cursor.stream_position()?;

        let unk = cursor.read_u32::<LE>()?;
        let patches_num = cursor.read_u32::<LE>()?;

        let patches_names_off = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };
        let patches_nums_off = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        assert_eq!(
            cursor.stream_position()? - start_pos,
            0x18,
            "патч май харт виз лав плокс"
        );

        let patches_names = match (0..patches_num)
            .map(|i| -> Result<String, PatchError> {
                cursor.seek(SeekFrom::Start(patches_names_off + (i as u64 * 8)))?;

                let id = cursor.read_u32::<LE>()?;
                let off = cursor.read_u32::<LE>()?;

                let string_off = seeks[id as usize] + off as u64;

                cursor.seek(SeekFrom::Start(string_off))?;
                Ok(util::string_from_buf_slow(cursor))
            })
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(v) => v,
            Err(v) => return Err(v),
        };

        let patches_nums = (0..patches_num).map(|i| -> Result<u8, PatchError> {
            cursor.seek(SeekFrom::Start(patches_nums_off + i as u64))?;

            Ok(cursor.read_u8()?)
        });

        let patches: Result<Vec<PatchEntry>, _> = patches_names
            .iter()
            .zip(patches_nums)
            .map(|i| -> Result<PatchEntry, PatchError> {
                let (rpak, pn) = i;
                match pn {
                    Ok(num) => Ok(PatchEntry {
                        rpak: rpak.clone(),
                        num,
                    }),
                    Err(v) => Err(v),
                }
            })
            .collect();

        match patches {
            Ok(v) => Ok(Self {
                generic,

                unk,
                patches: v,
            }),
            Err(v) => Err(v),
        }
    }
}
