// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FlipSharp)]
pub fn flip_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FlipSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M15 21h2v-2h-2v2zm4-12h2V7h-2v2zM3 3v18h6v-2H5V5h4V3H3zm16 0v2h2V3h-2zm-8 20h2V1h-2v22zm8-6h2v-2h-2v2zM15 5h2V3h-2v2zm4 8h2v-2h-2v2zm0 8h2v-2h-2v2z"/>
        </SvgIcon>
    }
}
