// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SettingsBluetoothRounded)]
pub fn settings_bluetooth_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SettingsBluetoothRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13.41,10L17,6.42c0.39-0.39,0.39-1.02,0-1.42l-4.79-4.79C12.07,0.07,11.89,0,11.71,0C11.32,0,11,0.32,11,0.71v6.88 L7.11,3.71c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41L10.59,10l-4.89,4.89c-0.39,0.39-0.39,1.02,0,1.41 c0.39,0.39,1.02,0.39,1.41,0L11,12.41v6.88c0,0.39,0.32,0.71,0.71,0.71c0.19,0,0.37-0.07,0.5-0.21L17,15 c0.39-0.39,0.39-1.02,0-1.42L13.41,10z M13,3.83l1.88,1.88L13,7.59V3.83z M13,16.17v-3.76l1.88,1.88L13,16.17z"/>
        </SvgIcon>
    }
}
