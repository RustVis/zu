// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Callback, Classes, Html, Properties};

use crate::styles::color::Color;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub auto_complete: AttrValue,

    #[prop_or_default]
    pub auto_focus: bool,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub color: Color,

    // pub default_value: AttrValue,
    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or(false)]
    pub disable_underline: bool,

    #[prop_or_default]
    pub end_adornment: Option<Html>,

    #[prop_or(false)]
    pub error: bool,

    #[prop_or(false)]
    pub full_width: bool,

    #[prop_or(false)]
    pub hidden_label: bool,

    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub input_component: AttrValue,

    #[prop_or(false)]
    pub dense_margin: bool,

    #[prop_or_default]
    pub max_rows: Option<i32>,

    #[prop_or_default]
    pub min_rows: Option<i32>,

    #[prop_or(false)]
    pub multiline: bool,

    #[prop_or_default]
    pub name: AttrValue,

    // TODO(Shaohua): Add change event.
    #[prop_or_default]
    pub on_change: Option<Callback<()>>,

    #[prop_or_default]
    pub placeholder: AttrValue,

    #[prop_or(false)]
    pub read_only: bool,

    #[prop_or(false)]
    pub required: bool,

    #[prop_or_default]
    pub start_adornment: Option<Html>,

    #[prop_or_default]
    pub style: AttrValue,

    /// Default is "text".
    #[prop_or_default]
    pub input_type: AttrValue,

    #[prop_or_default]
    pub value: AttrValue,
}

#[function_component(FilledInput)]
pub fn filled_input(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
