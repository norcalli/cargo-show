## cargo-show

[![crates.io version](https://img.shields.io/crates/v/cargo-show.svg)](https://img.shields.io/crates/v/cargo-show.svg)
[![Build Status](https://travis-ci.org/g-k/cargo-show.svg?branch=master)](https://travis-ci.org/g-k/cargo-show)
[![Build status](https://ci.appveyor.com/api/projects/status/m9cf5vhft7qwisas?svg=true)](https://ci.appveyor.com/project/g-k/cargo-show)

Prints package metadata like pip show, apt-cache show, npm view, gem query, etc.

To install:

```sh
$ cargo install cargo-show
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading cargo-show v0.5.5
  Installing cargo-show v0.5.5
   Compiling cc v1.0.18
   Compiling pkg-config v0.3.12
   Compiling unicode-xid v0.0.4
...
   Compiling g-k-crates-io-client v0.16.0
   Compiling docopt v0.8.3
   Compiling cargo-show v0.5.5
    Finished release [optimized] target(s) in 3m 27.44s
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
    -L --dependencies       Print the crate's dependencies as well.
    -h --help               Show this help page.
    --version               Show version.

Display a metadata for a create at crates.io.
```

To print package metadata:

```sh
$ cargo show webrender servo
Error fetching data for webrender: received 404 not found response code
---
id: servo
name: servo
description: Parked non-servo thing
documentation: None
homepage: None
repository: None
max_version: 0.0.1
downloads: 1297
license: None
created: 2014-12-04T23:41:05.915728+00:00
updated: 2015-12-11T23:55:55.315022+00:00
```

To print JSON:

```json
$ cargo show --json serde | cut -b '1-120'
{"crate":{"id":"serde","name":"serde","updated_at":"2018-08-25T03:04:21.721448+00:00","versions":[105274,105095,104994,1
```

To print package metadata and direct dependencies (alternatively use `-L`):

```sh
$ cargo show --dependencies time
---
id: time
name: time
description: Utilities for working with time-related functions in Rust.

documentation: https://doc.rust-lang.org/time
homepage: https://github.com/rust-lang/time
repository: https://github.com/rust-lang/time
max_version: 0.1.40
downloads: 4211782
license: None
created: 2014-11-13T06:52:51.369245+00:00
updated: 2018-05-10T19:25:59.151982+00:00
dependencies:
libc ^0.2.1
redox_syscall ^0.1
winapi ^0.3.0
rustc-serialize ^0.3 (opt)
log ^0.4 (dev)
winapi ^0.3.0 (dev)
```


To print package metadata and direct dependencies as JSON:

```sh
$ cargo show --dependencies --json time | python -m json.tool | head -n25
{
    "dependencies": [
        {
            "crate_id": "libc",
            "default_features": true,
            "downloads": 0,
            "features": [],
            "id": 414415,
            "kind": "normal",
            "optional": false,
            "req": "^0.2.1",
            "target": null,
            "version_id": 92243
        },
        {
            "crate_id": "log",
            "default_features": true,
            "downloads": 0,
            "features": [],
            "id": 414417,
            "kind": "dev",
            "optional": false,
            "req": "^0.4",
            "target": null,
            "version_id": 92243
```


To rename the command if you're used to other package managers:

```sh
$ cd /usr/local/bin/  # or someplace in path
$ ln $(which cargo-show) cargo-flizblorp  # needs to be a hardlink
$ cargo --list | grep fliz
    flizblorp
```

### Maintainers

* [@g-k](https://github.com/g-k)
* [@pravic](https://github.com/pravic)

### Contributors

* [@g-k](https://github.com/g-k)
* [@leoschwarz](https://github.com/leoschwarz)
* [@pravic](https://github.com/pravic)
