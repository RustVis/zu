// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, Children, Html, Properties};

use crate::menu::Variant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(false)]
    pub auto_focus: bool,

    #[prop_or(false)]
    pub auto_focus_item: bool,

    #[prop_or_default]
    pub children: Children,

    #[prop_or(false)]
    pub disabled_items_focusable: bool,

    #[prop_or(false)]
    pub disable_list_wrap: bool,

    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(MenuItem)]
pub fn menu_item(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
