// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SimCardAlertTwoTone)]
pub fn sim_card_alert_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SimCardAlertTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6,8.83V20h12V4h-7.17L6,8.83z M11,8h2v5h-2V8z M11,15h2v2h-2V15z" enable-background="new" opacity=".3"/><path d="M18,2h-8L4,8v12c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4C20,2.9,19.1,2,18,2z M18,20H6V8.83L10.83,4H18V20z"/>
        </SvgIcon>
    }
}
