// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::placement::Placement;
use yew::{function_component, html, AttrValue, Children, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub open: bool,

    #[prop_or_default]
    pub anchor_element: Option<Html>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or_default]
    pub container: Option<Html>,

    #[prop_or(false)]
    pub disable_portal: bool,

    #[prop_or(false)]
    pub keep_mounted: bool,
    // TODO(Shaohua): Replace modifiers in popper.js
    //pub modifiers: AttrValue,
    #[prop_or(Placement::Left)]
    pub placement: Placement,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or(false)]
    pub transition: bool,
}

#[function_component(Popper)]
pub fn popper(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
