// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MovingRounded)]
pub fn moving_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MovingRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2.7,17.29c0.39,0.39,1.02,0.39,1.41,0l4.59-4.59c0.39-0.39,1.02-0.39,1.41,0l1.17,1.17c1.17,1.17,3.07,1.17,4.24,0 l4.18-4.17l1.44,1.44c0.31,0.31,0.85,0.09,0.85-0.35V6.5C22,6.22,21.78,6,21.5,6h-4.29c-0.45,0-0.67,0.54-0.35,0.85l1.44,1.44 l-4.17,4.17c-0.39,0.39-1.02,0.39-1.41,0l-1.17-1.17c-1.17-1.17-3.07-1.17-4.24,0L2.7,15.88C2.32,16.27,2.32,16.91,2.7,17.29z"/>
        </SvgIcon>
    }
}
