// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Callback, Children, Classes, Html, Properties};

use crate::styles::direction::Direction;
use crate::styles::transition_duration::TransitionDuration;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub arial_label: AttrValue,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub direction: Direction,

    #[prop_or(false)]
    pub hidden: bool,

    #[prop_or_default]
    pub icon: Option<Html>,

    #[prop_or_default]
    pub on_close: Option<Callback<()>>,

    #[prop_or_default]
    pub on_open: Option<Callback<()>>,

    #[prop_or(false)]
    pub open: bool,

    #[prop_or_default]
    pub open_icon: Option<Html>,

    #[prop_or_default]
    pub style: AttrValue,

    // TODO(Shaohua): Add TransitionComponent
    #[prop_or_default]
    pub transition_duration: TransitionDuration,
}

#[function_component(SpeedDial)]
pub fn speed_dial(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
