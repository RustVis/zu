// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NetworkLockedRounded)]
pub fn network_locked_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NetworkLockedRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,12l0-7.58c0-0.89-1.08-1.34-1.71-0.71L3.71,20.29C3.08,20.92,3.52,22,4.41,22H15v-6c0-2.21,1.79-4,4-4H22z M22,17v-1 c0-1.1-0.9-2-2-2c-1.1,0-2,0.9-2,2v1c-0.55,0-1,0.45-1,1v3c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-3C23,17.45,22.55,17,22,17z M19,16c0-0.55,0.45-1,1-1s1,0.45,1,1v1h-2V16z"/>
        </SvgIcon>
    }
}
