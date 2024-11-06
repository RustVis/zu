// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BlenderRounded)]
pub fn blender_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BlenderRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M16.13,15.13l1.69-10.98C17.92,3.55,17.45,3,16.83,3H14v0c0-0.55-0.45-1-1-1h-2c-0.55,0-1,0.45-1,1v0H5C3.9,3,3,3.9,3,5v4 c0,1.1,0.9,2,2,2h2.23l0.64,4.13C6.74,16.05,6,17.43,6,19v1c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2v-1 C18,17.43,17.26,16.05,16.13,15.13z M5,9V5h1.31l0.62,4H5z M12,19c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S12.55,19,12,19z M14.29,14H9.72L8.33,5h7.34L14.29,14z"/>
        </SvgIcon>
    }
}
