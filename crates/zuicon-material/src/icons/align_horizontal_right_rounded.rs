// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AlignHorizontalRightRounded)]
pub fn align_horizontal_right_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AlignHorizontalRightRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,2L21,2c0.55,0,1,0.45,1,1v18c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1V3C20,2.45,20.45,2,21,2z M3.5,10h13 c0.83,0,1.5-0.67,1.5-1.5v0C18,7.67,17.33,7,16.5,7h-13C2.67,7,2,7.67,2,8.5v0C2,9.33,2.67,10,3.5,10z M9.5,17h7 c0.83,0,1.5-0.67,1.5-1.5v0c0-0.83-0.67-1.5-1.5-1.5h-7C8.67,14,8,14.67,8,15.5v0C8,16.33,8.67,17,9.5,17z"/>
        </SvgIcon>
    }
}
