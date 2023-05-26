// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod underline;

use yew::{classes, function_component, html, use_state, AttrValue, Children, Html, Properties};

use crate::styles::color::Color;
use crate::styles::CssClass;
use crate::typography::{Typography, Variant};

// Re-export
pub use underline::Underline;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub color: Color,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or_default]
    pub href: AttrValue,

    #[prop_or_default]
    pub style: AttrValue,

    /// Classes prop applied to the `Typography` element.
    #[prop_or_default]
    pub typography_classes: AttrValue,

    /// Controls when the link should have an underline.
    #[prop_or_default]
    pub underline: Underline,

    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(Link)]
pub fn link(props: &Props) -> Html {
    let focus_visible = use_state(|| false);

    let root_cls = classes!(
        "ZuLink-root",
        props.underline.css_class(),
        if "button" == props.component.as_str() {
            "ZuLink-button"
        } else {
            ""
        },
        if *focus_visible {
            "ZuLink-focusVisible"
        } else {
            ""
        }
    );

    let component = if props.component.is_empty() {
        "a"
    } else {
        props.component.as_str()
    };

    html! {
        <Typography
            classes={root_cls}
            style={&props.style}
            color={props.color.clone()}
            component={component.to_owned()}
            href={&props.href}
            variant={props.variant}>
            {for props.children.iter()}
        </Typography>
    }
}
