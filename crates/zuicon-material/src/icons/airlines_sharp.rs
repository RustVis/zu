// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AirlinesSharp)]
pub fn airlines_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AirlinesSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13,4L2,20h17l3-16H13z M14.5,14c-1.38,0-2.5-1.12-2.5-2.5c0-1.38,1.12-2.5,2.5-2.5s2.5,1.12,2.5,2.5 C17,12.88,15.88,14,14.5,14z"/>
        </SvgIcon>
    }
}
