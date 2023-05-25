// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TakeoutDining)]
pub fn takeout_dining(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TakeoutDining"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5.26,11h13.48l-0.67,9H5.93L5.26,11z M9.02,4h5.95L19,7.38l1.59-1.59L22,7.21 L19.21,10H4.79L2,7.21l1.41-1.41L5,7.38L9.02,4z" fill-rule="evenodd"/>
        </SvgIcon>
    }
}
