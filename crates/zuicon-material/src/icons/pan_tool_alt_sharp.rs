// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PanToolAltSharp)]
pub fn pan_tool_alt_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PanToolAltSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.18,13.4L19.1,21h-9L5,15.62l1.22-1.23L10,15.24V4.5C10,3.67,10.67,3,11.5,3S13,3.67,13,4.5v6h1.38L20.18,13.4z"/>
        </SvgIcon>
    }
}
