// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SailingOutlined)]
pub fn sailing_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SailingOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M11,13.5V2L3,13.5H11z M9,11.5H6.83L9,8.38V11.5z M21,13.5C21,6.5,14.5,1,12.5,1c0,0,1,3,1,6.5s-1,6-1,6H21z M15.38,5.24 c1.42,1.52,2.88,3.72,3.41,6.26h-3.68c0.21-1.1,0.39-2.46,0.39-4C15.5,6.71,15.45,5.95,15.38,5.24z M22,15H2 c0.31,1.53,1.16,2.84,2.33,3.73C4.98,18.46,5.55,18.01,6,17.5C6.73,18.34,7.8,19,9,19s2.27-0.66,3-1.5c0.73,0.84,1.8,1.5,3,1.5 s2.26-0.66,3-1.5c0.45,0.51,1.02,0.96,1.67,1.23C20.84,17.84,21.69,16.53,22,15z M22,23v-2h-1c-1.04,0-2.08-0.35-3-1 c-1.83,1.3-4.17,1.3-6,0c-1.83,1.3-4.17,1.3-6,0c-0.91,0.65-1.96,1-3,1H2l0,2h1c1.03,0,2.05-0.25,3-0.75c1.89,1,4.11,1,6,0 c1.89,1,4.11,1,6,0h0c0.95,0.5,1.97,0.75,3,0.75H22z"/>
        </SvgIcon>
    }
}
