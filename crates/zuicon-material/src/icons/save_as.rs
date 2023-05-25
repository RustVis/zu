// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SaveAs)]
pub fn save_as(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SaveAs"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,12.4V7l-4-4H5C3.89,3,3,3.9,3,5v14c0,1.1,0.89,2,2,2h7.4L21,12.4z M15,15c0,1.66-1.34,3-3,3s-3-1.34-3-3s1.34-3,3-3 S15,13.34,15,15z M6,6h9v4H6V6z M19.99,16.25l1.77,1.77L16.77,23H15v-1.77L19.99,16.25z M23.25,16.51l-0.85,0.85l-1.77-1.77 l0.85-0.85c0.2-0.2,0.51-0.2,0.71,0l1.06,1.06C23.45,16,23.45,16.32,23.25,16.51z"/>
        </SvgIcon>
    }
}
