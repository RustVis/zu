// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DevicesFoldSharp)]
pub fn devices_fold_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DevicesFoldSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,3v-3.03l-7,3V3v18h12V3H17z M20,19h-5.33L17,18V5h3V19z"/>
        </SvgIcon>
    }
}
