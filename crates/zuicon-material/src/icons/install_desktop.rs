// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(InstallDesktop)]
pub fn install_desktop(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("InstallDesktop"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,17H4V5h8V3H4C2.89,3,2,3.89,2,5v12c0,1.1,0.89,2,2,2h4v2h8v-2h4c1.1,0,2-0.9,2-2v-3h-2V17z"/>
        </SvgIcon>
    }
}
