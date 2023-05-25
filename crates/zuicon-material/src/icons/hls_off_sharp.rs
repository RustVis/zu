// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HlsOffSharp)]
pub fn hls_off_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HlsOffSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17.83,15h2.67v-3.5H17v-1h2V11h1.5V9h-5v3.5H19v1h-2V13h-1.17L17.83,15z M8,10.83V15H6.5v-2.5h-2V15H3V9h1.5v2h2V9.33 L1.39,4.22l1.41-1.41l18.38,18.38l-1.41,1.41L12.17,15H10v-2.17L8,10.83z"/>
        </SvgIcon>
    }
}
