// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::styles::item_align::ItemAlign;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub align_items: ItemAlign,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub component: AttrValue,
    // TODO(Shaohua): Add container component
    /// If true, compact vertical padding designed for keyboard and mouse input is used.
    #[prop_or(false)]
    pub dense: bool,

    /// If true, the left and right padding is removed.
    #[prop_or(false)]
    pub disable_gutters: bool,

    /// If true, all padding is removed.
    #[prop_or(false)]
    pub disable_padding: bool,

    /// If true, a 1px light border is added to the bottom of the list item.
    #[prop_or(false)]
    pub divider: bool,

    /// If true, a 1px light border is added to the bottom of the list item.
    #[prop_or_default]
    pub secondary_action: Option<Html>,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(ListItem)]
pub fn list_item(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
