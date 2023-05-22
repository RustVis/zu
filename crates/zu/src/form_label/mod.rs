// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};

use crate::styles::color::Color;
use crate::styles::CssValue;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    /// The content of the component.
    #[prop_or_default]
    pub children: Children,

    /// Override or extend the styles applied to the component.
    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub color: Color,

    /// The component used for the root node
    #[prop_or_default]
    pub component: AttrValue,

    /// If true, the label should be displayed in a disabled state.
    #[prop_or(false)]
    pub disabled: bool,

    /// If true, the label is displayed in an error state.
    #[prop_or(false)]
    pub error: bool,

    /// If true, the label should use filled classes key.
    #[prop_or(false)]
    pub filled: bool,

    /// If true, the input of this label is focused (used by FormGroup components).
    #[prop_or(false)]
    pub focused: bool,

    /// If true, the label will indicate that the input is required.
    #[prop_or(false)]
    pub required: bool,

    /// Custom inline style.
    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(FormLabel)]
pub fn form_label(props: &Props) -> Html {
    let component = if props.component.is_empty() {
        "label"
    } else {
        props.component.as_str()
    };

    let cls = classes!(
        "ZuFormLabel-root",
        if props.color == Color::Secondary {
            "ZuFormLabel-colorSecondary"
        } else {
            ""
        },
        if props.disabled {
            "ZuFormLabel-disabled"
        } else {
            ""
        },
        if props.filled {
            "ZuFormLabel-filled"
        } else {
            ""
        },
        if props.focused {
            "ZuFormLabel-focused"
        } else {
            ""
        },
        if props.required {
            "ZuFormLabel-required"
        } else {
            ""
        },
    );

    let style = [
        props.style.as_str(),
        &format!("color: {}", props.color.css_value()),
    ]
    .join(";");

    let asterisk_cls = if props.error {
        "ZuFormLabel-asterisk ZuFormLabel-error"
    } else {
        "ZuFormLabel-asterisk"
    };

    html! {
        <@{component.to_owned()} class={cls} style={style}>
            {for props.children.iter()}
            if props.required {
                <span class={asterisk_cls}>
                {"&thinsp;{'*'}"}
                </span>
            }
        </@>
    }
}
