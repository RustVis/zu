// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Nightlife)]
pub fn nightlife(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Nightlife"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M1,5h14l-6,9v4h2v2H5v-2h2v-4L1,5z M10.1,9l1.4-2H4.49l1.4,2H10.1z M17,5h5v3h-3v9h0c0,1.66-1.34,3-3,3s-3-1.34-3-3 s1.34-3,3-3c0.35,0,0.69,0.06,1,0.17L17,5z"/>
        </SvgIcon>
    }
}
