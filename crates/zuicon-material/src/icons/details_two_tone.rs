// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DetailsTwoTone)]
pub fn details_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DetailsTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13,8.92L18.6,19H13V8.92z M11,8.92V19H5.4L11,8.92z" opacity=".3"/><path d="M12,3L2,21h20L12,3z M13,8.92L18.6,19H13V8.92z M11,8.92V19H5.4L11,8.92z"/>
        </SvgIcon>
    }
}
