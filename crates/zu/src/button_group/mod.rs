// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Html, Properties};

use crate::button::Variant;
use crate::styles::color::Color;
use crate::styles::orientation::Orientation;
use crate::styles::size::Size;

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

    /// If true, the component is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// If true, no elevation is used.
    #[prop_or(false)]
    pub disable_elevation: bool,

    /// If true, the button keyboard focus ripple is disabled.
    #[prop_or(false)]
    pub disable_focus_ripple: bool,

    /// If true, the button ripple effect is disabled.
    #[prop_or(false)]
    pub disable_ripple: bool,

    /// If true, the buttons will take up the full width of its container.
    #[prop_or(false)]
    pub full_width: bool,

    /// The component orientation (layout flow direction).
    #[prop_or_default]
    pub orientation: Orientation,

    /// The size of the component. small is equivalent to the dense button styling.
    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

    /// The variant to use.
    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(ButtonGroup)]
pub fn button_group(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
