// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

//! Download icons from gstatic.com and convert to SvgIcon components

#![allow(dead_code)]

use inflections::Inflect;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

use zuicon_util::{get_svg_inner, need_update, TEMPLATE_FILE};

const SVG_DIR: &str = "third_party/material-ui/packages/mui-icons-material/material-icons";
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
    let name: String = name.replace("_24px", "");
    let names = vec!["box", "option", "type", "try", "loop", "html"];
    if names.contains(&name.as_str()) || name.chars().next().unwrap().is_ascii_digit() {
        format!("icon-{name}")
    } else {
        name
    }
}

fn build_icons(module_names: &mut Vec<String>) -> Result<(), io::Error> {
    let mut dir = PathBuf::new();
    dir.push(SVG_DIR);

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

#[derive(Debug, Clone, Deserialize)]
struct IconsIndex {
    host: String,
    asset_url_pattern: String,
    families: Vec<String>,
    icons: Vec<Icon>,
}

#[derive(Debug, Clone, Deserialize)]
struct Icon {
    name: String,
    version: i32,
    popularity: i32,
    codepoint: i32,
    unsupported_families: Vec<String>,
    categories: Vec<String>,
    tags: Vec<String>,
    sizes_px: Vec<i32>,
}

fn download_index() -> Result<IconsIndex, Box<dyn Error>> {
    let url = "https://fonts.google.com/metadata/icons";
    let resp = reqwest::blocking::get(url)?.text()?;
    let resp = resp.replace(")]}'", "");
    let index: IconsIndex = serde_json::from_str(&resp)?;
    Ok(index)
}

const IGNORED_NAMES: &[&str] = &[
    "123",
    "6_ft_apart",
    "add_chart", // Leads to inconsistent casing with `Addchart`
    "area_chart",
    "compost",
    "cruelty_free",
    "data_exploration",
    "disabled_visible",
    "drive_file_move_rtl",
    "emergency",
    "exposure_neg_1",  // Google product
    "exposure_neg_2",  // Google product
    "exposure_plus_1", // Google product
    "exposure_plus_2", // Google product
    "exposure_zero",   // Google product
    "free_cancellation",
    "front_hand",
    "generating_tokens",
    "group_off",
    "horizontal_distribute", // Advanced text editor
    "hotel_class",
    "incomplete_circle",
    "motion_photos_on",     // Google product
    "motion_photos_pause",  // Google product
    "motion_photos_paused", // Google product
    "new_label",
    "personal_injury",
    "pin_end",
    "pin_invoke",
    "polymer", // Legacy brand
    "private_connectivity",
    "real_estate_agent",
    "vertical_distribute", // Advanced text editor
];

fn is_icon_ignored(icon: &Icon) -> bool {
    IGNORED_NAMES.contains(&icon.name.as_str())
}

fn download_icons(index: &IconsIndex) -> Result<i32, Box<dyn Error>> {
    let theme_map = &[
        ("baseline", ""), // filled
        ("outline", "_outlined"),
        ("round", "_round"),
        ("twotone", "_two_tone"),
        ("sharp", "_sharp"),
    ];
    let theme_file_map = [
        ("baseline", ""), // filled
        ("outline", "_outlined"),
        ("round", "_rounded"),
        ("twotone", "_two_tone"),
        ("sharp", "_sharp"),
    ];
    let theme_file_map = HashMap::from(theme_file_map);

    let _ret = fs::create_dir("icons");
    let mut count = 0;
    for (theme, value) in theme_map {
        let formatted_theme = value.split("_").collect::<Vec<_>>().join("");
        for icon in &index.icons {
            if is_icon_ignored(icon) {
                continue;
            }
            let name = &icon.name;
            let version = &icon.version;
            let url = format!("https://fonts.gstatic.com/s/i/materialicons{formatted_theme}/{name}/v{version}/24px.svg");
            println!("Downloading icon {url}");
            let resp = reqwest::blocking::get(url)?.text()?;
            let file_map = theme_file_map.get(theme).unwrap();
            let output_file = format!("icons/{name}{file_map}_24px.svg");
            fs::write(output_file, resp)?;
            count += 1;
        }
    }

    Ok(count)
}

fn run() -> Result<(), Box<dyn Error>> {
    // 1. download icon index
    let icons_index = download_index()?;

    // 2. download icons
    let count = download_icons(&icons_index)?;
    println!("downloaded icons: {count}");

    Ok(())
}

fn main() {
    // Check ZUICON_UPDATE=1 environment.
    if need_update() {
        run().unwrap();
    }
}
