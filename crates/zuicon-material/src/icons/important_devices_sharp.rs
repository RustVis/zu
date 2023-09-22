// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ImportantDevicesSharp)]
pub fn important_devices_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ImportantDevicesSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M24 11.01L17 11v11h7V11.01zM23 20h-5v-7h5v7zM22 2H0v16h9v2H7v2h8v-2h-2v-2h2v-2H2V4h18v5h2V2zM11.97 9L11 6l-.97 3H7l2.47 1.76-.94 2.91 2.47-1.8 2.47 1.8-.94-2.91L15 9h-3.03z"/>
        </SvgIcon>
    }
}
