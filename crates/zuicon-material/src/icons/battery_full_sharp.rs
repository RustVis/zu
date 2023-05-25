// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BatteryFullSharp)]
pub fn battery_full_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BatteryFullSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M17 4h-3V2h-4v2H7v18h10V4z"/>
        </SvgIcon>
    }
}
