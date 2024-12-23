// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ScreenLockPortraitRounded)]
pub fn screen_lock_portrait_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ScreenLockPortraitRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M17,1H7C5.9,1,5,1.9,5,3v18c0,1.1,0.9,1.99,2,1.99L17,23c1.1,0,2-0.9,2-2V3C19,1.9,18.1,1,17,1z M17,18H7V6h10V18z"/><path d="M14,11v-1c0-1.1-0.9-2-2-2s-2,0.9-2,2v1c-0.55,0-1,0.45-1,1v3c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-3 C15,11.45,14.55,11,14,11z M13,11h-2v-1c0-0.55,0.45-1,1-1s1,0.45,1,1V11z"/>
        </SvgIcon>
    }
}
