// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DirectionsBusFilledSharp)]
pub fn directions_bus_filled_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DirectionsBusFilledSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C8,2,4,2.5,4,6v9.5c0,0.95,0.38,1.81,1,2.44V21h3v-2h8v2h3v-3.06 c0.62-0.63,1-1.49,1-2.44V6C20,2.5,16.42,2,12,2z M8.5,16C7.67,16,7,15.33,7,14.5S7.67,13,8.5,13s1.5,0.67,1.5,1.5S9.33,16,8.5,16z M15.5,16c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S16.33,16,15.5,16z M18,10H6V7h12V10z" enable-background="new"/>
        </SvgIcon>
    }
}
