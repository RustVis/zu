// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LteMobiledataRounded)]
pub fn lte_mobiledata_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LteMobiledataRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M6,14h2c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1H5c-0.55,0-1-0.45-1-1V9c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1V14z M10,10 h1v5c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-5h1c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-4C9.45,8,9,8.45,9,9v0 C9,9.55,9.45,10,10,10z M21,9L21,9c0-0.55-0.45-1-1-1h-3c-0.55,0-1,0.45-1,1v6c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1v0 c0-0.55-0.45-1-1-1h-2v-1h2c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-2v-1h2C20.55,10,21,9.55,21,9z"/>
        </SvgIcon>
    }
}
