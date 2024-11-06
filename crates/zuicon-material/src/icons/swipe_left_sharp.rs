// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SwipeLeftSharp)]
pub fn swipe_left_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SwipeLeftSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.18,15.4L19.1,23h-9L5,17.62l1.22-1.23L10,17.24V6.5C10,5.67,10.67,5,11.5,5S13,5.67,13,6.5v6h1.38L20.18,15.4z M12,2.5 c4.74,0,7.67,2.52,8.43,4.5H22c-0.73-2.88-4.51-6-10-6C8.78,1,5.82,2.13,3.5,4.02V2H2v5h5V5.5H4.09C6.21,3.64,8.97,2.5,12,2.5z"/>
        </SvgIcon>
    }
}
