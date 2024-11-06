// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ResetTvTwoTone)]
pub fn reset_tv_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ResetTvTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M20,10h-7.01V7L9,11l3.99,4v-3H20v5H4V5h16v3h2l0-3c0-1.1-0.9-2-2-2H4C2.9,3,2,3.9,2,5v12c0,1.1,0.9,2,2,2h4v2h8v-2h4 c1.1,0,1.99-0.9,1.99-2l0-5H22C22,10.9,21.1,10,20,10z"/>
        </SvgIcon>
    }
}
