// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ClearAllTwoTone)]
pub fn clear_all_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ClearAllTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M5 11h14v2H5zm-2 4h14v2H3zm4-8h14v2H7z"/>
        </SvgIcon>
    }
}
