// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PersonalVideoSharp)]
pub fn personal_video_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PersonalVideoSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M23 3H1v16h7v2h8v-2h6.99L23 3zm-2 14H3V5h18v12z"/>
        </SvgIcon>
    }
}
