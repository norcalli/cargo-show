#!/usr/bin/env bash

## Helper script to run cargo show and update the README.md
# run from project root

cat > README.md <<EOF
## cargo-show

[![crates.io version](https://img.shields.io/crates/v/cargo-show.svg)](https://img.shields.io/crates/v/cargo-show.svg)
[![Build Status](https://travis-ci.org/g-k/cargo-show.svg?branch=master)](https://travis-ci.org/g-k/cargo-show)
[![Build status](https://ci.appveyor.com/api/projects/status/m9cf5vhft7qwisas?svg=true)](https://ci.appveyor.com/project/g-k/cargo-show)

Prints package metadata like pip show, apt-cache show, npm view, gem query, etc.

To install:

\`\`\`
\$ cargo install cargo-show
    Updating registry \`https://github.com/rust-lang/crates.io-index\`
  Installing cargo-show v0.3.0
   Compiling glob v0.2.11
   Compiling num-traits v0.1.37
   Compiling winapi-build v0.1.1
...

   Compiling cargo v0.17.0
   Compiling docopt v0.7.0
   Compiling cargo-show v0.3.0
    Finished release [optimized] target(s) in 312.4 secs
  Installing /Users/greg/.cargo/bin/cargo-show
\$
\`\`\`

Usage:

\`\`\`
\$ cargo show --help
$(cargo show --help)
\`\`\`

To print package metadata:

\`\`\`
\$ cargo show webrender servo
$(cargo show webrender servo 2>&1)
\`\`\`

To print JSON:

\`\`\`
\$ cargo show --json serde | cut -b '1-120'
$(cargo show --json serde | cut -b '1-120')
\`\`\`

To rename the command if you're used to other package managers:

\`\`\`
\$ cd /usr/local/bin/  # or someplace in path
\$ ln \$(which cargo-show) cargo-flizblorp  # needs to be a hardlink
\$ cargo --list | grep fliz
    flizblorp
\`\`\`

### Contributors

[@leoschwarz](https://github.com/leoschwarz)
EOF


## if the readme changed cargo-show is broken or metadata from the
## example crates changed
git diff
