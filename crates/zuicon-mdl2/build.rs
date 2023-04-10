// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use inflections::Inflect;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

use zuicon_util::{get_svg_inner, need_update};

const SVG_DIR: &str = "third_party/fluentui-mdl2/src/components";
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

fn build_icons(module_names: &mut Vec<String>) -> Result<(), io::Error> {
    let mut dir = PathBuf::new();
    dir.push(SVG_DIR);

    let svg_extension = OsStr::new("tsx");
    let template = include_str!("template.rs");

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
        let rs_content = template
            .replace("NODE_NAME", &node_name)
            .replace("DATA_NAME", data_name)
            .replace("MARKUP", markup);

        fs::write(rs_filepath, rs_content).unwrap();
        module_names.push(module_name);
    }

    Ok(())
}

fn rebuild_icons() {
    let mut module_names = Vec::new();
    build_icons(&mut module_names).unwrap();

    let fd = File::create("src/lib.rs").unwrap();
    let mut buf_fd = BufWriter::new(fd);
    buf_fd.write_all(LIB_HEADER.as_bytes()).unwrap();
    module_names.sort();
    for module_name in &module_names {
        buf_fd
            .write_all(format!("pub mod {module_name};\n").as_bytes())
            .unwrap();
    }
}

fn main() {
    // Check ZUICON_UPDATE=1 environment.
    if need_update() {
        rebuild_icons()
    }
}
