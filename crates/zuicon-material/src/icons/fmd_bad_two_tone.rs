// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FmdBadTwoTone)]
pub fn fmd_bad_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FmdBadTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M11,15h2v-2h-2V15z M11,11h2V6h-2V11z M12,2c-4.2,0-8,3.22-8,8.2c0,3.32,2.67,7.25,8,11.8c5.33-4.55,8-8.48,8-11.8 C20,5.22,16.2,2,12,2z M12,19.33c-4.05-3.7-6-6.79-6-9.14C6,6.57,8.65,4,12,4s6,2.57,6,6.2C18,12.54,16.05,15.64,12,19.33z"/><path d="M12,19.33c4.05-3.7,6-6.79,6-9.14C18,6.57,15.35,4,12,4s-6,2.57-6,6.2C6,12.54,7.95,15.64,12,19.33z M11,6 h2v5h-2V6z M11,13h2v2h-2V13z" opacity=".3"/>
        </SvgIcon>
    }
}
