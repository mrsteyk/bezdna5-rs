use std::io::{Read, Seek, SeekFrom};

use byteorder::{ReadBytesExt, LE};

use crate::FileEntry;

#[derive(Debug)]
pub enum UIImageError {
    IOError(std::io::Error),
}

impl From<std::io::Error> for UIImageError {
    fn from(v: std::io::Error) -> Self {
        Self::IOError(v)
    }
}

impl From<UIImageError> for crate::RPakError {
    fn from(item: UIImageError) -> Self {
        Self::FileTypeParseError((
            "uiia",
            Box::new(match item {
                UIImageError::IOError(v) => Self::IOError(v),
                _ => crate::RPakError::Shiz("Unknown error!".to_string()),
            }),
        ))
    }
}

#[derive(Debug)]
pub struct UIImage {
    pub generic: super::FileGeneric,

    pub flags: u32,

    pub swizzle_512: u32,
    pub swizzle_1024: u32,
    pub block_skip: u32,

    pub image_data: Vec<u8>,
}

impl crate::FileEntry for UIImage {
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
        None
    }

    fn get_star_off(&self) -> Option<u64> {
        self.generic.get_star_off()
    }
    fn get_star_opt_off(&self) -> Option<u64> {
        self.generic.get_star_opt_off()
    }

    fn get_ext(&self) -> &str {
        "uiia"
    }
}

// argument must be 512b in size
pub fn swizzle_512_d(i: &[u8]) -> [u8; 512] {
    let mut ret = [0u8; 512];
    const sz: usize = 128 / 8;

    // I didn't type this by hand, don't worry lmfao
    ret[0..0 + sz].copy_from_slice(&i[0..0 + sz]);
    ret[16..16 + sz].copy_from_slice(&i[32..32 + sz]);
    ret[32..32 + sz].copy_from_slice(&i[128..128 + sz]);
    ret[48..48 + sz].copy_from_slice(&i[160..160 + sz]);

    ret[64..64 + sz].copy_from_slice(&i[16..16 + sz]);
    ret[80..80 + sz].copy_from_slice(&i[48..48 + sz]);
    ret[96..96 + sz].copy_from_slice(&i[144..144 + sz]);
    ret[112..112 + sz].copy_from_slice(&i[176..176 + sz]);

    ret[128..128 + sz].copy_from_slice(&i[64..64 + sz]);
    ret[144..144 + sz].copy_from_slice(&i[96..96 + sz]);
    ret[160..160 + sz].copy_from_slice(&i[192..192 + sz]);
    ret[176..176 + sz].copy_from_slice(&i[224..224 + sz]);

    ret[192..192 + sz].copy_from_slice(&i[80..80 + sz]);
    ret[208..208 + sz].copy_from_slice(&i[112..112 + sz]);
    ret[224..224 + sz].copy_from_slice(&i[208..208 + sz]);
    ret[240..240 + sz].copy_from_slice(&i[240..240 + sz]);

    ret[256..256 + sz].copy_from_slice(&i[256..256 + sz]);
    ret[272..272 + sz].copy_from_slice(&i[288..288 + sz]);
    ret[288..288 + sz].copy_from_slice(&i[384..384 + sz]);
    ret[304..304 + sz].copy_from_slice(&i[416..416 + sz]);

    ret[320..320 + sz].copy_from_slice(&i[272..272 + sz]);
    ret[336..336 + sz].copy_from_slice(&i[304..304 + sz]);
    ret[352..352 + sz].copy_from_slice(&i[400..400 + sz]);
    ret[368..368 + sz].copy_from_slice(&i[432..432 + sz]);

    ret[384..384 + sz].copy_from_slice(&i[320..320 + sz]);
    ret[400..400 + sz].copy_from_slice(&i[352..352 + sz]);
    ret[416..416 + sz].copy_from_slice(&i[448..448 + sz]);
    ret[432..432 + sz].copy_from_slice(&i[480..480 + sz]);

    ret[448..448 + sz].copy_from_slice(&i[336..336 + sz]);
    ret[464..464 + sz].copy_from_slice(&i[368..368 + sz]);
    ret[480..480 + sz].copy_from_slice(&i[464..464 + sz]);
    ret[496..496 + sz].copy_from_slice(&i[496..496 + sz]);

    ret
}

// Faster-ish version of swizzle_1024
pub fn swizzle_1024_f(a: &[u8]) -> [u8; 1024] {
    let mut ret = [0u8; 1024];
    const sz: usize = 128 / 8;
    const arr: [usize; 8] = [0, 0x20, 0x80, 0xa0, 0x200, 0x220, 0x280, 0x2A0];
    const load_idx: [usize; 8] = [
        0, 16, 64, 80, // ---
        256, 272, 320, 336,
    ];
    const store_idx: [usize; 8] = [0, 16, 32, 48, 64, 80, 96, 112];

    for i in 0..2usize {
        for j in 0..4usize {
            for l in 0..8usize {
                let store = (j * 128) + (i * 512) + store_idx[l];
                let load = arr[j + i * 2] + load_idx[l];
                ret[store..store + sz].copy_from_slice(&a[load..load + sz]);
            }
        }
    }

    ret
}

impl UIImage {
    pub fn ctor<R: Read + Seek + ReadBytesExt>(
        cursor: &mut R,
        seeks: &[u64],
        generic: super::FileGeneric,
    ) -> Result<Self, UIImageError> {
        // This parsing is the most barebone I think, a guy who came up with this smoked a ton of good stuff I guess

        // cursor.seek(SeekFrom::Start(generic.get_desc_off()))?;

        let start_pos = generic.get_desc_off();

        // Start of the description is fucking useless
        cursor.seek(SeekFrom::Start(generic.get_desc_off() + 0x12))?;

        // All we care, literally?
        let flags = cursor.read_u32::<LE>()?;

        // Now the juicy fucking data...
        // Start is pUseless
        cursor.seek(SeekFrom::Start(generic.get_data_off().unwrap() + 0x24))?;

        let swizzled = (flags & 3) == 3;

        let width = cursor.read_u16::<LE>()?;
        let height = cursor.read_u16::<LE>()?;

        // true data offset
        let data_offset = {
            let id = cursor.read_u32::<LE>()?;
            let off = cursor.read_u32::<LE>()?;
            seeks[id as usize] + off as u64
        };

        let blocks_w = (width + 0x1d) / 0x1f;
        let blocks_h = (height + 0x1d) / 0x1f;
        let blocks_num = blocks_w as u32 * blocks_h as u32;

        let [swizzle_512, swizzle_1024] = if swizzled {
            cursor.seek(SeekFrom::Start(data_offset))?;

            let mut ret = [0u32; 2];
            for _ in 0..blocks_num {
                let dword = cursor.read_u32::<LE>()?;
                // I wasn't kidding... who came up with this?
                // Is this something I don't understand????
                if (dword & 0xC0000000) == 0x40000000 {
                    let idx = ((dword - 0x40000000) >> 24) as usize;
                    ret[idx] += 1;
                }
            }

            ret
        } else {
            [0, 0]
        };

        let true_blockskip = 4 * ((blocks_num + 1) & 0xFFFFFFFE);
        cursor.seek(SeekFrom::Start(data_offset + true_blockskip as u64))?;
        let mut image_data = vec![0u8; swizzle_512 as usize * 512 + swizzle_1024 as usize * 1024];
        cursor.read_exact(&mut image_data)?;

        if swizzled {
            for i in 0..swizzle_512 as usize {
                let pos = i * 512;
                let swizzled = swizzle_512_d(&image_data[pos..pos + 512]);
                assert_ne!(&swizzled, &image_data[pos..pos + 512], "brih swizzle 512");
                image_data[pos..pos + 512].copy_from_slice(&swizzled);
            }

            for i in 0..swizzle_1024 as usize {
                let pos = (swizzle_512 as usize * 512) + (i * 1024);
                let swizzled = swizzle_1024_f(&image_data[pos..pos + 1024]);
                assert_ne!(&swizzled, &image_data[pos..pos + 1024], "brih swizzle 1024");
                image_data[pos..pos + 1024].copy_from_slice(&swizzled);
            }
        }

        Ok(Self {
            generic,

            flags,

            swizzle_512,
            swizzle_1024,
            block_skip: true_blockskip,

            image_data,
        })
    }
}
