// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HideImage)]
pub fn hide_image(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HideImage"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,5c0-1.1-0.9-2-2-2H5.83L21,18.17V5z"/><path d="M2.81,2.81L1.39,4.22L3,5.83V19c0,1.1,0.9,2,2,2h13.17l1.61,1.61l1.41-1.41L2.81,2.81z M6,17l3-4l2.25,3l0.82-1.1l2.1,2.1 H6z"/>
        </SvgIcon>
    }
}
