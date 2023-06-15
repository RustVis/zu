// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

#[path = "consts/theme.rs"]
mod theme;

#[path = "consts/svg_icons.rs"]
mod svg_icons;

use inflections::Inflect;
use rsass::{compile_scss_path, output};
use std::env;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use zu_util::icon::need_update;

fn merge_themes(style_files: &[&str], output_name: &str) -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let output_path = Path::new(&out_dir).join(output_name);

    let mut output_file = fs::OpenOptions::new()
        .truncate(true)
        .create(true)
        .write(true)
        .open(output_path)?;

    for file in style_files {
        let content = fs::read_to_string(file)?;
        output_file.write_all(content.as_bytes())?;
        output_file.write_all(b"\n\n")?;
    }

    Ok(())
}

fn compile_scss(input_name: &str, output_name: &str) -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let input_path = Path::new(&out_dir).join(input_name);
    let output_path = Path::new(&out_dir).join(output_name);

    let format = output::Format {
        style: output::Style::Expanded,
        ..Default::default()
    };
    let css = compile_scss_path(&input_path, format)?;

    let mut output_file = fs::OpenOptions::new()
        .truncate(true)
        .create(true)
        .write(true)
        .open(output_path)?;
    let css: String = String::from_utf8(css).unwrap();
    // NOTE(Shaohua): Remove @charset, as it is not supported by rsass yet.
    let css = css.replace("@charset \"UTF-8\";", "");
    output_file.write_all(css.as_bytes())?;

    Ok(())
}

fn generate_style_files() -> Result<(), Box<dyn Error>> {
    let mut dark_files = vec![
        "src/themes/dark-palette.scss",
        "src/themes/export-palette.scss",
        "src/themes/dark-components.scss",
    ];
    dark_files.extend_from_slice(theme::COLORS);
    dark_files.extend_from_slice(theme::COMMON_STYLES);
    merge_themes(&dark_files, "dark-theme.scss")?;
    compile_scss("dark-theme.scss", "dark-theme.css")?;

    let mut light_files = vec![
        "src/themes/light-palette.scss",
        "src/themes/export-palette.scss",
        "src/themes/light-components.scss",
    ];
    light_files.extend_from_slice(theme::COLORS);
    light_files.extend_from_slice(theme::COMMON_STYLES);
    merge_themes(&light_files, "light-theme.scss")?;
    compile_scss("light-theme.scss", "light-theme.css")?;

    //merge_themes(COLORS, "color-schemes.css")?;

    Ok(())
}

fn generate_svg_icons() -> Result<(), Box<dyn Error>> {
    const TEMPLATE_FILE: &str = include_str!("src/internal/svg_icons/template.rs");

    let mut mod_file = OpenOptions::new()
        .append(true)
        .open("src/internal/svg_icons.rs")?;
    for (node_name, path_data) in svg_icons::SVG_ICONS {
        let module_name = node_name.to_snake_case();
        println!("module name: {module_name}");
        let mut rs_filepath = PathBuf::new();
        rs_filepath.push("src/internal/svg_icons");
        rs_filepath.push(&module_name);
        rs_filepath.set_extension("rs");

        let rs_content = TEMPLATE_FILE
            .replace("{MODULE_NAME}", &module_name)
            .replace("{NODE_NAME}", node_name)
            .replace("{ICON_NAME}", node_name)
            .replace("{PATH_DATA}", path_data);

        fs::write(rs_filepath, rs_content).unwrap();

        let line = format!(
            r#"mod {module_name};
pub use {module_name}::{node_name};

"#
        );
        mod_file.write_all(line.as_bytes())?;
    }

    drop(mod_file);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    generate_style_files()?;

    // Check ZU_ICON_UPDATE=1 environment.
    if need_update() {
        generate_svg_icons()?;
    }

    Ok(())
}
