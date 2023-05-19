// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

use rsass::{compile_scss_path, output};

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

const COMMON_STYLES: &[&str] = &[
    "src/themes/border-radius.scss",
    "src/themes/breakpoints.scss",
    "src/themes/components-base.scss",
    "src/themes/direction.scss",
    "src/themes/fonts.scss",
    "src/themes/shadows.scss",
    "src/themes/shape.scss",
    "src/themes/spacings.scss",
    "src/themes/stroke-widths.scss",
    "src/themes/transitions.scss",
    "src/themes/typography.scss",
    "src/themes/z-index.scss",
    // Components
    "src/accordion/style.scss",
    "src/accordion_summary/style.scss",
    "src/avatar/style.scss",
    "src/avatar_group/style.scss",
    "src/badge/style.scss",
    "src/box/style.scss",
    "src/container/style.scss",
    "src/circular_progress/style.scss",
    "src/divider/style.scss",
    "src/form_group/style.scss",
    "src/form_label/style.scss",
    "src/linear_progress/style.scss",
    "src/paper/style.scss",
    "src/skeleton/style.scss",
    "src/stack/style.scss",
    "src/svg_icon/style.scss",
    "src/switch/style.scss",
    "src/typography/style.scss",
];

const COLORS: &[&str] = &[
    "src/colors/amber.css",
    "src/colors/blue.css",
    "src/colors/blueGrey.css",
    "src/colors/brown.css",
    "src/colors/cyan.css",
    "src/colors/deepOrange.css",
    "src/colors/deepPurple.css",
    "src/colors/green.css",
    "src/colors/grey.css",
    "src/colors/indigo.css",
    "src/colors/lightBlue.css",
    "src/colors/lightGreen.css",
    "src/colors/lime.css",
    "src/colors/orange.css",
    "src/colors/pink.css",
    "src/colors/purple.css",
    "src/colors/red.css",
    "src/colors/teal.css",
    "src/colors/yellow.css",
];

fn main() -> Result<(), Box<dyn Error>> {
    let mut dark_files = vec![
        "src/themes/dark-palette.scss",
        "src/themes/export-palette.scss",
        "src/themes/dark-components.scss",
    ];
    dark_files.extend_from_slice(COMMON_STYLES);
    merge_themes(&dark_files, "dark-theme.scss")?;
    compile_scss("dark-theme.scss", "dark-theme.css")?;

    let mut light_files = vec![
        "src/themes/light-palette.scss",
        "src/themes/export-palette.scss",
        "src/themes/light-components.scss",
    ];
    light_files.extend_from_slice(COMMON_STYLES);
    merge_themes(&light_files, "light-theme.scss")?;
    compile_scss("light-theme.scss", "light-theme.css")?;

    merge_themes(COLORS, "color-schemes.css")?;

    Ok(())
}
