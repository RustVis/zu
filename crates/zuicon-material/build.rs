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
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use zuicon_util::{get_svg_path_data, need_update};

const SVG_DIR: &str = "icons";
const CUSTOM_DIR: &str = "custom";
const TEMPLATE_FILE: &str = include_str!("src/template.rs");

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

    let _ret = fs::create_dir(SVG_DIR);
    let mut count = 0;
    for (theme, value) in theme_map {
        let formatted_theme = value.split('_').collect::<Vec<_>>().join("");
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
            let output_file = format!("{SVG_DIR}/{name}{file_map}_24px.svg");
            fs::write(output_file, resp)?;
            count += 1;
        }
    }

    Ok(count)
}

fn map_filename(name: &str) -> String {
    let name: String = name.replace("_24px", "");
    let names = vec!["box", "option", "type", "try", "loop", "html"];
    if names.contains(&name.as_str()) || name.chars().next().unwrap().is_ascii_digit() {
        format!("icon-{name}")
    } else {
        name
    }
}

fn build_icons(
    icons_dir: &str,
    module_names: &mut Vec<(String, String)>,
) -> Result<(), Box<dyn Error>> {
    let mut dir = PathBuf::new();
    dir.push(icons_dir);

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
        //let data_name = &stem_str;
        let node_name = stem_str.to_pascal_case();
        let module_name = stem_str.to_snake_case();
        let mut rs_filepath = PathBuf::new();
        rs_filepath.push("src");
        rs_filepath.push(icons_dir);
        rs_filepath.push(&module_name);
        rs_filepath.set_extension("rs");

        let svg_content = fs::read_to_string(&path)?;
        let path_data = get_svg_path_data(&svg_content)?;
        let rs_content = TEMPLATE_FILE
            .replace("{NODE_NAME}", &node_name)
            .replace("{ICON}", &node_name)
            .replace("{PATH_DATA}", &path_data);

        fs::write(rs_filepath, rs_content).unwrap();
        module_names.push((module_name, node_name));
    }

    Ok(())
}

fn generate_components(icons_dir: &str) -> Result<(), Box<dyn Error>> {
    let mut module_names = vec![];
    build_icons(icons_dir, &mut module_names)?;
    module_names.sort();

    let mut icons_file = OpenOptions::new()
        .append(true)
        .open(format!("src/{icons_dir}.rs"))?;
    for (module_name, node_name) in module_names.iter() {
        let line = format!(
            r#"#[cfg(feature = "{node_name}")]
mod {module_name};
#[cfg(feature = "{node_name}")]
pub use {module_name}::{node_name};

"#
        );
        icons_file.write_all(line.as_bytes())?;
    }
    drop(icons_file);

    let mut cargo_file = OpenOptions::new().append(true).open("Cargo.toml")?;
    for (_module_name, node_name) in module_names.iter() {
        let line = format!("{node_name} = []\n");
        cargo_file.write_all(line.as_bytes())?;
    }
    drop(cargo_file);

    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    // 1. Download icon index
    let icons_index = download_index()?;

    // 2. Download icons
    let _count = download_icons(&icons_index)?;

    // 3. Convert to SvgIcon components.
    generate_components(SVG_DIR)?;

    generate_components(CUSTOM_DIR)?;

    Ok(())
}

fn main() {
    // Check ZUICON_UPDATE=1 environment.
    if need_update() {
        run().unwrap();
    }
}
