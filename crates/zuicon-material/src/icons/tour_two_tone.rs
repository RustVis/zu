// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TourTwoTone)]
pub fn tour_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TourTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,4h-8h-1H7V2H5v2v10v8h2v-8h4h1h9l-2-5L21,4z M7,12V6h11.05l-1.2,3l1.2,3H7z M14,9c0,1.1-0.9,2-2,2s-2-0.9-2-2 s0.9-2,2-2S14,7.9,14,9z"/>
        </SvgIcon>
    }
}
