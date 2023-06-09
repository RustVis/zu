// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Html, Properties};

use crate::styles::transition_duration::TransitionDuration;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub style: AttrValue,

    // TODO(Shaohua): Add TransitionComponent
    // TODO(Shaohua): Add tranition props
    #[prop_or(TransitionDuration::Auto)]
    pub transition_duration: TransitionDuration,
}

#[function_component(StepContent)]
pub fn step_content(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
