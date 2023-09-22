// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PhoneEnabledSharp)]
pub fn phone_enabled_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PhoneEnabledSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,15.46l5.27-0.61l2.52,2.52c2.83-1.44,5.15-3.75,6.59-6.59l-2.53-2.53L15.46,3h5.51 C21.55,13.18,13.18,21.55,3,20.97V15.46z"/>
        </SvgIcon>
    }
}
