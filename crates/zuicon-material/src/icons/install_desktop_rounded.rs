// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(InstallDesktopRounded)]
pub fn install_desktop_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("InstallDesktopRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,17H4V5h8V3H4C2.9,3,2,3.9,2,5v12c0,1.1,0.89,2,2,2h4v1c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1v-1h4c1.1,0,2-0.9,2-2 v-3h-2V17z"/><path d="M17.71,13.29l3.59-3.59c0.39-0.39,0.39-1.02,0-1.41l0,0c-0.39-0.39-1.02-0.39-1.41,0L18,10.17V4c0-0.55-0.45-1-1-1h0 c-0.55,0-1,0.45-1,1v6.17l-1.89-1.88c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41l3.59,3.59 C16.69,13.68,17.32,13.68,17.71,13.29z"/>
        </SvgIcon>
    }
}
