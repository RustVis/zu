// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod margin;

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::styles::color::Color;
use crate::styles::label_variant::LabelVariant;
use crate::styles::margin::Margin;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub color: Color,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or(false)]
    pub disabled: bool,

    /// If true, the label is displayed in an error state.
    #[prop_or(false)]
    pub error: bool,

    /// If true, the component is displayed in focused state.
    #[prop_or(false)]
    pub focused: bool,

    /// If true, the component will take up the full width of its container.
    #[prop_or(false)]
    pub full_width: bool,

    /// If true, the label is hidden.
    #[prop_or(false)]
    pub hidden_label: bool,

    #[prop_or_default]
    pub margin: Margin,

    #[prop_or(false)]
    pub required: bool,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or(LabelVariant::Outlined)]
    pub variant: LabelVariant,
}

#[function_component(FormControl)]
pub fn form_control(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
