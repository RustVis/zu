// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod color;
mod font_size;

use yew::{function_component, html, Children, Html, Properties};

// Re-export
pub use color::Color;
pub use font_size::FontSize;

pub const DEFAULT_VIEW_BOX: &str = "0 0 24 24";

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: String,

    #[prop_or_default]
    pub color: Color,

    #[prop_or_default]
    pub component: String,

    /// The fontSize applied to the icon.
    ///
    /// Default is 24px.
    #[prop_or_default]
    pub font_size: FontSize,

    /// Element title.
    #[prop_or_default]
    pub title: String,

    /// Default is "0 0 24 24".
    #[prop_or_default]
    pub view_box: String,
}

#[function_component(SvgIcon)]
pub fn svg_icon(props: &Props) -> Html {
    html! {
        <svg>
        {props.children.clone()}
        </svg>
    }
}
