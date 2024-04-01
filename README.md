# riff-io

[![crates.io](https://img.shields.io/crates/v/riff-io.svg)](https://crates.io/crates/riff-io)

Rust crate for reading 
[Resource Interchange File Format](https://en.wikipedia.org/wiki/Resource_Interchange_File_Format) (RIFF) files, such 
as [Audio Video Interleave](https://en.wikipedia.org/wiki/Audio_Video_Interleave) (AVI) 
and [Waveform Audio File Format](https://en.wikipedia.org/wiki/WAV) (WAV).

## Looking For New Maintainers

I am no longer maintaining this crate and would be happy to transfer it to new maintainers.

## Features

- Provides access to file metadata containing the file structure (lists and chunks) 
- Ability to read bytes from any position in the file
- Uses memory-mapped files for efficiency.
- Cross-platform: Tested on Windows, Mac, and Linux. 

## Non Features
 
- There is no write support yet although it may be added in the future.

## Example

The example shows the file structure of the specified RIFF file.

```bash,no_run
cargo run --example view example.AVI
```

Sample output:

```text,no_run
LIST 'hdrl'
  CHUNK 'avih' offset=32 size=56
  LIST 'strl'
    CHUNK 'strh' offset=108 size=56
    CHUNK 'strf' offset=172 size=1064
    CHUNK 'indx' offset=1244 size=32248
  LIST 'odml'
    CHUNK 'dmlh' offset=33512 size=248
CHUNK 'JUNK' offset=33768 size=12
LIST 'movi'
  CHUNK 'ix00' offset=33800 size=32248
  CHUNK '00db' offset=66056 size=3818112
  CHUNK 'JUNK' offset=3884176 size=368
  ...
  CHUNK '00db' offset=164261384 size=3818112
  CHUNK 'JUNK' offset=168079504 size=368
CHUNK 'idx1' offset=168079880 size=1528
```

## Resources

- [AVI RIFF File Reference](https://docs.microsoft.com/en-us/previous-versions//ms779636(v=vs.85)?redirectedfrom=MSDN)
