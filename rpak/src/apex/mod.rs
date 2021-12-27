use std::cell::{RefCell, RefMut};
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::rc::Rc;

use crate::{NaiveDateTime, RPakFileT};
use byteorder::{ReadBytesExt, LE};

use crate::decomp::decompress;
use crate::util::string_from_buf;
use crate::{DataChunk, SectionDesc};

use self::filetypes::FileGeneric;

pub mod filetypes;

#[derive(Debug)]
pub struct ParseFileOptions {
    pub arig: bool,
    // pub aseq: bool,
    pub dtbl: bool,
    pub matl: bool,
    pub rmdl: bool,
    pub rui: bool,
    pub stgs: bool,
    pub stlt: bool,
    pub txtr: bool,
    pub uimg: bool,
    pub uiia: bool,
    pub patch: bool,
}

impl Default for ParseFileOptions {
    fn default() -> Self {
        Self {
            arig: false,
            dtbl: false,
            matl: false,
            rmdl: false,
            rui: false,
            stgs: false,
            stlt: false,
            txtr: false,
            uimg: false,
            uiia: false,
            patch: false,
        }
    }
}

#[derive(Debug)]
pub struct RPakHeader {
    pub magic: u32,
    pub version: u16,
    pub flags: u16,

    pub timestamp: NaiveDateTime,
    pub unk10: u64,

    pub size_disk: u64,
    pub unk20: u64,
    pub unk28: u64,

    pub size_decompressed: u64,
    pub unk38: u64,
    pub unk40: u64,

    pub starpak_len: u16,     // 0x48
    pub starpak_opt_len: u16, // 0x4a
    pub sections_num: u16,    // 0x4c
    pub data_chunks_num: u16, // 0x4e

    pub patches_num: u16, // 0x50

    pub unk52: u16,
    pub num_descriptors: u32,
    pub num_files: u32,
    pub relationship: u32,

    pub unk60: u32,
    pub unk64: u32,
    pub unk68: u32,
    pub unk6c: u32,

    pub unk70: u32,
    pub unk74: u32,
    pub unk78: u64,
}

impl RPakHeader {
    pub fn is_compressed(&self) -> bool {
        (self.flags >> 8) & 0xFF == 1
    }
    pub fn should_lla(&self) -> bool {
        // wait what?
        self.flags & 0x11 != 0
    }

    pub fn read<R: Read + Seek + ReadBytesExt>(cursor: &mut R) -> Result<Self, crate::RPakError> {
        cursor.seek(SeekFrom::Start(0))?;
        let magic = cursor.read_u32::<LE>()?;
        if magic != 0x6b615052 {
            return Err(crate::RPakError::InvalidMagic(magic));
        }
        let version = cursor.read_u16::<LE>()?;
        if version != 8 {
            return Err(crate::RPakError::InvalidVersion(version));
        }
        let flags = cursor.read_u16::<LE>()?;

        let header = RPakHeader {
            magic,
            version,
            flags,

            timestamp: {
                let file_time = cursor.read_u64::<LE>()?;

                if file_time != 0 {
                    let unix = (file_time / 10000000) - 11644473600;
                    NaiveDateTime::from_timestamp(unix as i64, 0)
                } else {
                    NaiveDateTime::from_timestamp(0, 0)
                }
            },
            unk10: cursor.read_u64::<LE>()?,
            size_disk: cursor.read_u64::<LE>()?,

            unk20: cursor.read_u64::<LE>()?,
            unk28: cursor.read_u64::<LE>()?,

            size_decompressed: cursor.read_u64::<LE>()?,
            unk38: cursor.read_u64::<LE>()?,
            unk40: cursor.read_u64::<LE>()?,

            starpak_len: cursor.read_u16::<LE>()?,
            starpak_opt_len: cursor.read_u16::<LE>()?,
            sections_num: cursor.read_u16::<LE>()?,
            data_chunks_num: cursor.read_u16::<LE>()?,

            patches_num: cursor.read_u16::<LE>()?,
            unk52: cursor.read_u16::<LE>()?,

            num_descriptors: cursor.read_u32::<LE>()?,
            num_files: cursor.read_u32::<LE>()?,
            relationship: cursor.read_u32::<LE>()?,

            unk60: cursor.read_u32::<LE>()?,
            unk64: cursor.read_u32::<LE>()?,
            unk68: cursor.read_u32::<LE>()?,
            unk6c: cursor.read_u32::<LE>()?,

            unk70: cursor.read_u32::<LE>()?,
            unk74: cursor.read_u32::<LE>()?,

            unk78: cursor.read_u64::<LE>()?,
        };

        if cursor.stream_position()? != crate::HEADER_SIZE_APEX as u64 {
            return Err(crate::RPakError::Shiz("apex::RPakFile::read".to_owned()));
        }

        Ok(header)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PatchHeader {
    pub data_size: u32, // The size of the patch pages???
    pub patch_from: u32, // First page to start patching
}

#[derive(Debug)]
pub struct PatchShit {
    pub header: PatchHeader,

    // This is beyond dumb
    // All this does is tell the compressed/decompressed size of the previous rpak in the chain...
    // Is this used to identify something or wha?
    pub decompressed_compressed_pair: Vec<(u64, u64)>,
    // WHY THE FUCK DOES PAIR EXIST IF THIS IS WHAT YOU USE A+_DSAIODIOASBIDUSAO(DFBOAS(BDOSABOUDASBO))
    pub rpak_number: Vec<u16>,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct RPakFile {
    pub header: RPakHeader,
    #[derivative(Debug = "ignore")]
    pub decompressed: Rc<RefCell<Cursor<Vec<u8>>>>,
    pub data_start: u64,

    pub starpak: String,
    pub starpak_opt: Option<String>,
    pub files: Vec<std::rc::Rc<dyn crate::FileEntry>>,
    pub sections: Vec<crate::SectionDesc>,
    #[derivative(Debug = "ignore")]
    pub data_chunks: Vec<DataChunk>,

    #[derivative(Debug = "ignore")]
    pub seeks: Vec<u64>,

    pub descriptors: Vec<(u32, u32)>,
    pub descriptors_guid: Vec<(u32, u32)>, // what the fuck
    pub unk60: Vec<u32>,
    pub unk64: Vec<u32>,
    pub unk68: Vec<u8>,
    pub unk6c: Vec<(u64, u64)>,
    pub unk70: Vec<(u64, u64, u64)>,

    pub patch_shit: Option<PatchShit>,
}

impl crate::RPakFileT for RPakFile {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn get_version(&self) -> crate::RPakVersion {
        crate::RPakVersion::APEX
    }
    fn get_files(&self) -> &Vec<std::rc::Rc<dyn crate::FileEntry>> {
        self.files.as_ref()
    }
    fn get_sections_desc(&self) -> &Vec<crate::SectionDesc> {
        self.sections.as_ref()
    }
    fn get_data_chunks(&self) -> &Vec<DataChunk> {
        self.data_chunks.as_ref()
    }

    fn is_compressed(&self) -> bool {
        self.header.is_compressed()
    }
    fn should_lla(&self) -> bool {
        self.header.should_lla()
    }

    fn get_decompressed(&self) -> RefMut<Cursor<Vec<u8>>> {
        (*self.decompressed).borrow_mut()
    }
}

impl RPakFile {
    pub fn read<R: Read + Seek + ReadBytesExt>(cursor: &mut R) -> Result<Self, crate::RPakError> {
        let header = RPakHeader::read(cursor)?;

        // if header.patches_num != 0 {
        //     todo!("Part RPak")
        // }
        if header.unk74 != 0 {
            todo!()
        }

        cursor.seek(SeekFrom::Start(0))?;
        let mut vec = Vec::<u8>::new();
        cursor.read_to_end(&mut vec)?;
        // TODO: maybe check disk size?
        let mut decompressed = if header.is_compressed() {
            let mut d = decompress(
                &mut vec,
                header.size_decompressed as usize,
                crate::HEADER_SIZE_APEX,
            )?;
            // TODO: fix header
            d[..crate::HEADER_SIZE_APEX].clone_from_slice(&vec[..crate::HEADER_SIZE_APEX]);
            Cursor::new(d)
        } else {
            // TODO: change to cursor's clone?
            Cursor::new(vec)
        };

        decompressed.seek(SeekFrom::Start(crate::HEADER_SIZE_APEX as u64))?;
        let patch_shit = if header.patches_num != 0 {
            Some(PatchShit {
                header: PatchHeader {
                    data_size: decompressed.read_u32::<LE>()?,
                    patch_from: decompressed.read_u32::<LE>()?,
                },

                decompressed_compressed_pair: (0..header.patches_num)
                    .map(|_| {
                        (
                            decompressed.read_u64::<LE>().unwrap(),
                            decompressed.read_u64::<LE>().unwrap(),
                        )
                    })
                    .collect(),
                rpak_number: (0..header.patches_num)
                    .map(|_| decompressed.read_u16::<LE>().unwrap())
                    .collect(),
            })
        } else {
            None
        };

        let starpak_start = decompressed.stream_position()?;
        let starpak = string_from_buf(&mut decompressed);

        let starpak_skipped = starpak_start + header.starpak_len as u64;
        decompressed.seek(SeekFrom::Start(starpak_skipped))?;
        let starpak_opt = if header.starpak_opt_len != 0 {
            let tmp = string_from_buf(&mut decompressed);
            match tmp.len() {
                0 => None,
                _ => Some(tmp),
            }
        } else {
            None
        };

        let starpak_opt_skipped = starpak_skipped + header.starpak_opt_len as u64;
        decompressed.seek(SeekFrom::Start(starpak_opt_skipped))?;
        let sections = SectionDesc::parse(&mut decompressed, header.sections_num)?;

        let sections_skipped = starpak_opt_skipped + (16 * header.sections_num as u64);
        decompressed.seek(SeekFrom::Start(sections_skipped))?;
        let data_chunks = DataChunk::parse(&mut decompressed, header.data_chunks_num)?;

        let data_chunks_skipped = sections_skipped + (12 * header.data_chunks_num as u64);
        // unk54 aka "where descriptors are" here (8)
        decompressed.seek(SeekFrom::Start(data_chunks_skipped))?;
        let descriptors: Vec<(u32, u32)> = (0..header.num_descriptors)
            .map(|_| {
                (
                    decompressed.read_u32::<LE>().unwrap(),
                    decompressed.read_u32::<LE>().unwrap(),
                )
            })
            .collect();

        let unk54_skipped = data_chunks_skipped + (8 * header.num_descriptors as u64);
        // parsing files is moved so we can get juicy file offsets

        let file_entries_skipped = unk54_skipped + (0x50 * header.num_files as u64);
        // unk5c here (8)
        decompressed.seek(SeekFrom::Start(file_entries_skipped))?;
        let relationship: Vec<(u32, u32)> = (0..header.relationship)
            .map(|_| {
                (
                    decompressed.read_u32::<LE>().unwrap(),
                    decompressed.read_u32::<LE>().unwrap(),
                )
            })
            .collect();

        let unk5c_skipped = file_entries_skipped + (8 * header.relationship as u64);
        // unk60 here (4)
        decompressed.seek(SeekFrom::Start(unk5c_skipped))?;
        let unk60: Vec<u32> = (0..header.unk60)
            .map(|_| decompressed.read_u32::<LE>().unwrap())
            .collect();

        let unk60_skipped = unk5c_skipped + (4 * header.unk60 as u64);
        // unk64 here (4)
        decompressed.seek(SeekFrom::Start(unk60_skipped))?;
        let unk64: Vec<u32> = (0..header.unk64)
            .map(|_| decompressed.read_u32::<LE>().unwrap())
            .collect();

        let unk64_skipped = unk60_skipped + (4 * header.unk64 as u64);
        // unk68 here (1)
        decompressed.seek(SeekFrom::Start(unk64_skipped))?;
        let unk68: Vec<u8> = (0..header.unk68)
            .map(|_| decompressed.read_u8().unwrap())
            .collect();

        let unk68_skipped = unk64_skipped + header.unk68 as u64;
        // unk6c here (16)
        decompressed.seek(SeekFrom::Start(unk68_skipped))?;
        let unk6c: Vec<(u64, u64)> = (0..header.unk6c)
            .map(|_| {
                (
                    decompressed.read_u64::<LE>().unwrap(),
                    decompressed.read_u64::<LE>().unwrap(),
                )
            })
            .collect();

        let unk6c_skipped = unk68_skipped + (16 * header.unk6c as u64);
        // unk70 here (24)
        decompressed.seek(SeekFrom::Start(unk6c_skipped))?;
        let unk70: Vec<(u64, u64, u64)> = (0..header.unk70)
            .map(|_| {
                (
                    decompressed.read_u64::<LE>().unwrap(),
                    decompressed.read_u64::<LE>().unwrap(),
                    decompressed.read_u64::<LE>().unwrap(),
                )
            })
            .collect();

        let unk70_skipped = unk6c_skipped + (24 * header.unk70 as u64);

        // populate seek array
        let mut seeks = vec![0u64; header.data_chunks_num as usize];
        if header.data_chunks_num > 0 {
            seeks[0] = unk70_skipped;
            if header.data_chunks_num > 1 {
                for i in 1..header.data_chunks_num as usize {
                    seeks[i] = seeks[i - 1] + data_chunks[i - 1].size as u64;
                }
            }
        }

        // populate files array
        decompressed.seek(SeekFrom::Start(unk54_skipped))?;
        let files: Vec<Rc<dyn crate::FileEntry>> = (0..header.num_files)
            .map(|_| {
                // if this ever produces an error - "I cri" (C)
                let generic = FileGeneric::read(&mut decompressed, &seeks).unwrap();
                Rc::new(generic) as Rc<dyn crate::FileEntry>
            })
            .collect();

        // We don't parse data just yet?...
        Ok(Self {
            header,
            decompressed: Rc::new(RefCell::new(decompressed)),
            data_start: unk70_skipped,

            starpak,
            starpak_opt,
            files,
            sections,
            data_chunks,

            seeks,

            descriptors,
            descriptors_guid: relationship,
            unk60,
            unk64,
            unk68,
            unk6c,
            unk70,

            patch_shit,
        })
    }

    pub fn parse_files(&mut self, options: &ParseFileOptions) -> Result<(), crate::RPakError> {
        // this is retarded...
        let files_with_errors: Result<Vec<_>, _> = self
            .files
            .iter()
            .map(
                |file_ref| -> Result<Rc<dyn crate::FileEntry>, crate::RPakError> {
                    if let Some(generic) = file_ref.as_any().downcast_ref::<FileGeneric>() {
                        // TODO: macros, mb?
                        Ok(match generic.extension.as_str() {
                            "txtr" => {
                                if options.txtr {
                                    Rc::new(filetypes::txtr::Texture::ctor(
                                        &mut *self.get_decompressed(),
                                        &self.seeks,
                                        generic.clone(),
                                    )?)
                                } else {
                                    file_ref.clone()
                                }
                            }
                            "matl" => {
                                if options.matl {
                                    Rc::new(filetypes::matl::Material::ctor(
                                        &mut *self.get_decompressed(),
                                        &self.seeks,
                                        generic.clone(),
                                    )?)
                                } else {
                                    file_ref.clone()
                                }
                            }
                            "ui" => {
                                if options.rui {
                                    Rc::new(filetypes::rui::RUI::ctor(
                                        &mut *self.get_decompressed(),
                                        &self.seeks,
                                        generic.clone(),
                                    )?)
                                } else {
                                    file_ref.clone()
                                }
                            }
                            "uimg" => {
                                if options.uimg {
                                    Rc::new(filetypes::uimg::UImg::ctor(
                                        &mut *self.get_decompressed(),
                                        &self.seeks,
                                        generic.clone(),
                                    )?)
                                } else {
                                    file_ref.clone()
                                }
                            }
                            "uiia" => {
                                if options.uiia {
                                    Rc::new(filetypes::uiia::UIImage::ctor(
                                        &mut *self.get_decompressed(),
                                        &self.seeks,
                                        generic.clone(),
                                    )?)
                                } else {
                                    file_ref.clone()
                                }
                            }
                            "dtbl" => {
                                if options.dtbl {
                                    Rc::new(filetypes::dtbl::DataTable::ctor(
                                        &mut *self.get_decompressed(),
                                        &self.seeks,
                                        generic.clone(),
                                    )?)
                                } else {
                                    file_ref.clone()
                                }
                            }
                            "stgs" => {
                                if options.stgs {
                                    Rc::new(filetypes::stgs::Settings::ctor(
                                        &mut *self.get_decompressed(),
                                        &self.seeks,
                                        (*generic).clone(),
                                    )?)
                                } else {
                                    file_ref.clone()
                                }
                            }
                            "stlt" => {
                                if options.stlt {
                                    Rc::new(filetypes::stlt::SettingsLayout::ctor(
                                        &mut *self.get_decompressed(),
                                        &self.seeks,
                                        (*generic).clone(),
                                    )?)
                                } else {
                                    file_ref.clone()
                                }
                            }
                            "mdl_" => {
                                if options.rmdl {
                                    Rc::new(filetypes::rmdl::Model::ctor(
                                        &mut *self.get_decompressed(),
                                        &self.seeks,
                                        (*generic).clone(),
                                    )?)
                                } else {
                                    file_ref.clone()
                                }
                            }
                            "arig" => {
                                if options.arig {
                                    Rc::new(filetypes::arig::AnimationRig::ctor(
                                        &mut *self.get_decompressed(),
                                        &self.seeks,
                                        (*generic).clone(),
                                    )?)
                                } else {
                                    file_ref.clone()
                                }
                            }
                            "Ptch" => {
                                if options.patch {
                                    Rc::new(filetypes::ptch::Patch::ctor(
                                        &mut *self.get_decompressed(),
                                        &self.seeks,
                                        (*generic).clone(),
                                    )?)
                                } else {
                                    file_ref.clone()
                                }
                            }
                            _ => file_ref.clone(),
                        })
                    } else {
                        Ok(file_ref.clone())
                    }
                },
            )
            .collect();

        match files_with_errors {
            Err(v) => Err(v),
            Ok(v) => {
                self.files = v;
                Ok(())
            }
        }
    }
}
