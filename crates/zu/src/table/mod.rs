// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod padding;

use yew::{function_component, html, AttrValue, Children, Html, Properties};

use crate::styles::size::Size;
pub use padding::Padding;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or_default]
    pub padding: Padding,

    #[prop_or_default]
    pub size: Size,

    #[prop_or(false)]
    pub sticky_header: bool,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(Table)]
pub fn table(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
