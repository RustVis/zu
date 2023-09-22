// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Timer3SelectSharp)]
pub fn timer_3_select_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Timer3SelectSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,11v2h-4v1h4v5h-6v-2h4v-1h-4v-5H21z M4,5v3h6v2.5H4v3h6V16H4v3h9V5H4z"/>
        </SvgIcon>
    }
}
