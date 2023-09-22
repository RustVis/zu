// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(StopTwoTone)]
pub fn stop_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("StopTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M8 8h8v8H8z" opacity=".3"/><path d="M6 18h12V6H6v12zM8 8h8v8H8V8z"/>
        </SvgIcon>
    }
}
