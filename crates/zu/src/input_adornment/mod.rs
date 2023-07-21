// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod horizontal_position;

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::styles::label_variant::LabelVariant;
pub use horizontal_position::HorizontalPosition;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub position: HorizontalPosition,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or(false)]
    pub disable_pointer_events: bool,

    #[prop_or(false)]
    pub disable_typography: bool,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub variant: LabelVariant,
}

#[function_component(InputAdornment)]
pub fn input_adornment(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
