// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(InstallMobile)]
pub fn install_mobile(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("InstallMobile"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,18H7V6h7V1H7C5.9,1,5,1.9,5,3v18c0,1.1,0.9,2,2,2h10c1.1,0,2-0.9,2-2v-5h-2V18z"/>
        </SvgIcon>
    }
}
