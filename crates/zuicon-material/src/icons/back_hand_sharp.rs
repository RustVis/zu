// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BackHandSharp)]
pub fn back_hand_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BackHandSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2.21,10.47L5,9.36L7.25,15H8V2h2.5v10h1V0H14v12h1V1.5h2.5V12h1V4.5H21V16c0,4.42-3.58,8-8,8c-3.26,0-6.19-1.99-7.4-5.02 L2.21,10.47z"/>
        </SvgIcon>
    }
}
