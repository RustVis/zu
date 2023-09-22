// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EMobiledataRounded)]
pub fn e_mobiledata_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EMobiledataRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M16,8L16,8c0-0.55-0.45-1-1-1H9C8.45,7,8,7.45,8,8v8c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-5v-2h5 c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-5V9h5C15.55,9,16,8.55,16,8z"/>
        </SvgIcon>
    }
}
