// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

mod track;
mod value_label;

use yew::{function_component, html, AttrValue, Callback, Classes, Html, Properties};

use crate::styles::color::PrimaryColor;
use crate::styles::orientation::Orientation;
use crate::styles::size::Size;
pub use track::Track;
pub use value_label::ValueLabel;

// TODO(Shaohua): Add SliderValueLabel class

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub aria_label: AttrValue,

    #[prop_or_default]
    pub aria_labelled_by: AttrValue,

    #[prop_or_default]
    pub aria_value_text: AttrValue,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub color: PrimaryColor,

    #[prop_or_default]
    pub default_value: Vec<i32>,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or(false)]
    pub disable_swap: bool,

    #[prop_or(false)]
    pub marks: bool,

    #[prop_or(100)]
    pub max: i32,

    #[prop_or(0)]
    pub min: i32,

    #[prop_or_default]
    pub name: AttrValue,

    #[prop_or_default]
    pub on_change: Option<Callback<()>>,

    #[prop_or_default]
    pub on_change_committed: Option<Callback<()>>,

    #[prop_or_default]
    pub orientation: Orientation,

    #[prop_or_default]
    pub scale: Option<Callback<()>>,

    #[prop_or_default]
    pub size: Size,

    #[prop_or(1)]
    pub step: i32,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub tab_index: Option<i32>,

    #[prop_or_default]
    pub track: Track,

    #[prop_or_default]
    pub value: Vec<i32>,

    #[prop_or_default]
    pub value_label_display: ValueLabel,
}

#[function_component(Slider)]
pub fn slider(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
