// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

mod event_type;

use yew::{function_component, html, Callback, Children, Html, Properties};

pub use event_type::{MouseEventType, TouchEventType};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,

    pub on_click_away: Callback<()>,

    #[prop_or(Some(MouseEventType::default()))]
    pub mouse_event: Option<MouseEventType>,

    #[prop_or(Some(TouchEventType::default()))]
    pub touch_event: Option<TouchEventType>,
}

#[function_component(ClickAwayListener)]
pub fn click_away_listener(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
