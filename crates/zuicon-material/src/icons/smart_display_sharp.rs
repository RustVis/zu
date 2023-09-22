// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SmartDisplaySharp)]
pub fn smart_display_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SmartDisplaySharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,4H2v16h20V4z M9.5,16.5v-9l7,4.5L9.5,16.5z"/>
        </SvgIcon>
    }
}
