// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CompareArrows)]
pub fn compare_arrows(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CompareArrows"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9.01,14H2v2h7.01v3L13,15l-3.99-4V14z M14.99,13v-3H22V8h-7.01V5L11,9L14.99,13z"/>
        </SvgIcon>
    }
}
