// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

mod variant;

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::styles::color::Color;
use crate::styles::size::Size;
pub use variant::Variant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(Color::Default)]
    pub color: Color,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or(false)]
    pub disable_focus_ripple: bool,

    #[prop_or(false)]
    pub disable_ripple: bool,

    #[prop_or_default]
    pub href: AttrValue,

    #[prop_or(Size::Large)]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(Fab)]
pub fn fab(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
