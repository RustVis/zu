// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(YardOutlined)]
pub fn yard_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("YardOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M18,13c-3.31,0-6,2.69-6,6C15.31,19,18,16.31,18,13z M6,13c0,3.31,2.69,6,6,6C12,15.69,9.31,13,6,13z M8,11.03 c0,0.86,0.7,1.56,1.56,1.56c0.33,0,0.63-0.1,0.89-0.28l-0.01,0.12c0,0.86,0.7,1.56,1.56,1.56s1.56-0.7,1.56-1.56l-0.01-0.12 c0.25,0.17,0.56,0.28,0.89,0.28c0.86,0,1.56-0.7,1.56-1.56c0-0.62-0.37-1.16-0.89-1.41C15.63,9.38,16,8.84,16,8.22 c0-0.86-0.7-1.56-1.56-1.56c-0.33,0-0.63,0.1-0.89,0.28l0.01-0.12c0-0.86-0.7-1.56-1.56-1.56s-1.56,0.7-1.56,1.56l0.01,0.12 C10.2,6.76,9.89,6.66,9.56,6.66C8.7,6.66,8,7.36,8,8.22c0,0.62,0.37,1.16,0.89,1.41C8.37,9.87,8,10.41,8,11.03z M12,8.06 c0.86,0,1.56,0.7,1.56,1.56s-0.7,1.56-1.56,1.56s-1.56-0.7-1.56-1.56S11.14,8.06,12,8.06z M20,4v16H4V4H20 M20,2H4C2.9,2,2,2.9,2,4 v16c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z"/>
        </SvgIcon>
    }
}
