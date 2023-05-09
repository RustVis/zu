// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

use rsass::{compile_scss_path, output};

fn merge_themes(style_files: &[&str], output_path: &str) -> io::Result<()> {
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

fn compile_scss<P: AsRef<Path>>(input_path: P, output_path: &str) -> Result<(), Box<dyn Error>> {
    let format = output::Format {
        style: output::Style::Compressed,
        ..Default::default()
    };
    let css = compile_scss_path(input_path.as_ref(), format)?;
    let mut output_file = fs::OpenOptions::new()
        .truncate(true)
        .create(true)
        .write(true)
        .open(output_path)?;
    output_file.write_all(&css)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    merge_themes(&["src/themes/dark-palette.scss"], "themes/dark-theme.scss")?;
    compile_scss("themes/dark-theme.scss", "themes/dark-theme.css")?;

    merge_themes(
        &["src/themes/light-palette.scss"],
        "themes/light-theme.scss",
    )?;
    compile_scss("themes/light-theme.scss", "themes/light-theme.css")?;

    let common_styles = [
        "src/themes/animation.scss",
        "src/themes/shape.scss",
        "src/themes/z-index.scss",
        // Components
        "src/skeleton/style.scss",
    ];
    merge_themes(&common_styles, "themes/common-theme.scss")?;
    compile_scss("themes/common-theme.scss", "themes/common-theme.css")
}
