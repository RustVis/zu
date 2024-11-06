// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewCompactRounded)]
pub fn view_compact_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewCompactRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,18h2.5v-2.5H4V18z M4,13.25h2.5v-2.5H4V13.25z M4,8.5h2.5V6H4V8.5z M17.5,6v2.5H20V6H17.5z M13,8.5h2.5V6H13V8.5z M17.5,18H20v-2.5h-2.5V18z M17.5,13.25H20v-2.5h-2.5V13.25z M8.5,18H11v-2.5H8.5V18z M13,18h2.5v-2.5H13V18z M8.5,8.5H11V6H8.5 V8.5z M13,13.25h2.5v-2.5H13V13.25z M8.5,13.25H11v-2.5H8.5V13.25z"/>
        </SvgIcon>
    }
}
