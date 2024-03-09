extern crate riff_io;

use std::env;
use std::io::Result;
use std::process::exit;

use std::fs::File;

use riff_io::{RiffFile, Entry, List, DataOwned};

fn main() -> Result<()> {
    if env::args().len() < 2 {
        println!("Usage: copy [source] [desy]");
        exit(-1);
    }

    let filename_src = env::args().nth(1).unwrap();
    let filename_dst = env::args().nth(2).unwrap();

    let file = RiffFile::open(&filename_src)?;
    let mut outfile = File::create(&filename_dst)?;

    let entries = file.read_entries()?;
    let toplevel = Entry::<DataOwned>::List(List {
        fourcc: *b"RIFF",
        list_type: *b"AVI ",
        children: entries.into_iter()
            .map(|e| e.to_owned(file.bytes())).collect(),
    });
    toplevel.write(&mut outfile)?;

    Ok(())
}
