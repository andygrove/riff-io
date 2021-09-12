# riff-io

Rust crate for reading [Resource Interchange File Format](https://en.wikipedia.org/wiki/Resource_Interchange_File_Format) (RIFF) files.

## Features

- Provides access to file metadata containing the file structure (lists and chunks) 
- Ability to read bytes from any position in the file
- Tested on Windows, Mac, and Linux. 
- Uses memory-mapped files for efficiency.
- Write support may be added in the future.

## Example

```bash,no_run
cargo run --example view example.AVI
```

Sample output:

```text,no_run
File type: AVI 
File size: 168081400
LIST 'hdrl'
  CHUNK 'avih'
  LIST 'strl'
    CHUNK 'strh'
    CHUNK 'strf'
    CHUNK 'indx'
  CHUNK ''
CHUNK ''
CHUNK ''
LIST 'movi'
  CHUNK 'ix00'
  CHUNK '00db'
  CHUNK 'JUNK'
  CHUNK '00db'
  ...
  CHUNK 'JUNK'
  CHUNK '00db'
CHUNK '[f8, 5, 0, 0]'
```

## Resources

- [AVI RIFF File Reference](https://docs.microsoft.com/en-us/previous-versions//ms779636(v=vs.85)?redirectedfrom=MSDN)