extern crate riff_io;

use std::env;
use std::io::Result;
use std::process::exit;

use std::fs::File;

use riff_io::RiffFile;

fn main() -> Result<()> {
    if env::args().len() < 2 {
        println!("Usage: copy [source] [desy]");
        exit(-1);
    }

    let filename_src = env::args().nth(1).unwrap();
    let filename_dst = env::args().nth(2).unwrap();

    let file = RiffFile::open(&filename_src)?;
    let mut outfile = File::create(&filename_dst)?;

    let toplevel = file.read_file()?;
    toplevel.to_owned(file.bytes()).write(&mut outfile)?;

    Ok(())
}
