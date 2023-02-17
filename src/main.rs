//! `cargo show`

extern crate docopt;
extern crate g_k_crates_io_client as crates_io;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use itertools::Itertools;
use std::collections::BTreeMap;
use std::fmt::{self, Display};
use std::process;

static DEFAULT: &'static str = "https://crates.io/";

static USAGE: &'static str = r"
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
    /// `--dependencies`
    flag_dependencies: bool,
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
    #[serde(skip)]
    features: BTreeMap<String, Vec<String>>,
    #[serde(skip)]
    versions_: Vec<String>,
}

/// crate metadata HTTP response
#[derive(Debug, Serialize, Deserialize)]
pub struct CrateMetaResponse {
    #[serde(rename = "crate")]
    crate_data: CrateMetadata,
    versions: Vec<CrateVersion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrateVersion {
    features: BTreeMap<String, Vec<String>>,
    #[serde(rename = "num")]
    version: String,
}

/// crate dependency
#[derive(Debug, Deserialize)]
pub struct CrateDependency {
    // in response.dependencies
    /// The dependent crate's ID.
    #[serde(rename = "crate_id")]
    id: String,

    /// The required version of the dependent crate (semver).
    req: String,

    /// The dependency type.
    ///
    /// Can be: `normal`, `dev` (examples, tests), `build` (deps required by `build.rs`).
    kind: CrateDependencyKind,

    /// The dependency is optional.
    optional: bool,
    // ...
}

/// The dependendy type.
#[derive(Debug, Deserialize, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum CrateDependencyKind {
    /// Just a regular dependency library.
    Normal,
    /// A dependency used in examples or tests.
    Dev,
    /// A dependency used in `build.rs`.
    Build,
}

/// crate dependencies HTTP response
#[derive(Debug, Deserialize)]
pub struct CrateDependencyResponse {
    dependencies: Vec<CrateDependency>,
}

impl fmt::Display for CrateMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self {
            created_at,
            description,
            documentation,
            downloads,
            homepage,
            id,
            keywords,
            license,
            max_version,
            name,
            repository,
            updated_at,
            versions: _,
            versions_,
            features,
        } = self;
        let description = description.or_display("None");
        let documentation = documentation.or_display("None");
        let license = license.or_display("None");
        let homepage = homepage.or_display("None");
        let repository = repository.or_display("None");
        let keywords = keywords.iter().format(", ");
        let feat_max_key_len = features.keys().map(|s| s.len()).max().unwrap_or(0);
        let it = features.keys().chunks(4);
        let feat = it
            .into_iter()
            .map(|chunk| chunk.map(|s| format!("{s:<feat_max_key_len$}")).format(" "))
            .format("\n          ");
        let feat_details = features
            .iter()
            .map(|(k, v)| format!("  {k:>feat_max_key_len$} = {}", v.iter().format(",")))
            // .map(|(k, v)| format!("  {k:>feat_max_key_len$} = {v:?}"))
            .format("\n");
        let versions = versions_.iter().take(5).format(", ");
        let def_feat = features
            .get("default")
            .map(|s| s.iter().format(","))
            .into_or_display("--");
        write!(
            f,
            "---\n\
            id: {id}\n\
            name: {name}\n\
            description: {description}\n\
            documentation: {documentation}\n\
            homepage: {homepage}\n\
            repository: {repository}\n\
            max_version: {max_version}\n\
            latest_versions: {versions}\n\
            downloads: {downloads}\n\
            license: {license}\n\
            created: {created_at}\n\
            updated: {updated_at}\n\
            keywords: {keywords}\n\
            default_features: {def_feat}\n\
            features: {feat}\n\
            features_detail:\n{feat_details}",
        )
    }
}

impl fmt::Display for CrateDependency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.id, self.req)?;
        let isdev = self.kind != CrateDependencyKind::Normal;
        let isopt = self.optional;
        if isdev || isopt {
            write!(f, " (")?;
            if isdev {
                write!(f, "{}", self.kind)?;
            }
            if isdev && isopt {
                write!(f, ", ")?;
            }
            if isopt {
                write!(f, "opt")?;
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl fmt::Display for CrateDependencyKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            CrateDependencyKind::Normal => "normal",
            CrateDependencyKind::Dev => "dev",
            CrateDependencyKind::Build => "build",
        };
        write!(f, "{}", name)
    }
}

/// fetches and prints package metadata from crates.io
fn print_crate_metadata(crate_name: &str, as_json: bool, with_deps: bool) -> Result<(), String> {
    let mut req = crates_io::Registry::new(DEFAULT.to_string(), None);

    let response = req
        .get_crate_data(crate_name)
        .map_err(|e| format!("Error fetching data for {}: {}", crate_name, e))?;

    let meta: Result<CrateMetaResponse, _> = serde_json::from_str(&response)
        .map_err(|e| format!("Error parsing JSON data for {}: {}", crate_name, e));

    let mut meta = meta?;
    meta.crate_data
        .features
        .clone_from(&meta.versions[0].features);
    meta.crate_data
        .versions_
        .extend(meta.versions.iter().map(|s| s.version.clone()));
    let crate_data = meta.crate_data;

    if as_json && with_deps {
        let response = req
            .get_crate_dependencies(&crate_data.id, &crate_data.max_version)
            .map_err(|e| format!("Error fetching dependencies for {}: {}", crate_name, e))?;
        println!("{}", response);
        return Ok(());
    }

    if as_json {
        println!("{}", response);
        return Ok(());
    }

    // print crate's metadata
    println!("{}", crate_data);

    if with_deps {
        println!("dependencies:");

        let response = req
            .get_crate_dependencies(&crate_data.id, &crate_data.max_version)
            .map_err(|e| format!("Error fetching dependencies for {}: {}", crate_name, e))?;

        let deps: Result<CrateDependencyResponse, _> = serde_json::from_str(&response)
            .map_err(|e| format!("Error patcing JSON dependencies for {}: {}", crate_name, e));

        let mut deps = deps?.dependencies;
        deps.sort_by(|a, b| a.kind.cmp(&b.kind));
        for dependency in deps {
            println!("{}", dependency);
        }
    }

    Ok(())
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
        let r = print_crate_metadata(crate_name, args.flag_json, args.flag_dependencies);
        if let Err(e) = r {
            eprintln!("{}", e);
        }
    }
}

pub struct OrDisplay<T: Display, U>(Option<T>, U);

impl<T: Display, U: Display> Display for OrDisplay<T, U> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            Some(t) => t.fmt(fmt),
            None => self.1.fmt(fmt),
        }
    }
}

pub trait OrDisplayExt<T: Display> {
    fn or_display<U: Display>(&self, u: U) -> OrDisplay<&T, U>;

    fn into_or_display<U: Display>(self, u: U) -> OrDisplay<T, U>;
}

impl<T: Display> OrDisplayExt<T> for Option<T> {
    fn or_display<U: Display>(&self, u: U) -> OrDisplay<&T, U> {
        OrDisplay(self.as_ref(), u)
    }

    fn into_or_display<U: Display>(self, u: U) -> OrDisplay<T, U> {
        OrDisplay(self, u)
    }
}
