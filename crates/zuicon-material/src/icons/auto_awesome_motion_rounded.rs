// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AutoAwesomeMotionRounded)]
pub fn auto_awesome_motion_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AutoAwesomeMotionRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13,2H4C2.9,2,2,2.9,2,4v9c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V4h9c0.55,0,1-0.45,1-1v0C14,2.45,13.55,2,13,2z M17,6H8 C6.9,6,6,6.9,6,8v9c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V8h9c0.55,0,1-0.45,1-1v0C18,6.45,17.55,6,17,6z M20,10h-8 c-1.1,0-2,0.9-2,2v8c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2v-8C22,10.9,21.1,10,20,10z"/>
        </SvgIcon>
    }
}
