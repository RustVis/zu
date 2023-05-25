// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SendAndArchiveSharp)]
pub fn send_and_archive_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SendAndArchiveSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,10c0.1,0,0.19,0.01,0.28,0.01L3,4v6l8,2l-8,2v6l7-2.95c0-0.02,0-0.03,0-0.05C10,13.13,13.13,10,17,10z"/><path d="M17,12c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S19.76,12,17,12z M17,20l-3-3h2.5v-3h1v3H20L17,20z"/>
        </SvgIcon>
    }
}
