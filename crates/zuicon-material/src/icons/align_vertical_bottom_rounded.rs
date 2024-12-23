// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AlignVerticalBottomRounded)]
pub fn align_vertical_bottom_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AlignVerticalBottomRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,22H3c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h18c0.55,0,1,0.45,1,1v0C22,21.55,21.55,22,21,22z M8.5,2L8.5,2 C7.67,2,7,2.67,7,3.5v13C7,17.33,7.67,18,8.5,18h0c0.83,0,1.5-0.67,1.5-1.5v-13C10,2.67,9.33,2,8.5,2z M15.5,8L15.5,8 C14.67,8,14,8.67,14,9.5v7c0,0.83,0.67,1.5,1.5,1.5h0c0.83,0,1.5-0.67,1.5-1.5v-7C17,8.67,16.33,8,15.5,8z"/>
        </SvgIcon>
    }
}
