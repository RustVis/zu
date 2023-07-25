// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Callback, Classes, Html, Properties};

use crate::styles::size::Size;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub default_value: Option<f64>,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or_default]
    pub empty_icon: Option<Html>,

    #[prop_or_default]
    pub empty_label_text: Option<Html>,

    #[prop_or(false)]
    pub highlight_selected_only: bool,

    #[prop_or_default]
    pub icon: Option<Html>,

    #[prop_or(5.0)]
    pub max: f64,

    #[prop_or_default]
    pub name: AttrValue,

    #[prop_or_default]
    pub on_change: Option<Callback<i32>>,

    #[prop_or_default]
    pub on_change_active: Option<Callback<i32>>,

    #[prop_or(1.0)]
    pub precision: f64,

    #[prop_or(false)]
    pub read_only: bool,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub value: Option<f64>,
}

#[function_component(Rating)]
pub fn rating(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
