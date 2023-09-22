// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SendAndArchiveRounded)]
pub fn send_and_archive_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SendAndArchiveRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,12c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S19.76,12,17,12z M19.15,17.85l-1.79,1.79c-0.2,0.2-0.51,0.2-0.71,0 l-1.79-1.79C14.54,17.54,14.76,17,15.21,17h1.29v-2.5c0-0.28,0.22-0.5,0.5-0.5s0.5,0.22,0.5,0.5V17h1.29 C19.24,17,19.46,17.54,19.15,17.85z"/><path d="M17,10c0.1,0,0.19,0.01,0.28,0.01L3,4v6l8,2l-8,2v6l7-2.95c0-0.02,0-0.03,0-0.05C10,13.13,13.13,10,17,10z"/>
        </SvgIcon>
    }
}
