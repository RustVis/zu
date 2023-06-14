// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::color::Color;
use crate::styles::size::Size;
use yew::{function_component, html, AttrValue, Callback, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(false)]
    pub checked: bool,

    #[prop_or_default]
    pub checked_icon: Option<Html>,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub color: Color,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or(false)]
    pub disable_ripple: bool,

    #[prop_or_default]
    pub icon: Option<Html>,

    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub name: AttrValue,

    #[prop_or_default]
    pub on_change: Option<Callback<()>>,

    #[prop_or(false)]
    pub required: bool,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub value: AttrValue,
}

#[function_component(Radio)]
pub fn radio(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
