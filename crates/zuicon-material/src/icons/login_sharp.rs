// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LoginSharp)]
pub fn login_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LoginSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M11,7L9.6,8.4l2.6,2.6H2v2h10.2l-2.6,2.6L11,17l5-5L11,7z M20,19h-8v2h10V3H12v2h8V19z"/>
        </SvgIcon>
    }
}
