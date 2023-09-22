// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ReplayCircleFilledTwoTone)]
pub fn replay_circle_filled_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ReplayCircleFilledTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M18,12.5c0,3.31-2.69,6-6,6s-6-2.69-6-6h2 c0,2.21,1.79,4,4,4s4-1.79,4-4s-1.79-4-4-4v3l-4-4l4-4v3C15.31,6.5,18,9.19,18,12.5z"/>
        </SvgIcon>
    }
}
