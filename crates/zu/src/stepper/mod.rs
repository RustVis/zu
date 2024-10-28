// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::styles::orientation::Orientation;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(0)]
    pub active_step: i32,

    #[prop_or(false)]
    pub alternative_label: bool,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub component: AttrValue,
    // TODO(Shaohua): Add connector component.
    #[prop_or(false)]
    pub non_linear: bool,

    #[prop_or_default]
    pub orientation: Orientation,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(Stepper)]
pub fn stepper(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
