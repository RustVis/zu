// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FormatColorFillSharp)]
pub fn format_color_fill_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FormatColorFillSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10,17.62L17.62,10l-10-10L6.21,1.41l2.38,2.38L2.38,10L10,17.62z M10,5.21L14.79,10H5.21 L10,5.21z" enable-background="new"/><path d="M19,17c1.1,0,2-0.9,2-2c0-1.33-2-3.5-2-3.5s-2,2.17-2,3.5C17,16.1,17.9,17,19,17z" enable-background="new"/>
        </SvgIcon>
    }
}
