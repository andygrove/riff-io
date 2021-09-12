extern crate riff_io;

use std::env;
use std::io::Result;
use std::process::exit;
use std::str;

use riff_io::{Entry, FourCC, RiffFile};

fn main() -> Result<()> {
    if env::args().len() < 2 {
        println!("Usage: view [filename]");
        exit(-1);
    }

    let filename = env::args().nth(1).unwrap();

    let file = RiffFile::open(&filename)?;

    println!("File type: {}", format_fourcc(file.file_type()));
    println!("File size: {}", file.file_size());

    let entries = file.read_root()?;
    for entry in &entries {
        show_entry(entry, 0)?;
    }

    Ok(())
}

fn show_entry(entry: &Entry, indent: usize) -> Result<()> {
    print!("{}", String::from("  ").repeat(indent));
    match entry {
        Entry::Chunk(chunk) => {
            println!("CHUNK '{}'", format_fourcc(&chunk.chunk_id));
        }
        Entry::List(list) => {
            println!("LIST '{}'", format_fourcc(&list.list_type));
            for entry in &list.children {
                show_entry(entry, indent + 1)?;
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
