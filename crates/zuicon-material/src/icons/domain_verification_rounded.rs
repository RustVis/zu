// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DomainVerificationRounded)]
pub fn domain_verification_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DomainVerificationRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10.23,15.83c0.39,0.39,1.02,0.39,1.41,0l4.24-4.24c0.39-0.39,0.39-1.02,0-1.42v0c-0.39-0.39-1.02-0.39-1.41,0l-3.54,3.53 l-1.41-1.41c-0.39-0.39-1.02-0.39-1.42,0s-0.39,1.02,0,1.41L10.23,15.83z"/><path d="M19,4H5C3.89,4,3,4.9,3,6v12c0,1.1,0.89,2,2,2h14c1.1,0,2-0.9,2-2V6C21,4.9,20.11,4,19,4z M19,17c0,0.55-0.45,1-1,1H6 c-0.55,0-1-0.45-1-1V8h14V17z"/>
        </SvgIcon>
    }
}
