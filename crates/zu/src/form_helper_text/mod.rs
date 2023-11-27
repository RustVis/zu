// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::styles::label_variant::LabelVariant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or(false)]
    pub error: bool,

    #[prop_or(false)]
    pub filled: bool,

    #[prop_or(false)]
    pub focused: bool,

    #[prop_or(true)]
    pub dense_margin: bool,

    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or(false)]
    pub required: bool,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub variant: LabelVariant,
}

#[function_component(FormHelperText)]
pub fn form_helper_text(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
