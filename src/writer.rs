// Copyright 2018 Francesco Bertolaccini
// With later changes
use crate::{Entry, List, Chunk, DataOwned};

use std::io::Write;
use std::io::Seek;

impl Entry<DataOwned> {
  pub fn write<T>(&self, writer: &mut T) -> std::io::Result<u64>
      where T: Seek + Write {
    match &self {
      &Entry::Chunk(Chunk{ id, chunk_size, data}) => {
        let data = &data.0;
        if data.len() as u64 > u32::MAX as u64 {
          use std::io::{Error, ErrorKind};
          return Err(Error::new(ErrorKind::InvalidData, "Data too big"));
        }

        let len = *chunk_size as u32;
        writer.write_all(id)?;
        writer.write_all(&len.to_le_bytes())?;
        writer.write_all(&data)?;
        Ok(8 + data.len() as u64)
      }
      &Entry::List(list@List{fourcc, list_type, children}) => {
        writer.write_all(fourcc)?;
        let len = list.bytes_len() as u32 - 8;
        writer.write_all(&len.to_le_bytes())?;
        writer.write_all(list_type)?;
        for child in children {
          child.write(writer)?;
        }

        Ok(8 + len as u64)
      }
    }
  }
}
