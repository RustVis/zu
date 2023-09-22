// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EditOffSharp)]
pub fn edit_off_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EditOffSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="m21.41 6.33-3.75-3.75-2.53 2.54 3.75 3.75 2.53-2.54zM1.39 4.22l7.32 7.32L3 17.25V21h3.75l5.71-5.71 7.32 7.32 1.41-1.41L2.81 2.81 1.39 4.22zm16.42 5.72-3.75-3.75-2.52 2.52 3.75 3.75 2.52-2.52z"/>
        </SvgIcon>
    }
}
