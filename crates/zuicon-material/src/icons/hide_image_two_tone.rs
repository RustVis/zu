// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HideImageTwoTone)]
pub fn hide_image_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HideImageTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,5v11.17l2,2V5c0-1.1-0.9-2-2-2H5.83l2,2H19z"/><path d="M2.81,2.81L1.39,4.22L3,5.83V19c0,1.1,0.9,2,2,2h13.17l1.61,1.61l1.41-1.41L2.81,2.81z M5,19V7.83l7.07,7.07L11.25,16 L9,13l-3,4h8.17l2,2H5z"/>
        </SvgIcon>
    }
}
