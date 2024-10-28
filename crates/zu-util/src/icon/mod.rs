// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use roxmltree::{Document, Error, Node, NodeType};

pub const UPDATE_KEY: &str = "ZU_ICON_UPDATE";
pub const TEMPLATE_FILE: &str = include_str!("template.rs");

/// Check whether icon modules shall be refreshed.
///
/// Current `ZU_ICON_UPDATE=1` environment is used.
#[must_use]
pub fn need_update() -> bool {
    std::env::var_os(UPDATE_KEY).map_or(false, |val| !val.is_empty())
}

/// Check whether icon modules shall be refreshed.
///
/// Current `ZU_ICON_UPDATE="name"` environment is used.
#[must_use]
pub fn need_update_with_name(name: &str) -> bool {
    std::env::var_os(UPDATE_KEY).map_or(false, |val| val == name)
}

/// Get inner html of an svg file, without `<svg>` root tag.
#[must_use]
pub fn get_svg_inner(s: &str) -> Option<&str> {
    let start_index= s.find("<svg")?;
    let end_index = s.find("</svg>")?;

    let mut start_index_end = start_index;
    for (index, c) in s[start_index..].chars().enumerate() {
        if c == '>' {
            start_index_end = start_index + index + 1;
            break;
        }
    }
    if start_index == start_index_end || start_index_end >= end_index {
        return None;
    }
    Some(s[start_index_end..end_index].trim())
}

/// Get all `<path />` nodes in an svg file.
///
/// # Errors
/// Returns error if s is not a valid svg file.
pub fn get_svg_path_data(s: &str) -> Result<String, Error> {
    let doc = Document::parse(s)?;
    let nodes: Vec<Node> = doc
        .descendants()
        .filter(|n| n.node_type() == NodeType::Element && n.has_tag_name("path"))
        .collect();
    let mut s = Vec::new();
    for node in nodes {
        let mut parts = vec!["<path".to_owned()];
        for attr in node.attributes() {
            parts.push(format!(" {}=\"{}\"", attr.name(), attr.value()));
        }
        parts.push("/>".to_owned());
        s.push(parts.join(""));
    }
    Ok(s.join(""))
}

#[cfg(test)]
mod tests {
    use super::get_svg_path_data;

    #[test]
    fn test_path() {
        let svg_file = include_str!("../../tests/2k_two_tone_24px.svg");
        let ret = get_svg_path_data(svg_file);
        assert!(ret.is_ok());
        const REAL_PATH: &str = r#"<path d="M5,19h14V5H5V19z M13,9h1.5v2.25L16.25,9H18l-2.25,3L18,15h-1.75l-1.75-2.25V15H13V9z M6.5,12.5 c0-0.55,0.45-1,1-1h2v-1h-3V9H10c0.55,0,1,0.45,1,1v1.5c0,0.55-0.45,1-1,1H8v1h3V15H6.5V12.5z" opacity=".3"/><path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M19,19H5V5h14V19z"/><path d="M11,13.5H8v-1h2c0.55,0,1-0.45,1-1V10c0-0.55-0.45-1-1-1H6.5v1.5h3v1h-2c-0.55,0-1,0.45-1,1V15H11V13.5z"/>"#;
        assert_eq!(ret.unwrap().as_str(), REAL_PATH);
    }
}
