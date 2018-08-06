#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;
extern crate base64;
extern crate failure;
extern crate serde_json;
extern crate slurp;
extern crate walkdir;

use failure::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(Debug, StructOpt)]
#[structopt(name = "consul-convert")]
struct Args {
    /// Path to directory with consul keys
    #[structopt(name = "src", long = "src", parse(from_os_str))]
    src: PathBuf,

    /// Path to export consul data file
    #[structopt(name = "dest", long = "dest", parse(from_os_str))]
    dest: PathBuf,

    /// Path to export consul data file
    #[structopt(name = "key-path", long = "key-path")]
    key_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Record {
    key: String,
    value: String,
    flags: u8,
}

fn record(src: &Path, path: &str) -> Record {
    let key = src.file_name().unwrap();
    let value = base64::encode(&slurp::read_all_to_string(src).unwrap());
    println!("Record {}", &key.to_str().unwrap());
    Record {
        key: format!("{}/{}", path, key.to_str().unwrap()),
        value,
        flags: 0,
    }
}

fn walk_dir(dir: &Path, dest: &Path, key_path: &str) -> Result<(), Error> {
    let all: Vec<Record> = WalkDir::new(dir)
        .min_depth(1)
        .into_iter()
        .filter_map(|entry| match entry {
            Ok(ref entry) if entry.file_type().is_file() => Some(record(&entry.path(), key_path)),
            _ => None,
        })
        .collect();

    let out = serde_json::to_string(&all)?;

    let mut file = File::create(dest)?;
    file.write_all(out.as_bytes())?;

    Ok(())
}

fn main() -> Result<(), Error> {
    let args = Args::from_args();

    walk_dir(&args.src, &args.dest, &args.key_path)?;

    Ok(())
}
