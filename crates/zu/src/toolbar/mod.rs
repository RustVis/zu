// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod variant;

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

pub use variant::Variant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or(false)]
    pub disable_gutters: bool,

    #[prop_or_default]
    pub style: AttrValue,

    pub variant: Variant,
}

#[function_component(Toolbar)]
pub fn toolbar(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
