// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DomainDisabledRounded)]
pub fn domain_disabled_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DomainDisabledRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M.71 2.39c-.39.39-.39 1.02 0 1.41L2 5.1V19c0 1.1.9 2 2 2h13.9l2.29 2.29c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L2.12 2.39C1.73 2 1.1 2 .71 2.39zM6 19H4v-2h2v2zm0-4H4v-2h2v2zm-2-4V9h2v2H4zm6 8H8v-2h2v2zm-2-4v-2h2v2H8zm4 4v-2h1.9l2 2H12zM8 5h2v2h-.45L12 9.45V9h7c.55 0 1 .45 1 1v7.45l2 2V9c0-1.1-.9-2-2-2h-8V5c0-1.1-.9-2-2-2H5.55L8 5.45V5zm8 6h2v2h-2z"/>
        </SvgIcon>
    }
}
