// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

mod variant;

use yew::{function_component, html, AttrValue, Callback, Children, Classes, Html, Properties};

use crate::styles::transition_duration::TransitionDuration;
pub use variant::Variant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub open: bool,

    #[prop_or_default]
    pub anchor_element: Option<Html>,

    #[prop_or(true)]
    pub auto_focus: bool,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(false)]
    pub disable_auto_focus_item: bool,

    #[prop_or_default]
    pub on_close: Option<Callback<()>>,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub transition_duration: TransitionDuration,

    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(Menu)]
pub fn menu(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
