// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OutputTwoTone)]
pub fn output_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("OutputTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,19H5V5h14v2h2V5c0-1.1-0.89-2-2-2H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.11,0,2-0.9,2-2v-2h-2V19z"/>
        </SvgIcon>
    }
}
