// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, Callback, Children, Html, Properties};

use crate::styles::transition_duration::{Easing, TransitionDuration};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    pub add_end_listener: Option<Callback<()>>,

    #[prop_or(true)]
    pub appear: bool,

    #[prop_or_default]
    pub easing: Option<Easing>,

    #[prop_or(true)]
    pub is_transition_in: bool,

    #[prop_or(TransitionDuration::Auto)]
    pub timeout: TransitionDuration,
}

#[function_component(Grow)]
pub fn grow(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
