// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod position;

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::styles::button_variant::ButtonVariant;
pub use position::Position;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or(false)]
    pub loading: bool,

    #[prop_or_default]
    pub loading_indicator: AttrValue,

    #[prop_or_default]
    pub loading_position: Position,

    #[prop_or_default]
    pub start_icon: Option<Html>,

    #[prop_or_default]
    pub end_icon: Option<Html>,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub variant: ButtonVariant,
}

#[function_component(LoadingButton)]
pub fn loading_button(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
