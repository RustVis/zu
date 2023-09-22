// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Looks4Sharp)]
pub fn looks_4_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Looks4Sharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M.04 0h24v24h-24V0z" fill="none"/><path d="M21.04 3h-18v18h18V3zm-6 14h-2v-4h-4V7h2v4h2V7h2v10z"/>
        </SvgIcon>
    }
}
