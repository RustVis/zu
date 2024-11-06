// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SecurityUpdateWarning)]
pub fn security_update_warning(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SecurityUpdateWarning"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M17,1H7C5.9,1,5.01,1.9,5.01,3v18c0,1.1,0.89,2,1.99,2h10c1.1,0,2-0.9,2-2V3C19,1.9,18.1,1,17,1z M17,18H7V6h10V18z"/>
        </SvgIcon>
    }
}
