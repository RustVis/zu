// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CreditScoreSharp)]
pub fn credit_score_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CreditScoreSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M2,4v16h7v-2H4v-6h18V4H2z M20,8H4V6h16V8z M14.93,19.17l-2.83-2.83l-1.41,1.41L14.93,22L22,14.93l-1.41-1.41L14.93,19.17z"/>
        </SvgIcon>
    }
}
