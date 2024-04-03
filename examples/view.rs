extern crate riff_io;

use std::env;
use std::io::Result;
use std::process::exit;
use std::str;

use riff_io::{Entry, FourCC, RiffFile, DataRef};

fn main() -> Result<()> {
    if env::args().len() < 2 {
        println!("Usage: view [filename]");
        exit(-1);
    }

    let filename = env::args().nth(1).unwrap();

    let file = RiffFile::open(&filename)?;

    println!("File type: {}", format_fourcc(file.file_type()));
    println!("File size: {}", file.file_size());

    show_entry(&file.read_file()?, 0, file.bytes())?;
    Ok(())
}

fn show_entry(entry: &Entry<DataRef>, indent: usize, file: &[u8]) -> Result<()> {
    print!("{}", String::from("  ").repeat(indent));
    match entry {
        Entry::Chunk(chunk) => {
            println!(
                "CHUNK '{}' offset={} size={}",
                format_fourcc(&chunk.id),
                chunk.data.offset,
                chunk.chunk_size
            );
            let mut d = [0,0,0,0];
            d.copy_from_slice(&chunk.bytes(file)[..4]);
            println!("{:?}", format_fourcc(&d));
        }
        Entry::List(list) => {
            println!("{} '{}', size={}", format_fourcc(&list.fourcc), format_fourcc(&list.list_type), list.bytes_len());
            for entry in &list.children {
                show_entry(entry, indent + 1, file)?;
            }
        }
    }

    Ok(())
}

fn format_fourcc(value: &FourCC) -> String {
    match str::from_utf8(value) {
        Ok(s) => s.to_string(),
        _ => format!("{:x?}", value),
    }
}
