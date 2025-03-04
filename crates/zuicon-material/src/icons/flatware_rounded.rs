// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FlatwareRounded)]
pub fn flatware_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FlatwareRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M16,7.08c0,1.77-0.84,3.25-2,3.82V20c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1v-9.1c-1.16-0.57-2-2.05-2-3.82 C10.01,4.83,11.35,3,13,3C14.66,3,16,4.83,16,7.08z M18.27,3.18C17.64,2.99,17,3.49,17,4.15L17,20c0,0.55,0.45,1,1,1h0 c0.55,0,1-0.45,1-1v-7h1c0.55,0,1-0.45,1-1V7C21,5.54,20.14,3.74,18.27,3.18z M8.28,3c-0.4,0-0.72,0.32-0.72,0.72V7H6.72V3.72 C6.72,3.32,6.4,3,6,3S5.28,3.32,5.28,3.72V7H4.44V3.72C4.44,3.32,4.12,3,3.72,3S3,3.32,3,3.72V9c0,1.1,0.9,2,2,2v9 c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-9c1.1,0,2-0.9,2-2V3.72C9,3.32,8.68,3,8.28,3z"/>
        </SvgIcon>
    }
}
