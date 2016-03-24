## cargo-show

[![crates.io version](https://img.shields.io/crates/v/cargo-show.svg)](https://img.shields.io/crates/v/cargo-show.svg)
[![Build Status](https://travis-ci.org/g-k/cargo-show.svg?branch=master)](https://travis-ci.org/g-k/cargo-show)
[![Build status](https://ci.appveyor.com/api/projects/status/m9cf5vhft7qwisas?svg=true)](https://ci.appveyor.com/project/g-k/cargo-show)

Prints package metadata like pip show, apt-cache show, npm view, gem query, etc.

To install:

```
$ cargo install cargo-show
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading cargo-show v0.1.0
   Compiling winapi-build v0.1.1
   Compiling rustc-serialize v0.3.18
   Compiling pkg-config v0.3.8
   Compiling strsim v0.3.0
... 
grab a cup of coffee 
...
contemplate life
...
more coffee
...
Installing /Users/greg/.multirust/toolchains/stable/cargo/bin/cargo-show
$
```

Usage:

```
$ cargo show --help
Usage:
    cargo show [options] <crate-name>...
    cargo show (-h|--help)
    cargo show --version

Options:
    --token=<token>         Use this crates.io API token.
    --json                  Print the JSON response.
    -h --help               Show this help page.
    --version               Show version.

Display a metadata for a create at crates.io.
```

To print package metadata:

```
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
downloads: 336
license: MPL-2.0
created: 2014-12-04T23:41:05Z
updated: 2015-12-11T23:55:55Z
```

To print JSON:

``` 
$ cargo show --json serde | cut -b '1-120'
{"crate":{"created_at":"2014-12-05T20:20:39Z","description":"A generic serialization/deserialization framework","documen
```

To rename the command if you're used to other package managers:

```
$ cd /usr/local/bin/  # or someplace in path
$ ln $(which cargo-show) cargo-flizblorp  # needs to be a hardlink
$ cargo --list | grep fliz
    flizblorp
```
