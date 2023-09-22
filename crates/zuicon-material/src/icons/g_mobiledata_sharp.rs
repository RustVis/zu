// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(GMobiledataSharp)]
pub fn g_mobiledata_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("GMobiledataSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M12,11v2h2v2H9V9h7V7H7v10h9v-6H12z"/>
        </SvgIcon>
    }
}
