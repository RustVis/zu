// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::fs;
use std::io;
use std::io::Write;

fn merge_themes(palette_file: &str, output_file: &str) -> io::Result<()> {
    let dark_styles = [
        palette_file,
        "src/themes/animation.css",
        "src/themes/shape.css",
        "src/themes/z-index.css",
        // Components
        "src/skeleton/style.css",
    ];

    let mut dark_file = fs::OpenOptions::new()
        .truncate(true)
        .create(true)
        .write(true)
        .open(output_file)?;

    for file in dark_styles {
        let content = fs::read_to_string(file)?;
        dark_file.write_all(content.as_bytes())?;
        dark_file.write_all(b"\n\n")?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    merge_themes("src/themes/dark-palette.css", "themes/dark-theme.css")?;
    merge_themes("src/themes/light-palette.css", "themes/light-theme.css")
}
