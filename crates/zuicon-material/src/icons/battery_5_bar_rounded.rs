// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Battery5BarRounded)]
pub fn battery_5_bar_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Battery5BarRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,5v16c0,0.55-0.45,1-1,1H8c-0.55,0-1-0.45-1-1V5c0-0.55,0.45-1,1-1h2V3c0-0.55,0.45-1,1-1h2c0.55,0,1,0.45,1,1v1h2 C16.55,4,17,4.45,17,5z M15,6H9v4h6V6z"/>
        </SvgIcon>
    }
}
