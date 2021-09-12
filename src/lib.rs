#![doc = include_str!("../README.md")]

use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{Error, Result};
use std::ops::Range;

use memmap::{Mmap, MmapOptions};

/// Four-character code
pub type FourCC = [u8; 4];

/// FourCC for 'RIFF'
const RIFF: FourCC = [0x52, 0x49, 0x46, 0x46];

/// FourCC for 'LIST'
const LIST: FourCC = [0x4c, 0x49, 0x53, 0x54];

/// Entry in a RIFF file, which can be a list or a chunk of data. Lists can be nested
#[derive(Debug)]
pub enum Entry {
    /// List can contain lists and chunks
    List(ListMeta),
    /// Chunks are leaf nodes
    Chunk(ChunkMeta),
}

/// Meta-data for a list
#[derive(Debug)]
pub struct ListMeta {
    /// four-character code list type
    pub list_type: FourCC,
    /// offset of data, in bytes
    pub data_offset: usize,
    /// length of data, in bytes
    pub data_size: usize,
    /// child entries, which can be a mix of lists and chunks
    pub children: Vec<Entry>,
}

/// Meta-data for a chunk of data
#[derive(Debug)]
pub struct ChunkMeta {
    /// four-character code chunk id
    pub chunk_id: FourCC,
    /// offset of data
    pub data_offset: usize,
    /// length of data, in bytes
    pub chunk_size: usize,
    /// Number of data bytes, which can be greater than chunk_size due to padding
    pub data_size: usize,
}

pub struct RiffFile {
    mmap: Mmap,
    /// four-character code file type, such as 'AVI '.
    file_type: FourCC,
    file_size: usize,
}

/// Resource Interchange File Format
impl RiffFile {
    /// Open a RIFF file
    pub fn open(filename: &str) -> Result<Self> {
        let file = File::open(&filename)?;
        let metadata = fs::metadata(&filename)?;

        let len = metadata.len() as usize;

        let mmap = unsafe { MmapOptions::new().map(&file)? };

        // https://docs.microsoft.com/en-us/previous-versions//ms779636(v=vs.85)?redirectedfrom=MSDN
        // 'RIFF' fileSize fileType (data)
        // where 'RIFF' is the literal FOURCC code 'RIFF', fileSize is a 4-byte value giving the size of
        // the data in the file, and fileType is a FOURCC that identifies the specific file type. The
        // value of fileSize includes the size of the fileType FOURCC plus the size of the data that
        // follows, but does not include the size of the 'RIFF' FOURCC or the size of fileSize. The file
        // data consists of chunks and lists, in any order.

        let header = &mmap[0..12];
        let four_cc = parse_fourcc(&header[0..4]);
        if four_cc != RIFF {
            return Err(Error::new(ErrorKind::Other, "Incorrect RIFF header"));
        }

        let file_size = parse_size(&header[4..8]) as usize;
        if len != file_size as usize + 8_usize {
            return Err(Error::new(ErrorKind::Other, "Incorrect file length"));
        }

        let mut file_type: FourCC = Default::default();
        file_type.copy_from_slice(&header[8..12]);

        Ok(Self {
            mmap,
            file_type,
            file_size,
        })
    }

    pub fn file_type(&self) -> &FourCC {
        &self.file_type
    }

    pub fn file_size(&self) -> usize {
        self.file_size
    }

    pub fn read_root(&self) -> Result<Vec<Entry>> {
        let mut pos = 12;
        let mut entries = vec![];
        while pos < self.file_size {
            let entry = self.read_entry(pos)?;
            pos = match &entry {
                Entry::List(list) => list.data_offset + list.data_size,
                Entry::Chunk(chunk) => chunk.data_offset + chunk.data_size,
            };
            entries.push(entry);
        }
        Ok(entries)
    }

    pub fn read_bytes(&self, range: Range<usize>) -> &[u8] {
        &self.mmap[range]
    }

    fn read_entry(&self, offset: usize) -> Result<Entry> {
        // read fourCC and size
        let header = &self.mmap[offset..offset + 8];
        let pos = offset + 8_usize;
        let four_cc = parse_fourcc(&header[0..4]);
        let size = parse_size(&header[4..8]) as usize;

        if four_cc == LIST {
            self.read_list(pos, size)
        } else {
            self.read_chunk(four_cc, pos, size)
        }
    }

    fn read_list(&self, offset: usize, list_size: usize) -> Result<Entry> {
        // 'LIST' listSize listType [listData]

        // where 'LIST' is the literal FOURCC code 'LIST', listSize is a 4-byte value giving
        // the size of the list, listType is a FOURCC code, and listData consists of chunks or
        // lists, in any order.
        //
        // The value of listSize includes the size of listType plus the
        // size of listData; it does not include the 'LIST' FOURCC or the size of listSize.
        // read fourCC and size

        let header = &self.mmap[offset..offset + 4];
        let data_offset = offset + 4_usize;
        let list_type = parse_fourcc(&header[0..4]);

        let end = list_size - 4;

        let mut children = vec![];

        let mut pos = data_offset;
        while pos < end {
            let entry = self.read_entry(pos)?;
            pos = match &entry {
                Entry::List(list) => list.data_offset + list.data_size,
                Entry::Chunk(chunk) => chunk.data_offset + chunk.data_size,
            };
            children.push(entry);
        }

        Ok(Entry::List(ListMeta {
            data_offset,
            list_type,
            data_size: list_size,
            children,
        }))
    }

    fn read_chunk(&self, chunk_id: FourCC, offset: usize, chunk_size: usize) -> Result<Entry> {
        // ckID ckSize ckData
        //
        // where ckID is a FOURCC that identifies the data contained in the chunk,
        //
        // ckSize is
        // a 4-byte value giving the size of the data in ckData, and ckData is zero or more
        // bytes of data. The data is always padded to nearest WORD boundary.
        //
        // ckSize gives
        // the size of the valid data in the chunk; it does not include the padding, the
        // size of ckID, or the size of ckSize.

        let data_size = chunk_size + chunk_size % 2;

        Ok(Entry::Chunk(ChunkMeta {
            data_offset: offset,
            chunk_id,
            chunk_size,
            data_size,
        }))
    }
}

fn parse_fourcc(header: &[u8]) -> FourCC {
    let mut four_cc: FourCC = Default::default();
    four_cc.copy_from_slice(header);
    four_cc
}

fn parse_size(array: &[u8]) -> u32 {
    (array[0] as u32)
        + ((array[1] as u32) << 8)
        + ((array[2] as u32) << 16)
        + ((array[3] as u32) << 24)
}
