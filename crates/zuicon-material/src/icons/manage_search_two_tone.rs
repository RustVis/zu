// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ManageSearchTwoTone)]
pub fn manage_search_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ManageSearchTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M18.17,13.75C18.69,12.96,19,12.02,19,11c0-2.76-2.24-5-5-5s-5,2.24-5,5s2.24,5,5,5c1.02,0,1.96-0.31,2.76-0.83L20.59,19 L22,17.59L18.17,13.75z M14,14c-1.65,0-3-1.35-3-3c0-1.65,1.35-3,3-3s3,1.35,3,3C17,12.65,15.65,14,14,14z"/>
        </SvgIcon>
    }
}
