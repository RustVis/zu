// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HouseRounded)]
pub fn house_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HouseRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,9.3V5c0-0.55-0.45-1-1-1h-1c-0.55,0-1,0.45-1,1v1.6l-3.33-3c-0.38-0.34-0.96-0.34-1.34,0l-8.36,7.53 C2.63,11.43,2.84,12,3.3,12H5v7c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1v-5h4v5c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1v-7h1.7 c0.46,0,0.68-0.57,0.33-0.87L19,9.3z M10,10c0-1.1,0.9-2,2-2s2,0.9,2,2H10z"/>
        </SvgIcon>
    }
}
