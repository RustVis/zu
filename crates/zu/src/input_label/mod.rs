// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod margin;
mod variant;

use yew::{function_component, html, AttrValue, Children, Html, Properties};

use crate::styles::{color::Color, size::Size};

// Re-export
use margin::MarginType;
use variant::Variant;

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
    pub focus: bool,

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
    pub variant: Variant,
}

#[function_component(InputLabel)]
pub fn input_label(props: &Props) -> Html {
    html! {
       <div style={&props.style}>
       </div>
    }
}
