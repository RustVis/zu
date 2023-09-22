// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Face3Sharp)]
pub fn face_3_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Face3Sharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22.91,11.96c-0.54-5.93-5.75-10.41-11.8-9.92C5.73,2.46,1.55,6.94,1.06,12.32L0,24h24c0,0,0,0,0,0L22.91,11.96z M4.54,9.13C5.41,9.68,6.43,10,7.5,10C9.36,10,11,9.07,12,7.65C13,9.07,14.64,10,16.5,10c1.07,0,2.09-0.32,2.96-0.87 C19.8,10.02,20,10.99,20,12c0,4.41-3.59,8-8,8s-8-3.59-8-8C4,10.99,4.2,10.02,4.54,9.13z"/>
        </SvgIcon>
    }
}
