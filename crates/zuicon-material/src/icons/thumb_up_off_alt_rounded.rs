// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ThumbUpOffAltRounded)]
pub fn thumb_up_off_alt_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ThumbUpOffAltRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13.12,2.06L7.58,7.6C7.21,7.97,7,8.48,7,9.01V19c0,1.1,0.9,2,2,2h9c0.8,0,1.52-0.48,1.84-1.21l3.26-7.61 C23.94,10.2,22.49,8,20.34,8h-5.65l0.95-4.58c0.1-0.5-0.05-1.01-0.41-1.37C14.64,1.47,13.7,1.47,13.12,2.06z M3,21 c1.1,0,2-0.9,2-2v-8c0-1.1-0.9-2-2-2s-2,0.9-2,2v8C1,20.1,1.9,21,3,21z"/>
        </SvgIcon>
    }
}
