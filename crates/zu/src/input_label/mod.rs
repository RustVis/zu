// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod margin;
mod size;
mod variant;

use yew::{classes, function_component, html, AttrValue, Children, Html, Properties};

use crate::form_label::FormLabel;
use crate::styles::color::Color;
use crate::styles::label_variant::LabelVariant;
use crate::styles::size::Size;
pub use margin::MarginType;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub color: Color,

    /// If true, the transition animation is disabled.
    #[prop_or(false)]
    pub disable_animation: bool,

    /// If true, the component is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// If true, the label is displayed in an error state.
    #[prop_or(false)]
    pub error: bool,

    /// If true, the input of this label is focused.
    #[prop_or(false)]
    pub focused: bool,

    /// If dense, will adjust vertical spacing. This is normally obtained via context from FormControl.
    #[prop_or_default]
    pub margin: MarginType,

    /// If true, the label will indicate that the input is required.
    #[prop_or(false)]
    pub required: bool,

    /// If true, the label is shrunk.
    #[prop_or(false)]
    pub shrink: bool,

    /// The size of the component.
    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub variant: LabelVariant,
}

#[function_component(InputLabel)]
pub fn input_label(props: &Props) -> Html {
    // TODO(Shaohua): Add form control state.
    let root_cls = classes!(
        "ZuInputLabel-root",
        // if props.form_control {
        //     "ZuInputLabel-formControl"
        // } else {
        //     ""
        // },
        if props.disable_animation {
            ""
        } else {
            "ZuInputLabel-animated"
        },
        if props.shrink {
            "ZuInputLabel-shrink"
        } else {
            ""
        },
        size::css_class(props.size),
        variant::css_class(props.variant),
    );
    let _asterisk_class = if props.required {
        "ZuInputLabel-asterisk"
    } else {
        ""
    };
    // TODO(Shaohua): Merge props.size into styles.

    html! {
        <FormLabel classes={root_cls}
            color={props.color}
            disabled={props.disabled}
            error={props.error}
            focused={props.focused}
            required={props.required}
            style={&props.style}>
            {for props.children.iter()}
        </FormLabel>
    }
}
