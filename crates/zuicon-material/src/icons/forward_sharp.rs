// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ForwardSharp)]
pub fn forward_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ForwardSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M12 8V4l8 8-8 8v-4H4V8h8z"/>
        </SvgIcon>
    }
}
