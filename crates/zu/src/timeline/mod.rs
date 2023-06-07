// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod position;

use yew::{function_component, html, AttrValue, Children, Html, Properties};

pub use position::Position;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub class_name: AttrValue,

    #[prop_or_default]
    pub position: Position,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(Timeline)]
pub fn timeline(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
