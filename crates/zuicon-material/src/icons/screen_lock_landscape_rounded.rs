// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ScreenLockLandscapeRounded)]
pub fn screen_lock_landscape_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ScreenLockLandscapeRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M21,5H3C1.9,5,1.01,5.9,1.01,7L1,17c0,1.1,0.9,2,2,2h18c1.1,0,2-0.9,2-2V7C23,5.9,22.1,5,21,5z M18,17H6V7h12V17z M14,11 v-1c0-1.1-0.9-2-2-2s-2,0.9-2,2v1c-0.55,0-1,0.45-1,1v3c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-3C15,11.45,14.55,11,14,11z M13,11 h-2v-1c0-0.55,0.45-1,1-1s1,0.45,1,1V11z"/>
        </SvgIcon>
    }
}
