// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

mod scroll_type;

use yew::{function_component, html, AttrValue, Callback, Children, Classes, Html, Properties};

use crate::styles::size::MaxWidth;
use crate::styles::transition_duration::TransitionDuration;
pub use scroll_type::ScrollType;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub open: bool,

    #[prop_or_default]
    pub aria_described_by: AttrValue,

    #[prop_or_default]
    pub aria_labelled_by: AttrValue,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(false)]
    pub disable_escape_key_down: bool,

    #[prop_or(false)]
    pub fullscreen: bool,

    #[prop_or(false)]
    pub full_width: bool,

    /// If max-width is None, it is disabled.
    #[prop_or(Some(MaxWidth::Small))]
    pub max_width: Option<MaxWidth>,

    #[prop_or_default]
    pub on_backdrop_click: Option<Callback<()>>,

    #[prop_or_default]
    pub on_close: Option<Callback<()>>,

    #[prop_or_default]
    pub scroll: ScrollType,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub transition_duration: TransitionDuration,
}

#[function_component(Dialog)]
pub fn dialog(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
