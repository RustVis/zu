// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(false)]
    pub error: bool,

    #[prop_or_default]
    pub icon: Option<Html>,

    #[prop_or_default]
    pub optional: Option<Html>,
    // TODO(Shaohua): Add StepIconComponent
    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(StepLabel)]
pub fn step_label(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
