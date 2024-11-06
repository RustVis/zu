// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PushPinSharp)]
pub fn push_pin_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PushPinSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16,9V4l2,0V2H6v2l2,0v5c0,1.66-1.34,3-3,3h0v2h5.97v7l1,1l1-1v-7H19v-2h0 C17.34,12,16,10.66,16,9z" fill-rule="evenodd"/>
        </SvgIcon>
    }
}
