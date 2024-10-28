// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, Callback, Children, Html, Properties};

use crate::styles::transition_duration::TransitionDuration;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub on_close: Callback<()>,

    pub on_open: Callback<()>,

    #[prop_or(false)]
    pub allow_swipe_in_children: bool,

    #[prop_or_default]
    pub children: Children,

    #[prop_or(false)]
    pub disable_backdrop_transition: bool,

    #[prop_or(false)]
    pub disable_discovery: bool,

    #[prop_or(false)]
    pub disable_swipe_to_open: bool,

    #[prop_or(0.52)]
    pub hysteresis: f64,

    #[prop_or(450)]
    pub min_fling_velocity: i32,

    #[prop_or(false)]
    pub open: bool,

    #[prop_or(20)]
    pub swipe_area_width: i32,

    #[prop_or_default]
    pub transition_duration: TransitionDuration,
}

#[function_component(SwipeableDrawer)]
pub fn swipeable_drawer(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
