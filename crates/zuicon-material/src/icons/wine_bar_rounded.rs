// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WineBarRounded)]
pub fn wine_bar_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WineBarRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M7,3C6.45,3,6,3.45,6,4l0,5c0,2.97,2.16,5.43,5,5.91V19H9c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1v0 c0-0.55-0.45-1-1-1h-2v-4.09c2.84-0.48,5-2.94,5-5.91l0-5c0-0.55-0.45-1-1-1H7z M16,8H8l0-3h8C16,5,16,8,16,8z"/>
        </SvgIcon>
    }
}
