// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Callback, Classes, Html, Properties};

use crate::styles::color::Color;
use crate::styles::input_type::InputType;
use crate::styles::label_variant::LabelVariant;
use crate::styles::margin::Margin;
use crate::styles::size::Size;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub auto_complete: AttrValue,

    #[prop_or(false)]
    pub auto_focus: bool,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub color: Color,

    #[prop_or_default]
    pub default_value: AttrValue,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or(false)]
    pub error: bool,

    #[prop_or(false)]
    pub full_width: bool,

    #[prop_or_default]
    pub helper_text: AttrValue,

    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub label: Html,

    #[prop_or_default]
    pub margin: Margin,

    #[prop_or_default]
    pub max_rows: Option<i32>,

    #[prop_or_default]
    pub min_rows: Option<i32>,

    #[prop_or(false)]
    pub multiline: bool,

    #[prop_or_default]
    pub name: AttrValue,

    #[prop_or_default]
    pub on_change: Option<Callback<AttrValue>>,

    #[prop_or_default]
    pub placeholder: AttrValue,

    #[prop_or(false)]
    pub required: bool,

    #[prop_or_default]
    pub rows: Option<i32>,

    #[prop_or(false)]
    pub select: bool,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub input_type: InputType,

    #[prop_or(false)]
    pub input_read_only: bool,

    #[prop_or(false)]
    pub input_shrink: bool,

    #[prop_or_default]
    pub value: AttrValue,

    #[prop_or_default]
    pub variant: LabelVariant,
}

#[function_component(TextField)]
pub fn text_field(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
