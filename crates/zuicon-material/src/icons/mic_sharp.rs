// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MicSharp)]
pub fn mic_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MicSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,14c1.66,0,3-1.34,3-3V5c0-1.66-1.34-3-3-3S9,3.34,9,5v6C9,12.66,10.34,14,12,14z"/><path d="M17,11c0,2.76-2.24,5-5,5s-5-2.24-5-5H5c0,3.53,2.61,6.43,6,6.92V21h2v-3.08c3.39-0.49,6-3.39,6-6.92H17z"/>
        </SvgIcon>
    }
}
