//! You can write a wit file to a json by running:
//!
//!     cargo run --bin write_to_json foo.wit
//!
use anyhow::{Error, Result};
use serde_json;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use wit_parser::*;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let file_name = &mut "".to_string();
    if args.len() > 1 {
        *file_name = args[1].clone();
    } else {
        println!("No filename provided");
    }

    let unresolved = UnresolvedPackageGroup::parse_file(Path::new(file_name))?;

    let mut resolve = Resolve::default();
    resolve.push_group(unresolved)?;

    // Serialize the struct to a JSON string
    let json_string = serde_json::to_string_pretty(&resolve)?;

    file_name.push_str(".json");

    // Create and open the file in write mode
    let mut file = File::create(file_name)?;

    // Write the JSON string to the file
    file.write_all(json_string.as_bytes())?;

    Ok(())
}
