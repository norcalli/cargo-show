## cargo-show

[![crates.io version](https://img.shields.io/crates/v/cargo-show.svg)](https://img.shields.io/crates/v/cargo-show.svg)
[![Build Status](https://travis-ci.org/g-k/cargo-show.svg?branch=master)](https://travis-ci.org/g-k/cargo-show)
[![Build status](https://ci.appveyor.com/api/projects/status/m9cf5vhft7qwisas?svg=true)](https://ci.appveyor.com/project/g-k/cargo-show)

Prints package metadata like pip show, apt-cache show, npm view, gem query, etc.

To install:

```sh
$ cargo install cargo-show
    Updating registry `https://github.com/rust-lang/crates.io-index`
  Installing cargo-show v0.5.0
   Compiling percent-encoding v1.0.0
   Compiling strsim v0.6.0
   Compiling quote v0.3.15
...
   Compiling docopt v0.8.1
   Compiling cargo-show v0.5.0
    Finished release [optimized] target(s) in 104.96 secs
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
downloads: 1060
license: MPL-2.0
created: 2014-12-04T23:41:05Z
updated: 2015-12-11T23:55:55Z
```

To print JSON:

```json
$ cargo show --json serde | cut -b '1-120'
{"crate":{"id":"serde","name":"serde","updated_at":"2017-07-12T04:20:29Z","versions":[59405,58325,54218,53794,53565,5330
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
