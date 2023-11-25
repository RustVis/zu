// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

mod position;

use yew::{classes, html, AttrValue, Children, Html, Properties};
use yew::{function_component, Classes};
use zu_util::prop::ToAttr;

use crate::stack::Stack;
use crate::styles::flex_direction::FlexDirection;
use crate::styles::position::Position;
use crate::typography::Typography;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    /// If true, the component appears selected.
    #[prop_or(false)]
    pub checked: bool,

    /// A control element.
    #[prop_or_default]
    pub control: Html,

    /// If true, the control is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// If true, the label is rendered as it is passed without an additional typography node.
    #[prop_or(false)]
    pub disable_typography: bool,

    // TODO(Shaohua): Add error state.
    #[prop_or(false)]
    pub error: bool,

    /// A text or an element to be used in an enclosing label element.
    #[prop_or_default]
    pub label: Html,

    #[prop_or(Position::End)]
    pub label_position: Position,

    /// If true, the label will indicate that the input is required.
    #[prop_or(false)]
    pub required: bool,

    #[prop_or_default]
    pub style: AttrValue,

    /// The value of the component.
    #[prop_or_default]
    pub value: AttrValue,
}

#[function_component(FormControlLabel)]
pub fn form_control_label(props: &Props) -> Html {
    let root_cls = classes!(
        "ZuFormControlLabel-root",
        if props.disabled {
            "ZuFormControlLabel-disabled"
        } else {
            ""
        },
        position::css_class(props.label_position),
        if props.error {
            "ZuFormControlLabel-error"
        } else {
            ""
        },
        if props.required {
            "ZuFormControlLabel-required"
        } else {
            ""
        },
        props.classes.clone(),
    );

    let label_cls = classes! {
        "ZuFormControlLabel-label",
        if props.disabled {
            "ZuFormControlLabel-disabled"
        } else {
            ""
        }
    };
    let asterisk_cls = classes! {
        "ZuFormControlLabel-asterisk",
        if props.error {
            "ZuFormControlLabel-error"
        } else {
            ""
        }
    };

    // TODO(Shaohua): Check label is null.
    // TODO(Shaohua): Pass disabled to props.control
    // TODO(Shaohua): Pass required to props.control
    let required = props.required;

    let label_elem = html! {
        <Typography classes={label_cls}>
            {props.label.clone()}
        </Typography>
    };

    html! {
        <label class={root_cls} style={props.style.to_attr()}>
            {props.control.clone()}

            if required {
                <Stack direction={FlexDirection::Row}>
                {label_elem.clone()}
                <span
                    aria-hidden="true"
                    class={asterisk_cls}>
                    {"\u{2009}*"}
                </span>
                </Stack>
            } else {
                {label_elem}
            }
        </label>
    }
}
