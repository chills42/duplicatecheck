# Purpose

DuplicateCheck will search a directory tree and list all duplicate files.  This can be useful when combining data from several sources.

# Building

This project is written in rust, and uses the [Cargo](https://crates.io/) build tool.

The following, run from the repo folder, will build the tool, placing the executable at `target\release\duplicatecheck'

```
cargo build --release
```

# Running

Simply run the executable from the root folder that you want to search within.

