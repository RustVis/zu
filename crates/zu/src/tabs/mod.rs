// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

mod behavior;
mod variant;

use yew::{function_component, html, AttrValue, Callback, Children, Classes, Html, Properties};

use crate::styles::color::{Color, PrimaryColor};
use crate::styles::orientation::Orientation;
pub use behavior::ButtonBehavior;
pub use variant::Variant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(false)]
    pub allow_scroll_buttons_mobile: bool,

    #[prop_or_default]
    pub aria_label: AttrValue,

    #[prop_or_default]
    pub aria_labeled_by: AttrValue,

    #[prop_or(false)]
    pub centered: bool,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or_default]
    pub indicator_color: PrimaryColor,

    #[prop_or_default]
    pub on_change: Option<Callback<(i32, i32)>>,

    #[prop_or_default]
    pub orientation: Orientation,

    #[prop_or_default]
    pub scroll_button_component: Option<Html>,

    #[prop_or_default]
    pub scroll_button_behavior: ButtonBehavior,

    #[prop_or(false)]
    pub selection_follows_focus: bool,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub text_color: Color,

    #[prop_or_default]
    pub value: i32,

    #[prop_or_default]
    pub variant: Variant,

    #[prop_or(false)]
    pub scrollbar_visible: bool,
}

#[function_component(Tabs)]
pub fn tabs(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
