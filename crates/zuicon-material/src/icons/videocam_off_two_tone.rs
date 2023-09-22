// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VideocamOffTwoTone)]
pub fn videocam_off_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VideocamOffTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M12.39 8L15 10.61V8zM5 8v8h9.73l-8-8z" opacity=".3"/><path d="M3.41 1.86L2 3.27 4.73 6H4c-.55 0-1 .45-1 1v10c0 .55.45 1 1 1h12c.21 0 .39-.08.55-.18L19.73 21l1.41-1.41L3.41 1.86zM5 16V8h1.73l8 8H5zm10-8v2.61l6 6V6.5l-4 4V7c0-.55-.45-1-1-1h-5.61l2 2H15z"/>
        </SvgIcon>
    }
}
