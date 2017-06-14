
//! `cargo show`

#![deny(missing_docs,
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unused_qualifications,
        unstable_features
)]

extern crate g_k_crates_io_client as crates_io;
extern crate docopt;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::fmt;
use std::process;
use std::io::Write;

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
#[derive(Debug, Deserialize)]
struct Args {
    /// `crate-name`
    arg_crate_name: Vec<String>,
    /// `--version`
    flag_version: bool,
    /// `--json`
    flag_json: bool,
}

/// crate metadata to print
#[derive(Debug, Serialize, Deserialize)]
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

/// crate metadata HTTP response
#[derive(Debug, Serialize, Deserialize)]
pub struct CrateMetaResponse {
    #[serde(rename="crate")]
    crate_data: CrateMetadata
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

    fn get_crate_metadata(crate_name: &str, body: &str) -> Option<CrateMetadata> {
        let resp_json: Result<CrateMetaResponse, _> = serde_json::from_str(body);
        match resp_json {
            Ok(data) => {
                Some(data.crate_data)
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

            match get_crate_metadata(crate_name, &*data) {
                Some(crate_meta) => print!("{}", crate_meta),
                None => {}
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
                   .and_then(|d| d.deserialize::<Args>())
                   .unwrap_or_else(|err| err.exit());

    if args.flag_version {
        println!("cargo-show version {}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    for crate_name in &args.arg_crate_name {
        print_crate_metadata(crate_name, args.flag_json);
    }
}
