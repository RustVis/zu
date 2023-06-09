// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Html, Properties};

use crate::styles::placement::Placement;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or(0)]
    pub delay: i32,

    #[prop_or_default]
    pub icon: Option<Html>,

    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or(false)]
    pub open: bool,

    #[prop_or_default]
    pub style: AttrValue,

    //TODO(Shaohua): Add tooltip class.
    #[prop_or(false)]
    pub tooltip_open: bool,

    #[prop_or_default]
    pub tooltip_placement: Placement,

    #[prop_or_default]
    pub tooltip_title: Option<Html>,
}

#[function_component(SpeedDialAction)]
pub fn speed_dial_action(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
