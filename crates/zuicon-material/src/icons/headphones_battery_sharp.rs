// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HeadphonesBatterySharp)]
pub fn headphones_battery_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HeadphonesBatterySharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8,6c-3.31,0-6,2.69-6,6v6h4v-5H3.5v-1c0-2.48,2.02-4.5,4.5-4.5s4.5,2.02,4.5,4.5v1H10v5h4v-6C14,8.69,11.31,6,8,6z"/>
        </SvgIcon>
    }
}
