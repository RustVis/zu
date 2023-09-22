// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

mod variant;

use yew::{
    function_component, html, AttrValue, Callback, Children, Classes, Html, MouseEvent, Properties,
};

use crate::styles::side::Side;
use crate::styles::transition_duration::TransitionDuration;
pub use variant::Variant;

const DEFAULT_ELEVATION: i32 = 16;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(Side::Left)]
    pub anchor: Side,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(DEFAULT_ELEVATION)]
    pub elevation: i32,

    #[prop_or(false)]
    pub hide_backdrop: bool,

    #[prop_or_default]
    pub on_close: Option<Callback<MouseEvent>>,

    #[prop_or(false)]
    pub open: bool,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub transition_duration: TransitionDuration,

    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(Drawer)]
pub fn drawer(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
