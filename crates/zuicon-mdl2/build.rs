// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use inflections::Inflect;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use zu_util::icon::{get_svg_inner, need_update_with_name, TEMPLATE_FILE};

const SVG_DIR: &str = "fluentui/packages/react-icons-mdl2/src/components";
const LIB_HEADER: &str = r###"// Auto Generated! DO NOT EDIT!

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]

"###;

fn map_filename(name: &str) -> String {
    let names = vec!["box", "option", "type"];
    if names.contains(&name) {
        return format!("icon-{name}");
    }
    name.to_string()
}

fn build_icons() -> Result<Vec<(String, String)>, io::Error> {
    let mut module_names = Vec::new();
    let mut dir = PathBuf::new();
    dir.push(SVG_DIR);

    let svg_extension = OsStr::new("tsx");

    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension() != Some(svg_extension) {
            println!("Ignore non svg file {path:?}");
            continue;
        }

        let stem = path.file_stem().unwrap();
        let stem_str = stem.to_str().unwrap();
        let stem_str = map_filename(stem_str);
        let data_name = &stem_str;
        let node_name = stem_str.to_pascal_case();
        let module_name = stem_str.to_snake_case();
        let mut rs_filepath = PathBuf::new();
        rs_filepath.push("src");
        rs_filepath.push(&module_name);
        rs_filepath.set_extension("rs");

        let svg_content = fs::read_to_string(&path).unwrap();
        let markup = get_svg_inner(&svg_content).unwrap();
        let rs_content = TEMPLATE_FILE
            .replace("NODE_NAME", &node_name)
            .replace("DATA_NAME", data_name)
            .replace("MARKUP", markup);

        fs::write(rs_filepath, rs_content).unwrap();
        module_names.push((module_name, node_name));
    }

    module_names.sort();
    Ok(module_names)
}

fn rebuild_icons() -> Result<(), Box<dyn Error>> {
    let module_names = build_icons()?;

    let mut lib_file = File::create("src/lib.rs")?;
    lib_file.write_all(LIB_HEADER.as_bytes())?;
    for (module_name, node_name) in &module_names {
        let line = format!(
            r#"#[cfg(feature = "{node_name}")]
mod {module_name};
#[cfg(feature = "{node_name}")]
pub use {module_name}::{node_name};

"#
        );
        lib_file.write_all(line.as_bytes())?;
    }
    drop(lib_file);

    let mut cargo_file = OpenOptions::new().append(true).open("Cargo.toml")?;
    for (_module_name, node_name) in module_names.iter() {
        let line = format!("{node_name} = []\n");
        cargo_file.write_all(line.as_bytes())?;
    }
    drop(cargo_file);

    Ok(())
}

fn main() {
    // Check ZU_ICON_UPDATE=mdl2 environment.
    if need_update_with_name("mdl2") {
        // Fetch fluentui repo first: `git clone https://github.com/microsoft/fluentui`
        rebuild_icons().unwrap();
    }
}
