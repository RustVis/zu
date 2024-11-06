// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RampRightOutlined)]
pub fn ramp_right_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RampRightOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M11,21h2V6.83l1.59,1.59L16,7l-4-4L8,7l1.41,1.41L11,6.83V9c0,4.27-4.03,7.13-6,8.27l1.46,1.46 C8.37,17.56,9.9,16.19,11,14.7L11,21z"/>
        </SvgIcon>
    }
}
