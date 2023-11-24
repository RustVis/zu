// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

mod color;

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::styles::color::Color;
use crate::styles::CssValue;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    /// The content of the component.
    #[prop_or_default]
    pub children: Children,

    /// Override or extend the styles applied to the component.
    #[prop_or_default]
    pub classes: Classes,

    /// The color of the component.
    #[prop_or_default]
    pub color: Color,

    /// The component used for the root node
    ///
    /// Default is `label`.
    #[prop_or(AttrValue::from("label"))]
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
    let root_cls = classes!(
        "ZuFormLabel-root",
        color::color_class(props.color),
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
        props.classes.clone(),
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

    // TODO(Shaohua): Pass disabled to children.

    html! {
        <@{props.component.to_string()}
            class={root_cls}
            disabled={props.disabled}
            style={style}>

            {for props.children.iter()}

            if props.required {
                <span
                    aria_hidden={"true"}
                    class={asterisk_cls}>
                {"&thinsp;*"}
                </span>
            }
        </@>
    }
}
