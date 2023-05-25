// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EMobiledata)]
pub fn e_mobiledata(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EMobiledata"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M16,9V7H8v10h8v-2h-6v-2h6v-2h-6V9H16z"/>
        </SvgIcon>
    }
}
