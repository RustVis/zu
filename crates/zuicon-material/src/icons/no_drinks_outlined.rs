// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NoDrinksOutlined)]
pub fn no_drinks_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NoDrinksOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21.19,21.19L2.81,2.81L1.39,4.22l8.23,8.23L11,14v5H6v2h12v-0.17l1.78,1.78L21.19,21.19z M13,19v-3.17L16.17,19H13z M7.83,5l-2-2H21v2l-6.2,6.97l-1.42-1.42L14.77,9h-2.94l-2-2h6.74l1.78-2H7.83z"/>
        </SvgIcon>
    }
}
