// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Callback, Children, Classes, Html, Properties};

use crate::styles::anchor_origin::AnchorOrigin;
use crate::styles::position::PositionValue;
use crate::styles::transition_duration::TransitionDuration;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(false)]
    pub open: bool,

    #[prop_or_default]
    pub anchor_element: Option<Html>,

    #[prop_or_default]
    pub anchor_origin: AnchorOrigin,

    #[prop_or_default]
    pub anchor_position: Option<PositionValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub container: Option<Html>,

    #[prop_or(8)]
    pub elevation: i32,

    #[prop_or(16)]
    pub margin_threshold: i32,

    #[prop_or_default]
    pub on_close: Option<Callback<()>>,

    #[prop_or_default]
    pub style: AttrValue,

    pub transform_origin: AnchorOrigin,

    #[prop_or_default]
    pub transition_component: Option<Html>,

    #[prop_or(TransitionDuration::Auto)]
    pub transition_duration: TransitionDuration,
}

#[function_component(Popover)]
pub fn popover(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
