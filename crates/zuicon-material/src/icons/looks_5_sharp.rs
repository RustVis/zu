// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Looks5Sharp)]
pub fn looks_5_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Looks5Sharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M21 3H3v18h18V3zm-6 6h-4v2h4v6H9v-2h4v-2H9V7h6v2z"/>
        </SvgIcon>
    }
}
