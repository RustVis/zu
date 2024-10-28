// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::styles::direction::HorizontalDirection;
use crate::styles::orientation::Orientation;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub direction: HorizontalDirection,

    pub orientation: Orientation,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(TabScrollButton)]
pub fn tab_scroll_button(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
