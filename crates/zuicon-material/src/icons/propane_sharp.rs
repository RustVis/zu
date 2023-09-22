// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PropaneSharp)]
pub fn propane_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PropaneSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16.75,6L16,6V3H8v3L7.25,6C3.97,6,1.1,8.53,1,11.82C0.9,15.21,3.62,18,7,18v3h2v-3h6v3h2v-3c3.38,0,6.1-2.79,6-6.18 C22.9,8.53,20.03,6,16.75,6z M10,5h4v1h-4V5z"/>
        </SvgIcon>
    }
}
