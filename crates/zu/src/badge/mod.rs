// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod anchor_origin;
mod color;
mod content;
mod overlap;
mod variant;

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};
use zu_util::prop::ToAttr;

use crate::styles::anchor_origin::AnchorOrigin;
use crate::styles::{color::Color, CssClass};
pub use content::Content;
pub use overlap::Overlap;
pub use variant::Variant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(AnchorOrigin::top_right())]
    pub anchor_origin: AnchorOrigin,

    /// The content rendered within the badge.
    #[prop_or_default]
    pub content: Option<Content>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or(Color::Default)]
    pub color: Color,

    /// Default value is `span`.
    #[prop_or(AttrValue::from("span"))]
    pub component: AttrValue,

    /// If true, the badge is invisible.
    #[prop_or(false)]
    pub invisible: bool,

    /// Max count to show.
    #[prop_or(99)]
    pub max: i32,

    /// Wrapped shape the badge should overlap.
    #[prop_or_default]
    pub overlap: Overlap,

    /// Controls whether the badge is hidden when badgeContent is zero.
    #[prop_or(false)]
    pub show_zero: bool,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub badge_classes: Classes,

    #[prop_or_default]
    pub badge_style: AttrValue,

    /// The variant to use.
    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(Badge)]
pub fn badge(props: &Props) -> Html {
    let root_cls = classes!("ZuBadge-root", props.classes.clone());

    let badge_cls = classes!(
        "ZuBadge-badge",
        props.variant.css_class(),
        if props.invisible {
            "ZuBadge-invisible"
        } else {
            ""
        },
        anchor_origin::css_class(props.anchor_origin),
        props.overlap.css_class(),
        color::color_class(props.color),
        props.badge_classes.clone(),
    );

    // TODO(Shaohua): Use invisible property.
    let _invisible = props.invisible || props.content.is_none() || props.variant != Variant::Dot;

    let display_value = if props.variant == Variant::Standard {
        match &props.content {
            None => None,
            Some(Content::Node(node)) => Some(html! {node.clone()}),
            Some(Content::Str(s)) => Some(html! {s}),
            Some(Content::String(s)) => Some(html! {s}),
            Some(Content::Num(num)) => {
                let s = if *num > props.max {
                    format!("{}+", props.max)
                } else {
                    num.to_string()
                };
                Some(html! {s})
            }
        }
    } else {
        None
    };

    html! {
        <@{props.component.to_string()} class={root_cls} style={props.style.to_attr()}>
            {for props.children.iter()}
            <span class={badge_cls} style={props.badge_style.to_attr()}>
                {display_value}
            </span>
        </@>
    }
}
