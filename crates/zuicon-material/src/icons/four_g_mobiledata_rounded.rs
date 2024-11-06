// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FourGMobiledataRounded)]
pub fn four_g_mobiledata_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FourGMobiledataRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M8,7L8,7C7.45,7,7,7.45,7,8v4H5V8c0-0.55-0.45-1-1-1h0C3.45,7,3,7.45,3,8v5c0,0.55,0.45,1,1,1h3v2c0,0.55,0.45,1,1,1h0 c0.55,0,1-0.45,1-1v-2h1c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H9V8C9,7.45,8.55,7,8,7z M17,12L17,12c0,0.55,0.45,1,1,1h1v2h-5V9 h6c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-6c-1.1,0-2,0.9-2,2v6c0,1.1,0.9,2,2,2h5c1.1,0,2-0.9,2-2v-3c0-0.55-0.45-1-1-1h-2 C17.45,11,17,11.45,17,12z"/>
        </SvgIcon>
    }
}
