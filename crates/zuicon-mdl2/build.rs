// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use inflections::Inflect;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use zu_util::icon::{get_svg_inner, need_update_with_name, TEMPLATE_FILE};

const SVG_DIR: &str = "fluentui/packages/react-icons-mdl2/src/components";
const LIB_HEADER: &str = r"// Auto Generated! DO NOT EDIT!

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]
#![allow(clippy::multiple_crate_versions)]

";

fn map_filename(name: String) -> String {
    let names = ["Box", "Option", "Type", "Move"];
    if names.contains(&name.as_str()) {
        return format!("{name}Icon");
    }
    name
}

fn build_icons() -> Result<Vec<(String, String)>, io::Error> {
    let mut module_names = Vec::new();
    let svg_extension = OsStr::new("tsx");

    for entry in fs::read_dir(SVG_DIR)? {
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
        let stem_str = stem_str.replace("Icon", "");
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
            r#"mod {module_name};
pub use {module_name}::{node_name};

"#
        );
        lib_file.write_all(line.as_bytes())?;
    }
    drop(lib_file);

    Ok(())
}

fn fetch_and_update_repo() {
    if fs::exists("icons").unwrap_or_default() {
        Command::new("git")
            .arg("pull")
            .current_dir("icons")
            .output()
            .expect("Failed to update fluentui repo");
    } else {
        Command::new("git")
            .arg("clone")
            .arg("https://github.com/microsoft/fluentui")
            .output()
            .expect("Failed to clone fluentui repo");
    }
}

fn main() {
    // Check ZU_ICON_UPDATE=mdl2 environment.
    if need_update_with_name("mdl2") {
        // Fetch fluentui repo first: `git clone https://github.com/microsoft/fluentui`
        fetch_and_update_repo();
        rebuild_icons().unwrap();
    }
}
