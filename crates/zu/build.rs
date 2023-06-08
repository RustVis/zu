// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use inflections::Inflect;
use std::env;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use zu_util::icon::need_update;

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
    "src/themes/_functions.scss",
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
    "src/accordion_details/style.scss",
    "src/accordion_summary/style.scss",
    "src/alert/style.scss",
    "src/alert_title/style.scss",
    "src/app_bar/style.scss",
    "src/autocomplete/style.scss",
    "src/avatar/style.scss",
    "src/avatar_group/style.scss",
    "src/backdrop/style.scss",
    "src/badge/style.scss",
    "src/bottom_navigation/style.scss",
    "src/bottom_navigation_action/style.scss",
    "src/box/style.scss",
    "src/breadcrumbs/style.scss",
    "src/button/style.scss",
    "src/button_base/style.scss",
    "src/button_group/style.scss",
    "src/card/style.scss",
    "src/card_action_area/style.scss",
    "src/card_actions/style.scss",
    "src/card_content/style.scss",
    "src/card_header/style.scss",
    "src/card_media/style.scss",
    "src/checkbox/style.scss",
    "src/chip/style.scss",
    "src/code/style.scss",
    "src/collapse/style.scss",
    "src/container/style.scss",
    "src/circular_progress/style.scss",
    "src/dialog/style.scss",
    "src/dialog_actions/style.scss",
    "src/dialog_content/style.scss",
    "src/dialog_content_text/style.scss",
    "src/dialog_title/style.scss",
    "src/divider/style.scss",
    "src/drawer/style.scss",
    "src/fab/style.scss",
    "src/fade/style.scss",
    "src/filled_input/style.scss",
    "src/form_control/style.scss",
    "src/form_control_label/style.scss",
    "src/form_group/style.scss",
    "src/form_helper_text/style.scss",
    "src/form_label/style.scss",
    "src/grow/style.scss",
    "src/icon_button/style.scss",
    "src/input_label/style.scss",
    "src/linear_progress/style.scss",
    "src/link/style.scss",
    "src/list/style.scss",
    "src/list_item_avatar/style.scss",
    "src/list_item_icon/style.scss",
    "src/paper/style.scss",
    "src/scoped_css_baseline/style.scss",
    "src/skeleton/style.scss",
    "src/stack/style.scss",
    "src/svg_icon/style.scss",
    "src/switch/style.scss",
    "src/switch_base/style.scss",
    "src/tab/style.scss",
    "src/table/style.scss",
    "src/table_body/style.scss",
    "src/table_cell/style.scss",
    "src/table_container/style.scss",
    "src/table_footer/style.scss",
    "src/table_head/style.scss",
    "src/table_pagination/style.scss",
    "src/table_row/style.scss",
    "src/table_sort_label/style.scss",
    "src/timeline/style.scss",
    "src/timeline_connector/style.scss",
    "src/timeline_content/style.scss",
    "src/timeline_dot/style.scss",
    "src/timeline_item/style.scss",
    "src/timeline_opposite_content/style.scss",
    "src/timeline_separator/style.scss",
    "src/toggle_button/style.scss",
    "src/toggle_button_group/style.scss",
    "src/toolbar/style.scss",
    "src/tree_item/style.scss",
    "src/tree_view/style.scss",
    "src/typography/style.scss",
    "src/zoom/style.scss",
];

const COLORS: &[&str] = &[
    "src/colors/amber.scss",
    "src/colors/blue.scss",
    "src/colors/blueGrey.scss",
    "src/colors/brown.scss",
    "src/colors/cyan.scss",
    "src/colors/deepOrange.scss",
    "src/colors/deepPurple.scss",
    "src/colors/green.scss",
    "src/colors/grey.scss",
    "src/colors/indigo.scss",
    "src/colors/lightBlue.scss",
    "src/colors/lightGreen.scss",
    "src/colors/lime.scss",
    "src/colors/orange.scss",
    "src/colors/pink.scss",
    "src/colors/purple.scss",
    "src/colors/red.scss",
    "src/colors/teal.scss",
    "src/colors/yellow.scss",
];

fn generate_style_files() -> Result<(), Box<dyn Error>> {
    let mut dark_files = vec![
        "src/themes/dark-palette.scss",
        "src/themes/export-palette.scss",
        "src/themes/dark-components.scss",
    ];
    dark_files.extend_from_slice(COLORS);
    dark_files.extend_from_slice(COMMON_STYLES);
    merge_themes(&dark_files, "dark-theme.scss")?;
    compile_scss("dark-theme.scss", "dark-theme.css")?;

    let mut light_files = vec![
        "src/themes/light-palette.scss",
        "src/themes/export-palette.scss",
        "src/themes/light-components.scss",
    ];
    light_files.extend_from_slice(COLORS);
    light_files.extend_from_slice(COMMON_STYLES);
    merge_themes(&light_files, "light-theme.scss")?;
    compile_scss("light-theme.scss", "light-theme.css")?;

    //merge_themes(COLORS, "color-schemes.css")?;

    Ok(())
}

pub const SVG_ICONS: &[(&str, &str)] = &[
    ("Add", r#"<path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z" />"#),
    (
        "ArrowDownward",
        r#"<path d="M20 12l-1.41-1.41L13 16.17V4h-2v12.17l-5.58-5.59L4 12l8 8 8-8z" />"#,
    ),
    ("ArrowDropDown", r#"<path d="M7 10l5 5 5-5z" />"#),
    (
        "Cancel",
        r#"<path d="M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm5 13.59L15.59 17 12 13.41 8.41 17 7 15.59 10.59 12 7 8.41 8.41 7 12 10.59 15.59 7 17 8.41 13.41 12 17 15.59z" />"#,
    ),
    (
        "CheckBox",
        r#"<path d="M19 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2V5c0-1.1-.89-2-2-2zm-9 14l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z" />"#,
    ),
    (
        "CheckBoxOutlineBlank",
        r#"<path d="M19 5v14H5V5h14m0-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z" />"#,
    ),
    (
        "CheckCircle",
        r#"<path d="M12 0a12 12 0 1 0 0 24 12 12 0 0 0 0-24zm-2 17l-5-5 1.4-1.4 3.6 3.6 7.6-7.6L19 8l-9 9z" />"#,
    ),
    (
        "Close",
        r#"<path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z" />"#,
    ),
    (
        "ErrorOutline",
        r#"<path d="M11 15h2v2h-2zm0-8h2v6h-2zm.99-5C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z" />"#,
    ),
    (
        "FirstPage",
        r#"<path d="M18.41 16.59L13.82 12l4.59-4.59L17 6l-6 6 6 6zM6 6h2v12H6z" />"#,
    ),
    (
        "IntermediateCheckBox",
        r#"<path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-2 10H7v-2h10v2z" />"#,
    ),
    (
        "InfoOutlined",
        r#"<path d="M11,9H13V7H11M12,20C7.59,20 4,16.41 4,12C4,7.59 7.59,4 12,4C16.41,4 20,7.59 20, 12C20,16.41 16.41,20 12,20M12,2A10,10 0 0,0 2,12A10,10 0 0,0 12,22A10,10 0 0,0 22,12A10, 10 0 0,0 12,2M11,17H13V11H11V17Z" />"#,
    ),
    (
        "KeyboardArrowLeft",
        r#"<path d="M15.41 16.09l-4.58-4.59 4.58-4.59L14 5.5l-6 6 6 6z" />"#,
    ),
    (
        "KeyboardArrowRight",
        r#"<path d="M8.59 16.34l4.58-4.59-4.58-4.59L10 5.75l6 6-6 6z" />"#,
    ),
    (
        "LastPage",
        r#"<path d="M5.59 7.41L10.18 12l-4.59 4.59L7 18l6-6-6-6zM16 6h2v12h-2z" />"#,
    ),
    (
        "MoreHorizontal",
        r#"<path d="M6 10c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm12 0c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm-6 0c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z" />"#,
    ),
    (
        "NavigateBefore",
        r#"<path d="M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z" />"#,
    ),
    (
        "NavigateNext",
        r#"<path d="M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z" />"#,
    ),
    (
        "Person",
        r#"<path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z" />"#,
    ),
    (
        "RadioButtonChecked",
        r#"<path d="M8.465 8.465C9.37 7.56 10.62 7 12 7C14.76 7 17 9.24 17 12C17 13.38 16.44 14.63 15.535 15.535C14.63 16.44 13.38 17 12 17C9.24 17 7 14.76 7 12C7 10.62 7.56 9.37 8.465 8.465Z" />"#,
    ),
    (
        "RadioButtonUnchecked",
        r#"<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z" />"#,
    ),
    (
        "ReportProblemOutlined",
        r#"<path d="M12 5.99L19.53 19H4.47L12 5.99M12 2L1 21h22L12 2zm1 14h-2v2h2v-2zm0-6h-2v4h2v-4z" />"#,
    ),
    (
        "Star",
        r#"<path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z" />"#,
    ),
    (
        "StarBorder",
        r#"<path d="M22 9.24l-7.19-.62L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21 12 17.27 18.18 21l-1.63-7.03L22 9.24zM12 15.4l-3.76 2.27 1-4.28-3.32-2.88 4.38-.38L12 6.1l1.71 4.04 4.38.38-3.32 2.88 1 4.28L12 15.4z" />"#,
    ),
    (
        "SuccessOutlined",
        r#"<path d="M20,12A8,8 0 0,1 12,20A8,8 0 0,1 4,12A8,8 0 0,1 12,4C12.76,4 13.5,4.11 14.2, 4.31L15.77,2.74C14.61,2.26 13.34,2 12,2A10,10 0 0,0 2,12A10,10 0 0,0 12,22A10,10 0 0, 0 22,12M7.91,10.08L6.5,11.5L11,16L21,6L19.59,4.58L11,13.17L7.91,10.08Z" />"#,
    ),
    (
        "Warning",
        r#"<path d="M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z" />"#,
    ),
];

fn generate_svg_icons() -> Result<(), Box<dyn Error>> {
    const TEMPLATE_FILE: &str = include_str!("src/internal/svg_icons/template.rs");

    let mut mod_file = OpenOptions::new()
        .append(true)
        .open("src/internal/svg_icons.rs")?;
    for (node_name, path_data) in SVG_ICONS {
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
