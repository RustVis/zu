// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub value: AttrValue,

    #[prop_or_default]
    pub children: Children,
}

#[function_component(TabContent)]
pub fn tab_content(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
