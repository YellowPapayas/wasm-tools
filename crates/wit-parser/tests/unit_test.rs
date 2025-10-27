//! You can run this test suite with:
//!
//!     cargo test --test all
//!
//! An argument can be passed as well to filter, based on filename, which test
//! to run
//!
//!     cargo test --test all foo.wit
use anyhow::{Error, Result};
use std::path::Path;
use wit_parser::*;

#[test]
fn main() -> Result<(), Error> {
    let unresolved = UnresolvedPackageGroup::parse_file(Path::new("tests/strings.wit"))?;

    let mut resolve = Resolve::default();
    let package_id = resolve.push_group(unresolved)?;
    let package = &resolve.packages[package_id];
    println!("Package: {}", package.name);
    println!("Docs: {:?}", package.docs);

    for (name, interface_id) in &package.interfaces {
        let interface = &resolve.interfaces[*interface_id];
        println!("  Interface: {}", name);
        println!("  Docs: {:?}", interface.docs);
        println!("  Annotations: {:?}", interface.annotations);

        for (type_name, type_id) in &interface.types {
            let mut type_kind = &TypeDefKind::Unknown;
            if let Some(type_def) = resolve.types.get(*type_id) {
                type_kind = &type_def.kind;
            }
            println!("      Type: {}, {:?}", type_name, type_kind);
        }

        for (func_name, func) in &interface.functions {
            println!("      Function: {}", func_name);

            for (param_name, param_type) in &func.params {
                println!("          Param: {}, {:?}", param_name, param_type);
            }
            println!("          Returns: {:?}", func.result);
        }
    }

    for (name, world_id) in &package.worlds {
        let world = &resolve.worlds[*world_id];
        println!("  World: {}", name);
        for (import_name, _) in &world.imports {
            let import_print = match import_name {
                WorldKey::Name(str) => str,
                WorldKey::Interface(id) => {
                    if let Some(test_print) = &resolve.interfaces[*id].name {
                        test_print
                    } else {
                        &String::new()
                    }
                }
            };
            println!("      Import: {:?}", import_print);
        }
        for (export_name, _) in &world.exports {
            let export_print = match export_name {
                WorldKey::Name(str) => str,
                WorldKey::Interface(id) => {
                    if let Some(test_print) = &resolve.interfaces[*id].name {
                        test_print
                    } else {
                        &String::new()
                    }
                }
            };
            println!("      Export: {:?}", export_print);
        }
    }

    Err(Error::msg("error"))
    //Ok(())
}
