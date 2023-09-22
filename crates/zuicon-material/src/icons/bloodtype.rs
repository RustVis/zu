// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Bloodtype)]
pub fn bloodtype(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Bloodtype"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M12,2c-5.33,4.55-8,8.48-8,11.8c0,4.98,3.8,8.2,8,8.2s8-3.22,8-8.2C20,10.48,17.33,6.55,12,2z M15,18H9v-2h6V18z M15,13h-2 v2h-2v-2H9v-2h2V9h2v2h2V13z"/>
        </SvgIcon>
    }
}
