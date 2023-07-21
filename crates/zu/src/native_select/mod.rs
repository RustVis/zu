// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::label_variant::LabelVariant;
use yew::{function_component, html, AttrValue, Callback, Children, Classes, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub icon_component: Option<Html>,

    #[prop_or_default]
    pub input: Option<Html>,

    #[prop_or_default]
    pub on_change: Option<Callback<()>>,

    #[prop_or_default]
    pub style: AttrValue,

    // TODO(Shaohua): Replace with Any.
    #[prop_or_default]
    pub value: AttrValue,

    #[prop_or_default]
    pub variant: LabelVariant,
}

#[function_component(NativeSelect)]
pub fn native_select(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
