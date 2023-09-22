// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AlignVerticalBottomTwoTone)]
pub fn align_vertical_bottom_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AlignVerticalBottomTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,22H2v-2h20V22z M10,2H7v16h3V2z M17,8h-3v10h3V8z"/>
        </SvgIcon>
    }
}
