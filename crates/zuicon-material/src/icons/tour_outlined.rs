// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TourOutlined)]
pub fn tour_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TourOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,4h-8h-1H7V2H5v2v10v8h2v-8h4h1h9l-2-5L21,4z M17.14,9.74l0.9,2.26H12h-1H7V6h5h1h5.05l-0.9,2.26L16.85,9L17.14,9.74z M14,9c0,1.1-0.9,2-2,2s-2-0.9-2-2s0.9-2,2-2S14,7.9,14,9z"/>
        </SvgIcon>
    }
}
