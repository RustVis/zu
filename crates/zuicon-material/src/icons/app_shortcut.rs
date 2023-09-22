// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AppShortcut)]
pub fn app_shortcut(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AppShortcut"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17 18H7V6h10v1h2V3c0-1.1-.9-2-2-2H7c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2v-4h-2v1z"/>
        </SvgIcon>
    }
}
