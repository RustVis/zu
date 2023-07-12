// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod color;
mod font_size;

use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};
use zu_util::prop::ToAttr;

use crate::styles::color::DisabledColor;
use crate::styles::CssClass;
pub use font_size::FontSize;

pub const DEFAULT_VIEW_BOX: &str = "0 0 24 24";

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub color: DisabledColor,

    #[prop_or_default]
    pub component: AttrValue,

    /// The fontSize applied to the icon.
    ///
    /// Default is 24px.
    #[prop_or_default]
    pub font_size: FontSize,

    #[prop_or_default]
    pub html_color: AttrValue,

    #[prop_or_default]
    pub style: AttrValue,

    /// Add data-icon attribute if specified.
    #[prop_or_default]
    pub icon: AttrValue,

    /// Element title.
    #[prop_or_default]
    pub title_access: AttrValue,

    /// Default is "0 0 24 24".
    #[prop_or_default]
    pub view_box: AttrValue,
}

#[function_component(SvgIcon)]
pub fn svg_icon(props: &Props) -> Html {
    let _component = if props.component.is_empty() {
        "svg"
    } else {
        props.component.as_str()
    };

    let root_cls = classes!(
        "ZuSvgIcon-root",
        color::css_class(props.color),
        props.font_size.css_class(),
    );

    let view_box = if props.view_box.is_empty() {
        DEFAULT_VIEW_BOX
    } else {
        props.view_box.as_str()
    };

    let aria_hidden = !props.title_access.is_empty();

    html! {
        <svg class={root_cls}
            style={props.style.to_attr()}
            focusable={"false"}
            color={props.html_color.to_attr()}
            aria-hidden={aria_hidden.to_attr()}
            data-icon={props.icon.to_attr()}
            viewBox={view_box.to_owned()}>
            {for props.children.iter()}
            if !props.title_access.is_empty() {
                <title>{&props.title_access}</title>
            }
        </svg>
    }
}
