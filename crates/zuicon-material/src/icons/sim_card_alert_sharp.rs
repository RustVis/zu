// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SimCardAlertSharp)]
pub fn sim_card_alert_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SimCardAlertSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,2H10L4,8v14h16V2z M13,17h-2v-2h2V17z M13,13h-2V8h2V13z"/>
        </SvgIcon>
    }
}
