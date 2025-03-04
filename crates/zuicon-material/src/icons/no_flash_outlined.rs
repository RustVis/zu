// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NoFlashOutlined)]
pub fn no_flash_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NoFlashOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.4,5.6H22L19,11V7h-1V2h4L20.4,5.6z M16,11.4l0,1.77l2,2V11c0-0.88-0.72-1.6-1.6-1.6h-2.54L12.58,8h-1.75l3.4,3.4H16z M2.1,2.1L0.69,3.51l5.66,5.66L6.14,9.4H3.6C2.72,9.4,2,10.12,2,11v9.4C2,21.28,2.72,22,3.6,22h12.8c0.75,0,1.38-0.52,1.55-1.22 l2.54,2.54l1.41-1.41L2.1,2.1z M11.5,15.5c0,0.83-0.67,1.5-1.5,1.5s-1.5-0.67-1.5-1.5S9.17,14,10,14S11.5,14.67,11.5,15.5z M15.96,20H4v-8.6h2.14h0.88l0.59-0.65l0.15-0.16l1.5,1.5C7.68,12.43,6.5,13.82,6.5,15.5c0,1.93,1.57,3.5,3.5,3.5 c1.68,0,3.07-1.18,3.42-2.76l2.55,2.55L15.96,20z"/><path d="M0,0h24v24H0V0z" fill="none"/>
        </SvgIcon>
    }
}
