// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AccessibleForwardRounded)]
pub fn accessible_forward_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AccessibleForwardRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M15 17h-2c0 1.65-1.35 3-3 3s-3-1.35-3-3 1.35-3 3-3v-2c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5zm3-3.5h-1.86l1.67-3.67C18.42 8.5 17.44 7 15.96 7h-5.2c-.81 0-1.54.47-1.87 1.2l-.28.76c-.21.56.11 1.17.68 1.33.49.14 1-.11 1.2-.58l.3-.71H13l-1.83 4.1c-.6 1.33.39 2.9 1.85 2.9H18v4c0 .55.45 1 1 1s1-.45 1-1v-4.5c0-1.1-.9-2-2-2z"/>
        </SvgIcon>
    }
}
