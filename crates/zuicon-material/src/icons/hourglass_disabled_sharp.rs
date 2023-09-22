// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HourglassDisabledSharp)]
pub fn hourglass_disabled_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HourglassDisabledSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2.1,2.1L0.69,3.51l8.9,8.9L6,16l0.01,0.01H6V22h12v-1.17l2.49,2.49l1.41-1.41L2.1,2.1z M16,20H8v-3.5l2.84-2.84L16,18.83 V20z"/>
        </SvgIcon>
    }
}
