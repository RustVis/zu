// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HourglassTopRounded)]
pub fn hourglass_top_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HourglassTopRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8,2C6.9,2,6,2.9,6,4l0.01,3.18c0,0.53,0.21,1.03,0.58,1.41L10,12l-3.41,3.43c-0.37,0.37-0.58,0.88-0.58,1.41L6,20 c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2v-3.16c0-0.53-0.21-1.04-0.58-1.41L14,12l3.41-3.4C17.79,8.22,18,7.71,18,7.18V4 c0-1.1-0.9-2-2-2H8z M16,16.91V19c0,0.55-0.45,1-1,1H9c-0.55,0-1-0.45-1-1v-2.09c0-0.27,0.11-0.52,0.29-0.71L12,12.5l3.71,3.71 C15.89,16.39,16,16.65,16,16.91z"/>
        </SvgIcon>
    }
}
