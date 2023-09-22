// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FormatTextdirectionRToLTwoTone)]
pub fn format_textdirection_r_to_l_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FormatTextdirectionRToLTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M8 6c0 1.1.9 2 2 2V4c-1.1 0-2 .9-2 2z" opacity=".3"/><path d="M6 6c0 2.21 1.79 4 4 4v5h2V4h2v11h2V4h2V2h-8C7.79 2 6 3.79 6 6zm4 2c-1.1 0-2-.9-2-2s.9-2 2-2v4zM4 18l4 4v-3h12v-2H8v-3z"/>
        </SvgIcon>
    }
}
