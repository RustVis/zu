// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SwipeRightSharp)]
pub fn swipe_right_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SwipeRightSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.18,15.4L19.1,23h-9L5,17.62l1.22-1.23L10,17.24V6.5C10,5.67,10.67,5,11.5,5S13,5.67,13,6.5v6h1.38L20.18,15.4z M19.91,5.5H17V7h5V2h-1.5v2.02C18.18,2.13,15.22,1,12,1C6.51,1,2.73,4.12,2,7h1.57C4.33,5.02,7.26,2.5,12,2.5 C15.03,2.5,17.79,3.64,19.91,5.5z"/>
        </SvgIcon>
    }
}
