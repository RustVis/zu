// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FormatTextdirectionLToRTwoTone)]
pub fn format_textdirection_l_to_r_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FormatTextdirectionLToRTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M9 8V4c-1.1 0-2 .9-2 2s.9 2 2 2z" opacity=".3"/><path d="M9 10v5h2V4h2v11h2V4h2V2H9C6.79 2 5 3.79 5 6s1.79 4 4 4zm0-6v4c-1.1 0-2-.9-2-2s.9-2 2-2zm12 14l-4-4v3H5v2h12v3z"/>
        </SvgIcon>
    }
}
