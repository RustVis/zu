// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AccountBox)]
pub fn account_box(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AccountBox"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M12,6c1.93,0,3.5,1.57,3.5,3.5 c0,1.93-1.57,3.5-3.5,3.5s-3.5-1.57-3.5-3.5C8.5,7.57,10.07,6,12,6z M19,19H5v-0.23c0-0.62,0.28-1.2,0.76-1.58 C7.47,15.82,9.64,15,12,15s4.53,0.82,6.24,2.19c0.48,0.38,0.76,0.97,0.76,1.58V19z"/>
        </SvgIcon>
    }
}
