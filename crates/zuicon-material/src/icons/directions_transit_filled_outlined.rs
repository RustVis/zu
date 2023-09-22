// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DirectionsTransitFilledOutlined)]
pub fn directions_transit_filled_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DirectionsTransitFilledOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C8,2,4,2.5,4,6v9.5C4,17.43,5.57,19,7.5,19L6,20v1h12v-1l-1.5-1c1.93,0,3.5-1.57,3.5-3.5V6C20,2.5,16.42,2,12,2z M12,4c3.71,0,5.13,0.46,5.67,1H13h-2H6.43C7.03,4.48,8.48,4,12,4z M6,7h5v3H6V7z M18,15.5c0,0.83-0.67,1.5-1.5,1.5h-9 C6.67,17,6,16.33,6,15.5V12h12V15.5z M18,10h-5V7h5V10z"/>
        </SvgIcon>
    }
}
