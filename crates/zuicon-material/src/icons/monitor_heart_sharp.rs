// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MonitorHeartSharp)]
pub fn monitor_heart_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MonitorHeartSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15.11,12.45L14,10.24l-3.11,6.21C10.73,16.79,10.38,17,10,17s-0.73-0.21-0.89-0.55L7.38,13H2v7h20v-7h-6 C15.62,13,15.27,12.79,15.11,12.45z"/><path d="M22,4H2v7h6c0.38,0,0.73,0.21,0.89,0.55L10,13.76l3.11-6.21c0.37-0.74,1.42-0.74,1.79,0L16.62,11H22V4z"/>
        </SvgIcon>
    }
}
