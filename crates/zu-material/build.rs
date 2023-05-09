// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::fs;
use std::io;
use std::io::Write;

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

fn main() -> io::Result<()> {
    merge_themes(&["src/themes/dark-palette.scss"], "themes/dark-theme.css")?;
    merge_themes(&["src/themes/light-palette.scss"], "themes/light-theme.css")?;
    let common_styles = [
        "src/themes/animation.scss",
        "src/themes/shape.scss",
        "src/themes/z-index.scss",
        // Components
        "src/skeleton/style.scss",
    ];
    merge_themes(&common_styles, "themes/common-theme.css")
}
