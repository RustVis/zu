// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod position;

use yew::function_component;
use yew::{classes, html, virtual_dom::VNode, AttrValue, Children, Html, Properties};

use crate::styles::side::Position;
use crate::typography::Typography;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    /// If true, the component appears selected.
    #[prop_or(false)]
    pub checked: bool,

    /// A control element.
    #[prop_or_default]
    pub control: VNode,

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
    pub label: VNode,

    #[prop_or(Position::End)]
    pub label_placement: Position,

    /// If true, the label will indicate that the input is required.
    #[prop_or(false)]
    pub required: bool,

    #[prop_or_default]
    pub style: AttrValue,

    // TODO(Shaohua): Convert value type to any.
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
        position::position_class(props.label_placement),
        if props.error {
            "ZuFormControlLabel-error"
        } else {
            ""
        },
        if props.required {
            "ZuFormControlLabel-required"
        } else {
            ""
        }
    );

    let label_cls = classes! {
        "ZuFormControlLable-label",
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
    html! {
        <label class={root_cls} style={&props.style}>
            <Typography classes={label_cls}>
                {props.label.clone()}
            </Typography>

            if props.required {
                <span class={asterisk_cls} aria-hidden="true">
                    {"&thinsp;{'*'}"}
                </span>
            }
        </label>
    }
}
