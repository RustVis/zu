// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DevicesFoldTwoTone)]
pub fn devices_fold_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DevicesFoldTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,3h-3c0-1.44-1.47-2.4-2.79-1.84l-3,1.29C10.48,2.76,10,3.49,10,4.29V19c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2V5 C22,3.9,21.1,3,20,3z M12,4.29L15,3v13.68l-3,1.29V4.29z M20,19h-5.33l1.12-0.48C16.52,18.2,17,17.48,17,16.68V5h3V19z"/>
        </SvgIcon>
    }
}
