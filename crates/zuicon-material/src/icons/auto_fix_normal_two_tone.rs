// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AutoFixNormalTwoTone)]
pub fn auto_fix_normal_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AutoFixNormalTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17.71,9.12l-2.83-2.83C14.68,6.1,14.43,6,14.17,6c-0.26,0-0.51,0.1-0.71,0.29L2.29,17.46c-0.39,0.39-0.39,1.02,0,1.41 l2.83,2.83C5.32,21.9,5.57,22,5.83,22s0.51-0.1,0.71-0.29l11.17-11.17C18.1,10.15,18.1,9.51,17.71,9.12z M5.83,19.59l-1.41-1.41 L11.59,11L13,12.41L5.83,19.59z M14.41,11L13,9.59l1.17-1.17l1.41,1.41L14.41,11z"/>
        </SvgIcon>
    }
}
