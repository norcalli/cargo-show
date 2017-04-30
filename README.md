## cargo-show

[![crates.io version](https://img.shields.io/crates/v/cargo-show.svg)](https://img.shields.io/crates/v/cargo-show.svg)
[![Build Status](https://travis-ci.org/g-k/cargo-show.svg?branch=master)](https://travis-ci.org/g-k/cargo-show)
[![Build status](https://ci.appveyor.com/api/projects/status/m9cf5vhft7qwisas?svg=true)](https://ci.appveyor.com/project/g-k/cargo-show)

Prints package metadata like pip show, apt-cache show, npm view, gem query, etc.

To install:

```sh
$ cargo install cargo-show
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading cargo-show v0.4.0
  Installing cargo-show v0.4.0
   Compiling utf8-ranges v1.0.0
   Compiling strsim v0.6.0
   Compiling libc v0.2.22
...

   Compiling g-k-crates-io-client v0.8.1
   Compiling cargo-show v0.4.0
    Finished release [optimized] target(s) in 94.37 secs
  Installing /Users/greg/.cargo/bin/cargo-show
$
```

Usage:

```sh
$ cargo show --help
Usage:
    cargo show [options] <crate-name>...
    cargo show (-h|--help)
    cargo show --version

Options:
    --json                  Print the JSON response.
    -h --help               Show this help page.
    --version               Show version.

Display a metadata for a create at crates.io.
```

To print package metadata:

```sh
$ cargo show webrender servo
Error fetching data for webrender: cannot find crate
---
id: servo
name: servo
description: Parked non-servo thing
documentation: None
homepage: None
repository: None
max_version: 0.0.1
downloads: 966
license: MPL-2.0
created: 2014-12-04T23:41:05Z
updated: 2015-12-11T23:55:55Z
```

To print JSON:

```json
$ cargo show --json serde | cut -b '1-120'
{"categories":[{"category":"Encoding","crates_cnt":42,"created_at":"2017-01-17T19:13:05Z","description":"Encoding and/or
```

To rename the command if you're used to other package managers:

```sh
$ cd /usr/local/bin/  # or someplace in path
$ ln $(which cargo-show) cargo-flizblorp  # needs to be a hardlink
$ cargo --list | grep fliz
    flizblorp
```

### Contributors

[@leoschwarz](https://github.com/leoschwarz)
