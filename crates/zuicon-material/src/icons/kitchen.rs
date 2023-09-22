// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Kitchen)]
pub fn kitchen(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Kitchen"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20 9V4c0-1.1-.9-2-2-2H6c-1.1 0-2 .9-2 2v5h16zM8 5h2v3H8V5zm-4 6v9c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2v-9H4zm6 6H8v-5h2v5z"/>
        </SvgIcon>
    }
}
