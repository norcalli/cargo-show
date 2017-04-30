
//! `cargo show`

// these are broken over many line to make rustfmt happy
#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations)]
#![deny(trivial_casts, trivial_numeric_casts, unsafe_code)]
#![deny(unstable_features, unused_qualifications)]

extern crate g_k_crates_io_client as crates_io;
extern crate docopt;
extern crate rustc_serialize;

use std::fmt;
use std::process;
use std::io::Write;
use rustc_serialize::json;

// from: http://stackoverflow.com/a/27590832
macro_rules! println_stderr(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

static DEFAULT: &'static str = "https://crates.io/";

static USAGE: &'static str = r"
Usage:
    cargo show [options] <crate-name>...
    cargo show (-h|--help)
    cargo show --version

Options:
    --json                  Print the JSON response.
    -h --help               Show this help page.
    --version               Show version.

Display a metadata for a create at crates.io.
";


/// Docopt input args.
#[derive(Debug, RustcDecodable)]
struct Args {
    /// `crate-name`
    arg_crate_name: Vec<String>,
    /// `--version`
    flag_version: bool,
    /// `--json`
    flag_json: bool,
}

/// crate metadata to print
#[derive(Debug, RustcDecodable)]
pub struct CrateMetadata {
    // in response.crate
    created_at: String,
    description: Option<String>,
    documentation: Option<String>,
    downloads: u64,
    homepage: Option<String>,
    id: String,
    keywords: Vec<String>,
    license: Option<String>,
    max_version: String,
    name: String,
    repository: Option<String>,
    updated_at: String,
    versions: Vec<u64>, // also top level keywords and versions arrays of objects
}


impl fmt::Display for CrateMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {



        write!(f,
               "---
id: {id}
name: {name}
description: {description}
documentation: \
                {documentation}
homepage: {homepage}
repository: {repository}
max_version: \
                {max_version}
downloads: {downloads}
license: {license}
created: {created_at}
\
                updated: {updated_at}
",
               id = self.id,
               name = self.name,
               description = self.description.as_ref().unwrap_or(&"None".to_owned()),
               documentation = self.documentation.as_ref().unwrap_or(&"None".to_owned()),
               max_version = self.max_version,
               downloads = self.downloads,
               license = self.license.as_ref().unwrap_or(&"None".to_owned()),
               homepage = self.homepage.as_ref().unwrap_or(&"None".to_owned()),
               repository = self.repository.as_ref().unwrap_or(&"None".to_owned()),
               created_at = self.created_at,
               updated_at = self.updated_at)
    }
}

/// fetches and prints package metadata from crates.io
fn print_crate_metadata(crate_name: &str, as_json: bool) {
    let mut reg = crates_io::Registry::new(DEFAULT.to_string(), None);

    fn get_crate_json(crate_name: &str, body: &str) -> Option<String> {
        match json::Json::from_str(body) {
            Ok(data) => {
                if let Some(crate_json) = data.as_object().and_then(|j| j.get("crate")) {
                    // the entire object was decoded so encoding a part of it should be fine
                    json::encode(crate_json).ok()
                } else {
                    println_stderr!("No 'crate' field found in JSON data for {}.", crate_name);
                    None
                }
            }
            Err(e) => {
                println_stderr!("Error parsing JSON data for {}: {}", crate_name, e);
                None
            }
        }
    }

    match reg.get_crate_data(crate_name) {
        Ok(data) => {
            if as_json {
                println!("{}", data);
                return ();
            }

            if let Some(crate_json) = get_crate_json(crate_name, &*data) {
                let crate_meta: CrateMetadata = json::decode(&*crate_json)
                                                    .ok()
                                                    .expect("Unable to decode JSON to \
                                                             CrateMetadata.");

                print!("{}", crate_meta);
            }
        }
        Err(e) => {
            // e.g. crate name not found
            println_stderr!("Error fetching data for {}: {}", crate_name, e);
        }
    }
}

fn main() {
    let args = docopt::Docopt::new(USAGE)
                   .and_then(|d| d.decode::<Args>())
                   .unwrap_or_else(|err| err.exit());

    if args.flag_version {
        println!("cargo-show version {}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    for crate_name in &args.arg_crate_name {
        print_crate_metadata(crate_name, args.flag_json);
    }
}
