// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(GMobiledataRounded)]
pub fn g_mobiledata_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("GMobiledataRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M12,12L12,12c0,0.55,0.45,1,1,1h1v2H9V9h6c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H9C7.9,7,7,7.9,7,9v6c0,1.1,0.9,2,2,2h5 c1.1,0,2-0.9,2-2v-3c0-0.55-0.45-1-1-1h-2C12.45,11,12,11.45,12,12z"/>
        </SvgIcon>
    }
}
