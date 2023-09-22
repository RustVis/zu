// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Woman2Rounded)]
pub fn woman_2_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Woman2Rounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13.94,8.31c-0.38-0.96-1.42-1.52-2.48-1.24c-0.66,0.17-1.18,0.7-1.43,1.34l-2.48,6.22C7.28,15.29,7.77,16,8.47,16h2.03 v5c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-5h2.03c0.71,0,1.19-0.71,0.93-1.37L13.94,8.31z"/>
        </SvgIcon>
    }
}
