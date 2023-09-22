// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(StartSharp)]
pub fn start_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("StartSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14.59,7.41L18.17,11H6v2h12.17l-3.59,3.59L16,18l6-6l-6-6L14.59,7.41z M2,6v12h2V6H2z"/>
        </SvgIcon>
    }
}
