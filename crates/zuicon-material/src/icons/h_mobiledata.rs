// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HMobiledata)]
pub fn h_mobiledata(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HMobiledata"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M15,11H9V7H7v10h2v-4h6v4h2V7h-2V11z"/>
        </SvgIcon>
    }
}
