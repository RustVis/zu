// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MediaBluetoothOffRounded)]
pub fn media_bluetooth_off_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MediaBluetoothOffRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M9,6.17V5c0-1.1,0.9-2,2-2h2c1.1,0,2,0.9,2,2v0c0,1.1-0.9,2-2,2h-2v1.17L9,6.17z M19.42,15l2.18,2.17 c0.22,0.22,0.22,0.58,0,0.8l0,0c-0.22,0.22-0.58,0.22-0.8,0l-5.98-5.98c-0.22-0.22-0.22-0.58,0-0.8l0,0c0.22-0.22,0.58-0.22,0.8,0 l2.35,2.35V9.61c0-0.45,0.54-0.67,0.85-0.35l2.82,2.82c0.2,0.2,0.2,0.51,0,0.71L19.42,15z M19.17,13.55l1.13-1.13l-1.13-1.13 V13.55z M20.49,20.49c0.39,0.39,0.39,1.02,0,1.41l0,0c-0.39,0.39-1.02,0.39-1.41,0l-3.28-3.28l-0.16,0.16 c-0.23,0.23-0.62,0.23-0.85,0l0,0c-0.23-0.23-0.23-0.62,0-0.85l0.16-0.16L11,13.83l0,3.02c0,2.07-1.68,4.01-3.74,4.14 C4.94,21.13,3,19.29,3,17c0-2.21,1.79-4,4.01-4c0.73,0,1.41,0.21,2,0.55v-1.72L2.1,4.92c-0.39-0.39-0.39-1.02,0-1.41l0,0 c0.39-0.39,1.02-0.39,1.41,0L20.49,20.49z"/>
        </SvgIcon>
    }
}
