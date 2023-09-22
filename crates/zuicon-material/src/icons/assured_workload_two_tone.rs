// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AssuredWorkloadTwoTone)]
pub fn assured_workload_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AssuredWorkloadTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,6L12,1L2,6v2h20V6z M6.47,6L12,3.24L17.53,6H6.47z"/><path d="M2,19v2h12.4c-0.21-0.64-0.32-1.31-0.36-2H2z"/><path d="M20,14l-4,2v2.55c0,2.52,1.71,4.88,4,5.45c2.29-0.57,4-2.93,4-5.45V16L20,14z M19.28,21l-2.03-2.03l1.06-1.06l0.97,0.97 l2.41-2.38l1.06,1.06L19.28,21z"/>
        </SvgIcon>
    }
}
