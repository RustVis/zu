// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Man4Outlined)]
pub fn man_4_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Man4Outlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13.75,7h-3.5C9.04,7,8.11,8.07,8.27,9.26L10,22h4l1.73-12.74C15.89,8.07,14.96,7,13.75,7z"/>
        </SvgIcon>
    }
}
