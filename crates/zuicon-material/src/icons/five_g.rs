// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FiveG)]
pub fn five_g(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FiveG"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,13h2v2h-5V9h7c0-1.1-0.9-2-2-2h-5c-1.1,0-2,0.9-2,2v6c0,1.1,0.9,2,2,2h5c1.1,0,2-0.9,2-2v-4h-4V13z"/><path d="M3,13h5v2H3v2h5c1.1,0,2-0.9,2-2v-2c0-1.1-0.9-2-2-2H5V9h5V7H3V13z"/>
        </SvgIcon>
    }
}
