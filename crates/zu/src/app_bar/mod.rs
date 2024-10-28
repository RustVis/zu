// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

mod color;
mod position;

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::paper::Paper;
use crate::styles::CssClass;

// Re-export
pub use color::Color;
pub use position::Position;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub color: Color,

    /// If true, the color prop is applied in dark mode.
    #[prop_or(false)]
    pub enable_color_on_dark: bool,

    #[prop_or_default]
    pub position: Position,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(AppBar)]
pub fn app_bar(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuAppBar-root",
        props.color.css_class(),
        props.position.css_class(),
        props.classes.clone(),
    );

    html! {
        <Paper
            classes={root_cls}
            component="header"
            style={&props.style}
            elevation={4}>
        </Paper>
    }
}
