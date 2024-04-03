/*! Copies `AVIF`-marked EXIF metadata from one file on top another, saving the outcome in third. */

extern crate riff_io;

use std::env;
use std::io::Result;
use std::process::exit;

use std::fs::File;

use riff_io::{Chunk, DataOwned, DataRef, Entry, List, RiffFile};

fn find_chunk<'a>(
    e: &'a Entry<DataRef>,
    pred: &impl Fn(&Chunk<DataRef>) -> bool,
) -> Option<&'a Chunk<DataRef>> {
    match e {
        Entry::Chunk(c) => {
            if pred(&c) {
                Some(c)
            } else {
                None
            }
        }
        Entry::List(l) => l.children.iter().find_map(|e| find_chunk(e, pred)),
    }
}

fn walk(
    e: Entry<DataOwned>,
    tf: &impl Fn(Entry<DataOwned>) -> Entry<DataOwned>,
) -> Entry<DataOwned> {
    let e = match e {
        Entry::List(l) => Entry::List(List {
            children: l.children.into_iter().map(|e| walk(e, tf)).collect(),
            ..l
        }),
        c => c,
    };
    tf(e)
}

fn main() -> Result<()> {
    if env::args().len() < 2 {
        println!("Usage: copy [source] [desy]");
        exit(-1);
    }

    let filename_src = env::args().nth(1).unwrap();
    let filename_base = env::args().nth(2).unwrap();
    let filename_new = env::args().nth(3).unwrap();

    let file = RiffFile::open(&filename_src)?;
    let toplevel = file.read_file()?;
    let avif_chunk = find_chunk(&toplevel, &|c: &Chunk<DataRef>| {
        let b = c.bytes(file.bytes());
        &c.id == b"strd" && &b[..4] == b"AVIF"
    })
    .unwrap();
    let avif_chunk = avif_chunk.to_owned(file.bytes());
    let avif_chunk = Entry::Chunk(avif_chunk);
    let name_chunk = find_chunk(&toplevel, &|c: &Chunk<DataRef>| {
        let b = c.bytes(file.bytes());
        &c.id == b"strn" && &b[..4] == b"FUJI"
    })
    .unwrap();
    let name_chunk = name_chunk.to_owned(file.bytes());
    let name_chunk = Entry::Chunk(name_chunk);

    let file = RiffFile::open(&filename_base)?;
    let toplevel = file.read_file()?.to_owned(file.bytes());

    let new_toplevel = walk(toplevel, &|e| match e {
        Entry::List(l) if &l.list_type == b"strl" => {
            let mut l = l.clone();
            let avif_chunk = avif_chunk.clone();
            let position = l.children.iter().position(|e| match e {
                Entry::Chunk(c) if &c.id == b"strd" => true,
                _ => false,
            });
            match position {
                Some(p) => {
                    l.children[p] = avif_chunk;
                }
                None => {
                    let pos = l.children.iter().position(|e| match e {
                        Entry::Chunk(c) if &c.id == b"strf" => true,
                        _ => false,
                    });
                    let pos = pos.map(|i| i + 1).unwrap_or(l.children.len());
                    l.children.insert(pos, avif_chunk);
                }
            };
            let name_chunk = name_chunk.clone();
            let position = l.children.iter().position(|e| match e {
                Entry::Chunk(c) if &c.id == b"strn" => true,
                _ => false,
            });
            match position {
                Some(p) => {
                    l.children[p] = name_chunk;
                }
                None => {
                    let pos = l.children.iter().position(|e| match e {
                        Entry::Chunk(c) if &c.id == b"strd" => true,
                        _ => false,
                    });
                    let pos = pos.map(|i| i + 1).unwrap_or(l.children.len());
                    l.children.insert(pos, name_chunk);
                }
            };
            Entry::List(l)
        }
        other => other,
    });

    let mut outfile = File::create(&filename_new)?;

    new_toplevel.write(&mut outfile)?;

    Ok(())
}
