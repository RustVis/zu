// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PermMediaRounded)]
pub fn perm_media_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PermMediaRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,19H3V7c0-0.55-0.45-1-1-1S1,6.45,1,7v12c0,1.1,0.9,2,2,2h16c0.55,0,1-0.45,1-1S19.55,19,19,19z"/><path d="M21,4h-7l-1.41-1.41C12.21,2.21,11.7,2,11.17,2H7C5.9,2,5.01,2.9,5.01,4L5,15c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V6 C23,4.9,22.1,4,21,4z M18,13h-8c-0.41,0-0.65-0.47-0.4-0.8l1.38-1.83c0.2-0.27,0.6-0.27,0.8,0L13,12l2.22-2.97 c0.2-0.27,0.6-0.27,0.8,0l2.38,3.17C18.65,12.53,18.41,13,18,13z"/>
        </SvgIcon>
    }
}
