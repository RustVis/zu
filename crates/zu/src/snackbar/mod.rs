// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, AttrValue, Callback, Children, Classes, Html, Properties};

use crate::styles::anchor_origin::AnchorOrigin;
use crate::styles::transition_duration::TransitionDuration;

// TODO(Shaohua): Replace AnchorOrigin with custom type.

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub action: Option<Html>,

    #[prop_or(AnchorOrigin::bottom_left())]
    pub anchor_origin: AnchorOrigin,

    #[prop_or_default]
    pub auto_hide_duration: Option<i32>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(false)]
    pub disable_window_blur_listener: bool,

    #[prop_or_default]
    pub key: AttrValue,

    #[prop_or_default]
    pub message: Option<Html>,

    #[prop_or_default]
    pub on_close: Option<Callback<()>>,

    #[prop_or(false)]
    pub open: bool,

    #[prop_or_default]
    pub resume_hide_duration: Option<i32>,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub transition_duration: TransitionDuration,
}

#[function_component(Snackbar)]
pub fn snackbar(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
