#[macro_use]
extern crate derivative;

mod binding;

mod util;

mod consts;
use std::{
    any::Any,
    cell::RefMut,
    collections::HashMap,
    fmt::Debug,
    io::{Cursor, Read, Seek, SeekFrom},
    rc::Rc,
};

pub use chrono::NaiveDateTime;

use byteorder::{ReadBytesExt, LE};
pub use consts::*;

mod hashing;
pub use hashing::hash;

pub mod decomp;

pub mod apex;

// dynamic shit per ext
/// This trait represents what every file in the game should have and I know the meaning of it and it's also very useful
pub trait FileEntry: Debug {
    fn as_any(&self) -> &dyn Any;

    // getters
    /// Internal name hash used by the lookup functions
    fn get_guid(&self) -> u64;
    /// File's extension (no longer than 4 characters)
    fn get_ext(&self) -> &str;
    /// Implemented per file extension, not all types have it
    fn get_name(&self) -> Option<&str>; // ergh?
    /// Offset from the start of the file of so called description.
    /// Every file should have this field
    fn get_desc_off(&self) -> u64;
    /// Predicted unaligned size of so called description
    fn get_desc_size(&self) -> usize;
    // mb easier handling??? TODO: is this better?
    /// Offset of data associated with this file, not every file has it
    fn get_data_off(&self) -> Option<u64>;
    /// StarPak offset of file's data. Not all files have it.
    fn get_star_off(&self) -> Option<u64>;
    /// Apex specific optional StarPak offset. Not all files have it.
    fn get_star_opt_off(&self) -> Option<u64>; // TF2 won't implement this rather...
}

// this is static across all(2) games
/// Section Descriptor of RPak
#[derive(Debug)]
pub struct SectionDesc {
    /// Weird section type, use `&0b111` to find out real type?
    pub section_type: u32,
    /// Align byte that sometimes gets used
    pub align_byte: u32,
    /// Unaligned(?) sum of sizes of all data chunk sharing the type of `section_type`
    pub size_unaligned: u64,
}

impl SectionDesc {
    pub fn read<R: Read + Seek + ReadBytesExt>(cursor: &mut R) -> Result<Self, std::io::Error> {
        Ok(Self {
            section_type: cursor.read_u32::<LE>()?,
            align_byte: cursor.read_u32::<LE>()?,
            size_unaligned: cursor.read_u64::<LE>()?,
        })
    }

    /// Parses an array of `SectionDesc`s of known size from a good-enough buffer
    ///
    /// # Arguments
    ///
    /// * `cursor` - A buffer which implements Read, Seek, ReadBytesExt
    /// * `size` - Known size of the array, u16 is the game's limitation @ the moment...
    pub fn parse<R: Read + Seek + ReadBytesExt>(
        cursor: &mut R,
        size: u16,
    ) -> Result<Vec<Self>, std::io::Error> {
        let mut ret = Vec::with_capacity(size as usize);
        for _ in 0..size {
            ret.push(Self::read(cursor)?);
        }

        Ok(ret)
    }
}

#[derive(Debug)]
pub struct DataChunk {
    /// Section ID in `SectionDesc` array
    pub section_id: u32,
    /// Align byte that sometimes gets used
    pub align_byte: u32,
    /// Size of the chunk
    pub size: u32,
}

impl DataChunk {
    pub fn read<R: Read + Seek + ReadBytesExt>(cursor: &mut R) -> Result<Self, std::io::Error> {
        Ok(Self {
            section_id: cursor.read_u32::<LE>()?,
            align_byte: cursor.read_u32::<LE>()?,
            size: cursor.read_u32::<LE>()?,
        })
    }

    /// Parses an array of `DataChunk`s of known size from a good-enough buffer
    ///
    /// # Arguments
    ///
    /// * `cursor` - A buffer which implements Read, Seek, ReadBytesExt
    /// * `size` - Known size of the array, u16 is the game's limitation @ the moment...
    pub fn parse<R: Read + Seek + ReadBytesExt>(
        cursor: &mut R,
        size: u16,
    ) -> Result<Vec<Self>, std::io::Error> {
        let mut ret = Vec::with_capacity(size as usize);
        for _ in 0..size {
            ret.push(Self::read(cursor)?);
        }

        Ok(ret)
    }
}

/// This trait represents generic workflow with all RPakFiles
pub trait RPakFile: Debug {
    fn as_any(&self) -> &dyn Any;

    fn is_compressed(&self) -> bool;
    fn should_lla(&self) -> bool;

    fn get_decompressed(&self) -> RefMut<Cursor<Vec<u8>>>;

    fn get_version(&self) -> RPakVersion;
    fn get_sections_desc(&self) -> &Vec<SectionDesc>;
    fn get_files(&self) -> &Vec<Rc<dyn FileEntry>>;
    fn get_data_chunks(&self) -> &Vec<DataChunk>;
}

/// Takes rpak file and parses it into a viable format
pub fn parse_rpak<R: Read + Seek + ReadBytesExt>(
    cursor: &mut R,
) -> Result<Box<dyn RPakFile>, RPakError> {
    match get_rpak_version_cursor(cursor) {
        // RPakVersion::TF2 => unimplemented!(),
        RPakVersion::APEX => match apex::RPakFile::read(cursor) {
            Ok(file) => Ok(Box::new(file)),
            Err(err) => Err(err),
        },
        ver => Err(RPakError::InvalidVersion(ver as u16)),
    }
}

/// Quick parses the header for the known RPak version
pub fn get_rpak_version(file: Vec<u8>) -> RPakVersion {
    if file.len() < 88 {
        RPakVersion::Invalid
    } else {
        match file[4] as u16 + ((file[5] as u16) << 8) {
            // 7 => RPakVersion::TF2,
            8 => RPakVersion::APEX,
            _ => RPakVersion::Invalid,
        }
    }
}

/// Quick parses the header of the buffer for the known RPak version
pub fn get_rpak_version_cursor<R: Read + Seek + ReadBytesExt>(cursor: &mut R) -> RPakVersion {
    match cursor.seek(SeekFrom::Start(4)) {
        Ok(_) => match cursor.read_u16::<LE>() {
            Ok(v) => match v {
                // 7 => RPakVersion::TF2,
                8 => RPakVersion::APEX,
                _ => RPakVersion::Invalid,
            },
            _ => RPakVersion::Invalid,
        },
        _ => RPakVersion::Invalid,
    }
}

fn generate_pair(string: &str) -> (u64, String) {
    // idk how to not clone...
    let guid = hashing::hash(string.to_owned());
    (guid, string.to_owned())
}

/// Try to populate the predicted names `HashMap`
#[allow(unused_variables)]
pub fn predict_names(rpak_file: &dyn RPakFile, file_stem: String) -> HashMap<u64, String> {
    let mut ret = HashMap::<u64, String>::new();

    if file_stem.ends_with("_loadscreen") {
        // LoadScreen's texture and atlas
        let mapname = &file_stem[0..file_stem.len() - 11];
        let atlas = generate_pair(&format!(
            "ui_image_atlas/loadscreens/{}_widescreen.rpak",
            mapname
        ));
        let texture = generate_pair(&format!(
            "texture/ui_atlas/loadscreens/{}_widescreen.rpak",
            mapname
        ));

        ret.insert(atlas.0, atlas.1);
        ret.insert(texture.0, texture.1);
    } else if file_stem.starts_with("mp_") {
        // Map shit...
        // TODO: _genN and _muN shit...
        let mapname = &file_stem;
        // let mapname_short = ;

        // use short here
        let map_zones = generate_pair(&format!("datatable/map_zones/zones_{}.rpak", mapname));
        ret.insert(map_zones.0, map_zones.1);

        // use full here
        let props = generate_pair(&format!("maps/{}_props.rpak", mapname));
        ret.insert(props.0, props.1);

        // use full here
        let cubemaps_hdr = generate_pair(&format!("texture/maps/{}/cubemaps_hdr.rpak", mapname));
        ret.insert(cubemaps_hdr.0, cubemaps_hdr.1);

        // use full here
        let cubemap_ambients = generate_pair(&format!(
            "texture_extension/maps/{}/cubemaps_hdr/cubemap_ambients.rpak",
            mapname
        ));
        ret.insert(cubemap_ambients.0, cubemap_ambients.1);

        // use full here
        let overviews_atlas = generate_pair(&format!("ui_image_atlas/overviews/{}.rpak", mapname));
        let overviews_texture =
            generate_pair(&format!("texture/ui_atlas/overviews/{}.rpak", mapname));
        ret.insert(overviews_atlas.0, overviews_atlas.1);
        ret.insert(overviews_texture.0, overviews_texture.1);
    }

    rpak_file
        .get_files()
        .iter()
        .for_each(|f| match f.get_ext() {
            "matl" => {
                if !f.get_name().unwrap().ends_with("_colpass") {
                    // spc = exp
                    // ilm = glw
                    for i in &["col", "nml", "opa", "ilm", "spc", "glw", "lim"] {
                        let pair = generate_pair(&format!("{}_{}.rpak", f.get_name().unwrap(), i));
                        ret.insert(pair.0, pair.1);
                    }
                }
            }
            "ui" => {
                let pair = generate_pair(&format!("ui/{}.rpak", f.get_name().unwrap()));
                ret.insert(pair.0, pair.1);
            }
            _ => {}
        });

    // comics...
    for i in 6..=11 {
        for j in 0..32 {
            let pair = generate_pair(&format!("datatable/comic/season{}/page{}.rpak", i, j));
            ret.insert(pair.0, pair.1);

            let pair = generate_pair(&format!(
                "settings/itemflav/quest_comic/s{:02}/page{:02}.rpak",
                i, j
            ));
            ret.insert(pair.0, pair.1);
        }
    }

    ret
}
