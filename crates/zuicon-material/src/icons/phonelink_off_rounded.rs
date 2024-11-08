// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PhonelinkOffRounded)]
pub fn phonelink_off_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PhonelinkOffRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M24 19V9c0-.55-.45-1-1-1h-6c-.55 0-1 .45-1 1v3.61l2 2V10h4v7h-1.61l2.93 2.93c.39-.13.68-.49.68-.93zM21 6c.55 0 1-.45 1-1s-.45-1-1-1H7.39l2 2H21zM1.36 2.21c-.39.39-.39 1.02 0 1.41l1.11 1.11C2.18 5.08 2 5.52 2 6v11h-.5c-.83 0-1.5.67-1.5 1.5S.67 20 1.5 20h16.23l1.64 1.64c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L2.77 2.21c-.39-.39-1.02-.39-1.41 0zM4 17V6.27L14.73 17H4z"/>
        </SvgIcon>
    }
}
