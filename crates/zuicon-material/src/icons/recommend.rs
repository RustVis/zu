// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Recommend)]
pub fn recommend(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Recommend"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0z" fill="none"/><path d="M12 2a10 10 0 1010 10A10 10 0 0012 2zm6 9.8a.9.9 0 01-.1.5l-2.1 4.9a1.34 1.34 0 01-1.3.8H9a2 2 0 01-2-2v-5a1.28 1.28 0 01.4-1L12 5l.69.69a1.08 1.08 0 01.3.7v.2L12.41 10H17a1 1 0 011 1z"/>
        </SvgIcon>
    }
}
