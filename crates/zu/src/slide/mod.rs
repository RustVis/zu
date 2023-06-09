// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod easing;
mod transition_duration;

use yew::{function_component, html, Callback, Children, Html, Properties};

use crate::styles::direction::Direction;
use crate::styles::transition_duration::{Easing, TransitionDuration};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,

    #[prop_or_default]
    pub add_event_listener: Option<Callback<()>>,

    #[prop_or(true)]
    pub appear: bool,

    #[prop_or_default]
    pub container: Option<Html>,

    #[prop_or(Direction::Down)]
    pub direction: Direction,

    #[prop_or(easing::default_easing())]
    pub easing: Easing,

    #[prop_or(true)]
    pub is_transition_in: bool,

    #[prop_or(transition_duration::default_duration())]
    pub timeout: TransitionDuration,
}

#[function_component(Slide)]
pub fn slide(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
