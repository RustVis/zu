// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SdCardAlert)]
pub fn sd_card_alert(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SdCardAlert"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M18,2h-8L4.02,8L4,20c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4C20,2.9,19.1,2,18,2z M13,17h-2v-2h2V17z M13,13h-2V8h2V13z"/>
        </SvgIcon>
    }
}
