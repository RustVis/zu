// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewSidebar)]
pub fn view_sidebar(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewSidebar"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16,20H2V4h14V20z M18,8h4V4h-4V8z M18,20h4v-4h-4V20z M18,14h4v-4h-4V14z"/>
        </SvgIcon>
    }
}
