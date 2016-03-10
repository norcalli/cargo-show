## cargo-show

[![Build Status](https://travis-ci.org/g-k/cargo-show.svg?branch=master)](https://travis-ci.org/g-k/cargo-show)

Prints package metadata like pip show, apt-cache show, npm view, gem query, etc.

Usage:

```sh
$ cargo run show -- --help
     Running `target/debug/cargo-show show serde docopt`
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
$ cargo run show -- serde docopt
     Running `target/debug/cargo-show show serde docopt`
---
id: serde
name: serde
description: A generic serialization/deserialization framework
documentation: https://serde-rs.github.io/serde/serde/serde/index.html
homepage: None
repository: https://github.com/serde-rs/serde
max_version: 0.7.0
downloads: 184449
license: MIT/Apache-2.0
created: 2014-12-05T20:20:39Z
updated: 2016-02-27T05:29:34Z
---
id: docopt
name: docopt
description: Command line argument parsing.
documentation: http://burntsushi.net/rustdoc/docopt/
homepage: https://github.com/docopt/docopt.rs
repository: https://github.com/docopt/docopt.rs
max_version: 0.6.78
downloads: 134241
license: Unlicense/MIT
created: 2014-11-20T22:44:11Z
updated: 2015-12-16T00:01:56Z
 $ cargo run show -- --json serde | cut -b '1-120'
     Running `target/debug/cargo-show show --json serde`
{"crate":{"created_at":"2014-12-05T20:20:39Z","description":"A generic serialization/deserialization framework","documen
```
