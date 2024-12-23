// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DomainAddRounded)]
pub fn domain_add_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DomainAddRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6,19H4v-2h2V19z M6,15H4v-2h2V15z M6,11H4V9h2V11z M6,7H4V5h2V7z M10,19H8v-2h2V19z M10,15H8v-2h2V15z M10,11H8V9h2V11z M10,7H8V5h2V7z M16,19h-4v-2h2v-2h-2v-2h2v-2h-2V9h8v6h2V8c0-0.55-0.45-1-1-1h-9V4c0-0.55-0.45-1-1-1H3C2.45,3,2,3.45,2,4v16 c0,0.55,0.45,1,1,1h13V19z M18,11h-2v2h2V11z M18,15h-2v2h2V15z M24,20c0,0.55-0.45,1-1,1h-1v1c0,0.55-0.45,1-1,1s-1-0.45-1-1v-1 h-1c-0.55,0-1-0.45-1-1s0.45-1,1-1h1v-1c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1v1h1C23.55,19,24,19.45,24,20z"/>
        </SvgIcon>
    }
}
