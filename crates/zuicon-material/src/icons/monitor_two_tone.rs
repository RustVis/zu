// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MonitorTwoTone)]
pub fn monitor_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MonitorTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,3H4C2.9,3,2,3.9,2,5v11c0,1.1,0.9,2,2,2h3l-1,1v2h12v-2l-1-1h3c1.1,0,2-0.9,2-2V5C22,3.9,21.1,3,20,3z M20,16H4V5h16 V16z"/>
        </SvgIcon>
    }
}
