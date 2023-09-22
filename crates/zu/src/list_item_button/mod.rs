// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::styles::item_align::ItemAlign;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub align_items: ItemAlign,

    #[prop_or(false)]
    pub auto_focus: bool,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or(false)]
    pub dense: bool,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or(false)]
    pub disable_gutters: bool,

    #[prop_or(false)]
    pub divider: bool,

    #[prop_or_default]
    pub focus_visible_class_name: AttrValue,

    #[prop_or(false)]
    pub selected: bool,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(ListItemButton)]
pub fn list_item_button(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
