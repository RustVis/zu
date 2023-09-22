// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DesignServicesTwoTone)]
pub fn design_services_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DesignServicesTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.97,5.86l-2.83-2.83c-0.39-0.39-1.02-0.39-1.41,0l-4.49,4.49L8.35,3.63c-0.78-0.78-2.05-0.78-2.83,0l-1.9,1.9 c-0.78,0.78-0.78,2.05,0,2.83l3.89,3.89L3,16.76V21h4.24l4.52-4.52l3.89,3.89c0.95,0.95,2.23,0.6,2.83,0l1.9-1.9 c0.78-0.78,0.78-2.05,0-2.83l-3.89-3.89l4.49-4.49C21.36,6.88,21.36,6.25,20.97,5.86z M5.04,6.94l1.89-1.9c0,0,0,0,0,0l1.27,1.27 L7.02,7.5l1.41,1.41l1.19-1.19l1.2,1.2l-1.9,1.9L5.04,6.94z M6.41,19H5v-1.41l9.61-9.61l1.3,1.3l0.11,0.11L6.41,19z M16.5,16.98 l1.19-1.19l1.27,1.27l-1.9,1.9l-3.89-3.89l1.9-1.9l1.2,1.2l-1.19,1.19L16.5,16.98z M17.44,7.98l-1.41-1.41l1.41-1.41l1.41,1.41 L17.44,7.98z"/>
        </SvgIcon>
    }
}
