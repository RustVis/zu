// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DataThresholdingSharp)]
pub fn data_thresholding_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DataThresholdingSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,3H3v18h18V3z M10.67,8.17l2,2l3.67-3.67l1.41,1.41L12.67,13l-2-2l-3,3l-1.41-1.41L10.67,8.17z M5,16h1.72L5,17.72V16z M5.84,19l3-3h1.83l-3,3H5.84z M9.8,19l3-3h1.62l-3,3H9.8z M13.53,19l3-3h1.62l-3,3H13.53z M19,19h-1.73L19,17.27V19z"/>
        </SvgIcon>
    }
}
