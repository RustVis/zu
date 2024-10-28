// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

mod easing;

use yew::{function_component, html, Callback, Children, Html, Properties};

use crate::styles::transition_duration::{Easing, TransitionDuration};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,

    #[prop_or_default]
    pub add_end_listener: Option<Callback<()>>,

    #[prop_or(true)]
    pub appear: bool,

    #[prop_or(easing::default_easing())]
    pub easing: Easing,

    #[prop_or(false)]
    pub is_transition_in: bool,

    #[prop_or_default]
    pub timeout: TransitionDuration,
}

#[function_component(Fade)]
pub fn fade(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
