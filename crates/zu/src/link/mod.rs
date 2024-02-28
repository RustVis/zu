// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

mod underline;

use web_sys::FocusEvent;
use yew::{
    classes, function_component, html, use_state, AttrValue, Callback, Children, Classes, Html,
    Properties,
};

use crate::styles::color::Color;
use crate::styles::CssClass;
use crate::typography::{Typography, Variant};

// Re-export
pub use underline::Underline;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    /// The content of the component.
    #[prop_or_default]
    pub children: Children,

    /// Override or extend the styles applied to the component.
    #[prop_or_default]
    pub classes: Classes,

    /// The color of the link.
    #[prop_or(Color::Primary)]
    pub color: Color,

    /// The component used for the root node.
    /// Default value is `a`.
    #[prop_or(AttrValue::from("a"))]
    pub component: AttrValue,

    #[prop_or_default]
    pub on_blur: Option<Callback<FocusEvent>>,

    #[prop_or_default]
    pub on_focus: Option<Callback<FocusEvent>>,

    #[prop_or_default]
    pub href: AttrValue,

    #[prop_or_default]
    pub style: AttrValue,

    /// Classes prop applied to the `Typography` element.
    #[prop_or_default]
    pub typography_classes: AttrValue,

    /// Controls when the link should have an underline.
    #[prop_or(Underline::Always)]
    pub underline: Underline,

    /// Applies the theme typography styles.
    #[prop_or(Variant::Inherit)]
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
        },
        props.classes.clone(),
    );

    html! {
        <Typography
            classes={root_cls}
            color={props.color}
            component={&props.component}
            href={&props.href}
            style={&props.style}
            variant={props.variant}>
            {for props.children.iter()}
        </Typography>
    }
}
