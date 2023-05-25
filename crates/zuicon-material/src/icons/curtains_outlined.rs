// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CurtainsOutlined)]
pub fn curtains_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CurtainsOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,19V3H4v16H2v2h20v-2H20z M18,10.86c-2.05-0.58-3.64-2.93-3.94-5.86H18V10.86z M15.81,12c-2.04,1.35-3.5,3.94-3.76,7 h-0.09c-0.26-3.06-1.72-5.65-3.76-7c2.04-1.35,3.5-3.94,3.76-7h0.09C12.31,8.06,13.77,10.65,15.81,12z M9.94,5 C9.64,7.93,8.05,10.27,6,10.86V5H9.94z M6,13.14c2.05,0.58,3.64,2.93,3.94,5.86H6V13.14z M14.06,19c0.3-2.93,1.89-5.27,3.94-5.86 V19H14.06z"/>
        </SvgIcon>
    }
}
