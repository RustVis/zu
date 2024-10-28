// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{
    function_component, html, AttrValue, Callback, Children, Classes, Html, MouseEvent, Properties,
};

use crate::styles::color::Color;
use crate::styles::label_variant::SimpleLabelVariant;
use crate::styles::size::Size;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub avatar: Option<Html>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(false)]
    pub clickable: bool,

    #[prop_or(Color::Default)]
    pub color: Color,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or_default]
    pub delete_icon: Option<Html>,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or_default]
    pub href: AttrValue,

    #[prop_or_default]
    pub icon: Option<Html>,

    #[prop_or_default]
    pub label: Option<Html>,

    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub on_delete: Option<Callback<()>>,

    #[prop_or(Size::Medium)]
    pub size: Size,

    #[prop_or(false)]
    pub skip_focus_when_disabled: bool,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub variant: SimpleLabelVariant,
}

#[function_component(Chip)]
pub fn chip(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
