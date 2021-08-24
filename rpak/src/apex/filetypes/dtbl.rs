use std::io::{Read, Seek, SeekFrom};

use byteorder::{ReadBytesExt, LE};

use crate::FileEntry;

#[derive(Debug)]
pub enum DataTableError {
    IOError(std::io::Error),
}

impl From<std::io::Error> for DataTableError {
    fn from(v: std::io::Error) -> Self {
        Self::IOError(v)
    }
}

impl From<DataTableError> for crate::RPakError {
    fn from(item: DataTableError) -> Self {
        Self::FileTypeParseError((
            "dtbl",
            Box::new(match item {
                DataTableError::IOError(v) => Self::IOError(v),
                _ => crate::RPakError::Shiz("Unknown error!".to_string()),
            }),
        ))
    }
}

// desc
// column_num: u32 // 0x0 - column count
// row_num: u32 // 0x4 - row count
// columns_desc: Descriptor // 0x8 - columns (pad: u64, pad: u64, typ: u32, offset: u32) somewhere in there is a descriptor of name
// rows_desc: Descriptor // 0x10 - rows
// pad: u64 // 0x18
// elem_size: u32 // 0x20 - elem size
// pad: u32 // 0x24

#[derive(Debug)]
pub enum ColumnData {
    Bool(bool),              // 0
    Int(i32),                // 1
    Float(f32),              // 2
    Vector([f32; 3]),        // 3
    String(String),          // 4
    Asset(String),           // 5
    AssetNoPreCache(String), // 6

    Invalid(u32), // has ID
}

#[derive(Debug)]
pub struct Column {
    unk0_seek: u64, // kind of name desc...
    unk8: u64,      // some sort of hash...
    typ: u32,       // maps to ColumnData
    offset: u32,    // offset in row...
}

#[derive(Debug)]
pub struct DataTable {
    pub generic: super::FileGeneric,

    pub elem_size: u32,

    pub columns: Vec<Column>,

    // Or a hashmap???
    pub column_names: Vec<String>,
    pub data: Vec<Vec<ColumnData>>,

    pub unk18: u64,
    pub unk24: u32,
}

impl crate::FileEntry for DataTable {
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
        self.generic.get_data_off()
    }
    fn get_desc_size(&self) -> usize {
        self.generic.get_desc_size()
    }

    fn get_name(&self) -> Option<&str> {
        None // TODO
    }

    fn get_star_off(&self) -> Option<u64> {
        None // we know for sure
    }
    fn get_star_opt_off(&self) -> Option<u64> {
        None // we know for sure
    }

    fn get_ext(&self) -> &str {
        "dtbl"
    }
}

impl DataTable {
    pub fn ctor<R: Read + Seek + ReadBytesExt>(
        cursor: &mut R,
        seeks: &[u64],
        generic: super::FileGeneric,
    ) -> Result<Self, DataTableError> {
        cursor.seek(SeekFrom::Start(generic.get_desc_off()))?;

        assert_eq!(generic.get_desc_size(), 0x28, "Wha?");

        let start_pos = cursor.stream_position()?;

        let column_num = cursor.read_u32::<LE>()?;
        let row_num = cursor.read_u32::<LE>()?;

        let columns_seek = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        let rows_seek = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        let unk18 = cursor.read_u64::<LE>()?;
        let elem_size = cursor.read_u32::<LE>()?;
        let unk24 = cursor.read_u32::<LE>()?;

        assert_eq!(cursor.stream_position()? - start_pos, 0x28, "я датаеблан");

        // println!("{:X}[{:X}] | {:X} | {}x{} | {:X} | {:X}", generic.get_desc_off(), generic.get_desc_size(), start_pos, column_num, row_num, columns_seek, rows_seek);

        cursor.seek(SeekFrom::Start(columns_seek))?;
        let columns = {
            let mut ret = Vec::<Column>::with_capacity(column_num as usize);
            for _ in 0..column_num {
                let unk0_seek = {
                    let id = cursor.read_u32::<LE>()?;
                    let off = cursor.read_u32::<LE>()?;
                    seeks[id as usize] + off as u64
                };
                // TODO: s3 switch??? this is peak stupid tbh...
                // let unk8 = cursor.read_u64::<LE>()?;
                let typ = cursor.read_u32::<LE>()?;
                let offset = cursor.read_u32::<LE>()?;

                ret.push(Column {
                    unk0_seek,
                    unk8: 0,
                    typ,
                    offset,
                })
            }

            ret
        };

        let column_names: Vec<String> = columns
            .iter()
            .map(|f| {
                cursor.seek(SeekFrom::Start(f.unk0_seek)).unwrap();
                crate::util::string_from_buf(cursor)
            })
            .collect();
        let data = {
            let mut ret = Vec::<Vec<ColumnData>>::with_capacity(row_num as usize);
            for i in 0..row_num {
                let base_pos = rows_seek + (elem_size * i) as u64;
                let mut vec = Vec::<ColumnData>::with_capacity(column_num as usize);
                for column in &columns {
                    cursor
                        .seek(SeekFrom::Start(base_pos + column.offset as u64))
                        .unwrap();

                    let d = match column.typ {
                        0 => ColumnData::Bool(cursor.read_u32::<LE>()? != 0),
                        1 => ColumnData::Int(cursor.read_i32::<LE>()?),
                        2 => ColumnData::Float(cursor.read_f32::<LE>()?),
                        3 => {
                            let mut tmp = [0f32; 3];
                            cursor.read_f32_into::<LE>(&mut tmp)?;
                            ColumnData::Vector(tmp)
                        }
                        4 => {
                            let seek = {
                                let id = cursor.read_u32::<LE>()?;
                                let off = cursor.read_u32::<LE>()?;
                                seeks[id as usize] + off as u64
                            };
                            cursor.seek(SeekFrom::Start(seek))?;
                            ColumnData::String(crate::util::string_from_buf(cursor))
                        }
                        5 => {
                            let seek = {
                                let id = cursor.read_u32::<LE>()?;
                                let off = cursor.read_u32::<LE>()?;
                                seeks[id as usize] + off as u64
                            };
                            cursor.seek(SeekFrom::Start(seek))?;
                            ColumnData::Asset(crate::util::string_from_buf(cursor))
                        }
                        6 => {
                            let seek = {
                                let id = cursor.read_u32::<LE>()?;
                                let off = cursor.read_u32::<LE>()?;
                                seeks[id as usize] + off as u64
                            };
                            cursor.seek(SeekFrom::Start(seek))?;
                            ColumnData::AssetNoPreCache(crate::util::string_from_buf(cursor))
                        }
                        v => ColumnData::Invalid(v),
                    };
                    vec.push(d);
                }
                ret.push(vec);
            }

            ret
        };

        //println!("{:X?}", columns);

        Ok(Self {
            generic,

            elem_size,

            columns,
            column_names,

            data,

            unk18,
            unk24,
        })
    }
}
