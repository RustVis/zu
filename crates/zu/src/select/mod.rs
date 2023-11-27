// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Callback, Children, Classes, Html, Properties};

use crate::styles::label_variant::LabelVariant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub aria_described_by: AttrValue,

    #[prop_or(false)]
    pub auto_width: bool,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(false)]
    pub default_open: bool,

    #[prop_or_default]
    pub default_value: AttrValue,

    #[prop_or(false)]
    pub display_empty: bool,

    #[prop_or_default]
    pub icon_component: Option<Html>,

    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub input: Option<Html>,

    #[prop_or_default]
    pub label: Option<Html>,

    #[prop_or_default]
    pub label_id: AttrValue,

    #[prop_or(false)]
    pub multiple: bool,

    #[prop_or(false)]
    pub native: bool,

    #[prop_or_default]
    pub on_change: Option<Callback<()>>,

    #[prop_or_default]
    pub on_close: Option<Callback<()>>,

    #[prop_or_default]
    pub on_open: Option<Callback<()>>,

    #[prop_or(false)]
    pub open: bool,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or(LabelVariant::Outlined)]
    pub variant: LabelVariant,

    #[prop_or_default]
    pub value: AttrValue,
}

#[function_component(Select)]
pub fn select(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
