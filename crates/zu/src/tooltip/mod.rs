// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Callback, Children, Classes, Html, Properties};

use crate::styles::placement::Placement;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,

    #[prop_or(false)]
    pub arrow: bool,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(false)]
    pub describe_child: bool,

    #[prop_or(false)]
    pub disable_focus_listener: bool,

    #[prop_or(false)]
    pub disable_hover_listener: bool,

    #[prop_or(false)]
    pub disable_interactive: bool,

    #[prop_or(false)]
    pub disable_touch_listener: bool,

    #[prop_or(100)]
    pub enter_delay: i32,

    #[prop_or(0)]
    pub enter_next_delay: i32,

    #[prop_or(700)]
    pub enter_touch_delay: i32,

    #[prop_or(false)]
    pub follow_cursor: bool,

    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or(0)]
    pub leave_delay: i32,

    #[prop_or(1500)]
    pub leave_touch_delay: i32,

    #[prop_or_default]
    pub on_close: Option<Callback<()>>,

    #[prop_or_default]
    pub on_open: Option<Callback<()>>,

    #[prop_or(false)]
    pub open: bool,

    #[prop_or(Placement::Bottom)]
    pub placement: Placement,

    #[prop_or_default]
    pub popper_component: Option<Html>,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub title: AttrValue,

    #[prop_or_default]
    pub transition_component: Option<Html>,
}

#[function_component(Tooltip)]
pub fn tooltip(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
