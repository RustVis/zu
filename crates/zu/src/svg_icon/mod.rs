// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod color;
mod font_size;

use yew::{classes, function_component, html, Children, Html, Properties};

use crate::styles::CssClass;

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

    #[prop_or_default]
    pub html_color: String,

    #[prop_or_default]
    pub style: String,

    /// Add data-icon attribute if specified.
    #[prop_or_default]
    pub icon: String,

    /// Element title.
    #[prop_or_default]
    pub title_access: String,

    /// Default is "0 0 24 24".
    #[prop_or_default]
    pub view_box: String,
}

#[function_component(SvgIcon)]
pub fn svg_icon(props: &Props) -> Html {
    let _component = if props.component.is_empty() {
        "svg".to_owned()
    } else {
        props.component.clone()
    };

    let root_cls = classes!(
        "ZuSvgIcon-root",
        props.color.css_class(),
        props.font_size.css_class(),
    );

    let view_box = if props.view_box.is_empty() {
        DEFAULT_VIEW_BOX.to_owned()
    } else {
        props.view_box.clone()
    };
    let html_color = if props.html_color.is_empty() {
        None
    } else {
        Some(props.html_color.clone())
    };
    let aria_hidden = !props.title_access.is_empty();

    let style = if props.style.is_empty() {
        None
    } else {
        Some(props.style.clone())
    };

    let icon = if props.icon.is_empty() {
        None
    } else {
        Some(props.icon.clone())
    };

    html! {
        <svg class={root_cls}
            style={style}
            focusable={"false"}
            color={html_color}
            aria-hidden={aria_hidden.to_string()}
            data-icon={icon}
            viewBox={view_box}>
            {for props.children.iter()}
            if !props.title_access.is_empty() {
                <title>{&props.title_access}</title>
            }
        </svg>
    }
}
