// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Timer10SelectSharp)]
pub fn timer_10_select_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Timer10SelectSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13,8v8h-3V8H13 M16,5H7v14h9V5z M1,8h2v11h3V5H1V8z M23,11h-6v5h4v1h-4v2h6v-5h-4v-1h4V11z"/>
        </SvgIcon>
    }
}
