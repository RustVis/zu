// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Battery3BarSharp)]
pub fn battery_3_bar_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Battery3BarSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,4v18H7V4h3V2h4v2H17z M15,6H9v8h6V6z"/>
        </SvgIcon>
    }
}
