// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FourGPlusMobiledataSharp)]
pub fn four_g_plus_mobiledata_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FourGPlusMobiledataSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M13,11v2h2v2h-4V9h6V7H9v10h8v-6H13z M24,11h-2V9h-2v2h-2v2h2v2h2v-2h2V11z M7,7H5v5H3V7H1v7h4v3h2v-3h1v-2H7V7z"/>
        </SvgIcon>
    }
}
