use std::{fs::File, io::{BufReader, Read, Seek, SeekFrom}, path::Path};

use rpak::{apex::RPakHeader, decomp::decompress};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Invalid usage!")
    } else {
        let path = Path::new(&args[1]);
        let file_stem = path.file_stem().unwrap().to_str().unwrap();
        let file = File::open(path).unwrap();
        println!("stem: {}", file_stem);
        let mut cursor = BufReader::new(file);

        let header = RPakHeader::read(&mut cursor).unwrap();

        cursor.seek(SeekFrom::Start(0)).unwrap();
        let mut vec = Vec::<u8>::new();
        cursor.read_to_end(&mut vec).unwrap();

        let decompressed = if header.is_compressed() {
            let mut d = decompress(
                &mut vec,
                header.size_decompressed as usize,
                rpak::HEADER_SIZE_APEX,
            ).unwrap();
            // TODO: fix header...
            d[..rpak::HEADER_SIZE_APEX].clone_from_slice(&vec[..rpak::HEADER_SIZE_APEX]);
            d
        } else {
            unimplemented!()
        };

        print!("Writing decompressed... ");
        std::fs::write(args[1].to_owned() + ".raw", decompressed).unwrap();
        println!("ok");

        println!("{:#?}", header);
    }
}
