// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::styles::label_variant::SimpleLabelVariant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub variant: SimpleLabelVariant,
}

#[function_component(TimelineDot)]
pub fn timeline_dot(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
