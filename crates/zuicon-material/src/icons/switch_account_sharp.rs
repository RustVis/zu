// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SwitchAccountSharp)]
pub fn switch_account_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SwitchAccountSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,6H2v16h16v-2H4V6z M6,2v16h16V2H6z M14,5c1.66,0,3,1.34,3,3c0,1.66-1.34,3-3,3s-3-1.34-3-3C11,6.34,12.34,5,14,5z M7.76,16c1.47-1.83,3.71-3,6.24-3s4.77,1.17,6.24,3H7.76z"/>
        </SvgIcon>
    }
}
