// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MedicalServicesTwoTone)]
pub fn medical_services_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MedicalServicesTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,20h16V8H4V20z M8,13h3v-3h2v3h3v2h-3v3h-2v-3H8V13z"/><path d="M20,6h-4V4c0-1.1-0.9-2-2-2h-4C8.9,2,8,2.9,8,4v2H4C2.9,6,2,6.9,2,8v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V8 C22,6.9,21.1,6,20,6z M10,4h4v2h-4V4z M20,20H4V8h16V20z"/>
        </SvgIcon>
    }
}
