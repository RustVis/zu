// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Callback, Children, Html, Properties};

use crate::styles::orientation::Orientation;
use crate::styles::transition_duration::TransitionDuration;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub add_end_listener: Option<Callback<()>>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub collapsed_size: i32,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or(false)]
    pub is_transition_in: bool,

    #[prop_or(Orientation::Vertical)]
    pub orientation: Orientation,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or(TransitionDuration::standard())]
    pub timeout: TransitionDuration,
}

#[function_component(Collapse)]
pub fn collapse(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
