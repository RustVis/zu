// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WifiCallingSharp)]
pub fn wifi_calling_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WifiCallingSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13.21,17.37c-2.83-1.44-5.15-3.75-6.59-6.59l2.53-2.53L8.54,3H3.03C2.45,13.18,10.82,21.55,21,20.97v-5.51l-5.27-0.61 L13.21,17.37z"/><path d="M22,4.95C21.79,4.78,19.67,3,16.5,3c-3.18,0-5.29,1.78-5.5,1.95L16.5,12L22,4.95z"/>
        </SvgIcon>
    }
}
