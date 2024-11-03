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

const SVG_DIR: &str = "icons/packages/icons-svg/svg";
const LIB_HEADER: &str = r"// Auto Generated! DO NOT EDIT!

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]

";

fn build_icons(folder: &str) -> Result<Vec<(String, String)>, io::Error> {
    let mut module_names = Vec::new();
    let mut dir = PathBuf::new();
    dir.push(SVG_DIR);
    dir.push(folder);

    let svg_extension = OsStr::new("svg");

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
        let data_name = stem_str;
        let node_name = stem_str.to_pascal_case();
        let module_name = stem_str.to_snake_case().to_owned();
        let mut rs_filepath = PathBuf::new();
        rs_filepath.push("src");
        rs_filepath.push(folder);
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

    // Write to module file.
    let mut module_file = File::create(format!("src/{}.rs", folder))?;
    module_file.write_all(LIB_HEADER.as_bytes())?;
    for (module_name, node_name) in &module_names {
        let line = format!(
            r#"mod {module_name};
pub use {module_name}::{node_name};

"#
        );
        module_file.write_all(line.as_bytes())?;
    }
    drop(module_file);

    Ok(module_names)
}

fn rebuild_icons() -> Result<(), Box<dyn Error>> {
    let mut module_names = Vec::new();
    module_names.extend(build_icons("filled")?);
    module_names.extend(build_icons("outlined")?);
    module_names.extend(build_icons("twotone")?);
    module_names.sort();
    module_names.dedup();

    Ok(())
}

fn fetch_and_update_repo() {
    if fs::exists("icons").unwrap_or_default() {
        Command::new("git")
            .arg("pull")
            .current_dir("icons")
            .output()
            .expect("Failed to update ant-design-icons repo");
    } else {
        Command::new("git")
            .arg("clone")
            .arg("https://github.com/ant-design/ant-design-icons")
            .arg("icons")
            .output()
            .expect("Failed to clone ant-design-icons repo");
    }
}

fn main() {
    // Check ZU_ICON_UPDATE=ant environment.
    if need_update_with_name("ant") {
        // Fetch icons repo first: `git clone https://github.com/ant-design/ant-design-icons icons`
        fetch_and_update_repo();
        rebuild_icons().unwrap();
    }
}
