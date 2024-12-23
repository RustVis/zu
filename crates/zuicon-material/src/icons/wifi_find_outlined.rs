// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WifiFindOutlined)]
pub fn wifi_find_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WifiFindOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,6c4.14,0,7.88,1.68,10.59,4.39L24,8.98C20.93,5.9,16.69,4,12,4C7.31,4,3.07,5.9,0,8.98L12,21l1.41-1.42L2.93,9.08 C5.45,7.16,8.59,6,12,6z"/><path d="M21,14c0-2.24-1.76-4-4-4s-4,1.76-4,4c0,2.24,1.76,4,4,4c0.75,0,1.44-0.21,2.03-0.56L21.59,20L23,18.59l-2.56-2.56 C20.79,15.44,21,14.75,21,14z M17,16c-1.12,0-2-0.88-2-2c0-1.12,0.88-2,2-2s2,0.88,2,2C19,15.12,18.12,16,17,16z"/>
        </SvgIcon>
    }
}
