// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CheckroomSharp)]
pub fn checkroom_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CheckroomSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21.6,18.2L13,11.75v-0.91c1.65-0.49,2.8-2.17,2.43-4.05c-0.26-1.31-1.3-2.4-2.61-2.7C10.54,3.57,8.5,5.3,8.5,7.5h2 C10.5,6.67,11.17,6,12,6s1.5,0.67,1.5,1.5c0,0.84-0.69,1.52-1.53,1.5L11,9l0,2.75L2.4,18.2C1.63,18.78,2.04,20,3,20h9h9 C21.96,20,22.37,18.78,21.6,18.2z M6,18l6-4.5l6,4.5H6z"/>
        </SvgIcon>
    }
}
