// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DataThresholdingOutlined)]
pub fn data_thresholding_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DataThresholdingOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M19,19h-1.73L19,17.27V19z M19,16 h-0.85l-3,3h-1.62l3-3h-2.12l-3,3H9.8l3-3h-2.12l-3,3H5.84l3-3H6.72L5,17.72V5h14V16z"/>
        </SvgIcon>
    }
}
