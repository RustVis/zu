// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, AttrValue, Classes, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(false)]
    pub active: bool,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(false)]
    pub completed: bool,

    #[prop_or(false)]
    pub error: bool,

    #[prop_or_default]
    pub icon: Option<Html>,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(StepIcon)]
pub fn step_icon(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
