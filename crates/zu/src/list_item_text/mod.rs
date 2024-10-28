// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(false)]
    pub disable_typography: bool,

    #[prop_or(false)]
    pub inset: bool,

    #[prop_or_default]
    pub primary: Option<Html>,

    #[prop_or_default]
    pub secondary: Option<Html>,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(ListItemText)]
pub fn list_item_text(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
