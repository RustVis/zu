// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ElevenMpSharp)]
pub fn eleven_mp_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ElevenMpSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,3v18h18V3H3z M8,5.5h3v6H9.5V7H8V5.5z M12.5,18.5H11V14h-1v3H8.5v-3h-1v4.5H6v-6h6.5V18.5z M13,5.5h3v6h-1.5V7H13V5.5z M18,17h-3v1.5h-1.5v-6H18V17z"/>
        </SvgIcon>
    }
}
