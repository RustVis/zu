// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LocalPostOfficeSharp)]
pub fn local_post_office_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LocalPostOfficeSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M22 4H2.01v16H22V4zm-2 4l-8 5-8-5V6l8 5 8-5v2z"/>
        </SvgIcon>
    }
}
