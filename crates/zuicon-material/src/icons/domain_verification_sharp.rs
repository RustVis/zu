// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DomainVerificationSharp)]
pub fn domain_verification_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DomainVerificationSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,4v16h18V4H3z M19,18H5V8h14V18z"/>
        </SvgIcon>
    }
}
