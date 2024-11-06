// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PhoneAndroidSharp)]
pub fn phone_android_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PhoneAndroidSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M19 1H5v22h14V1zm-5 20h-4v-1h4v1zm3-3H7V4h10v14z"/>
        </SvgIcon>
    }
}
