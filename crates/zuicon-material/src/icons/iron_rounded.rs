// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(IronRounded)]
pub fn iron_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("IronRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8.27,10c0.34,0,0.68-0.16,0.84-0.47C9.28,9.22,9.62,9,10,9h4c0.55,0,1,0.45,1,1v1H6c-2.21,0-4,1.79-4,4v2 c0,0.55,0.45,1,1,1h13c0.55,0,1-0.45,1-1v-1c1.66,0,3-1.34,3-3V9c0-0.55,0.45-1,1-1h0c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h0 c-1.66,0-3,1.34-3,3v4c0,0.55-0.45,1-1,1v-4c0-1.66-1.34-3-3-3h-4C8.87,7,7.89,7.62,7.37,8.55C7.01,9.2,7.53,10,8.27,10z"/>
        </SvgIcon>
    }
}
