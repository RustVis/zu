// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ForestTwoTone)]
pub fn forest_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ForestTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.14,12H22L15,2l-3,4.29L9,2L2,12h1.86L0,18h7v4h4v-4h2v4h4v-4h7L20.14,12z M3.66,16l3.86-6H5.84L9,5.49L12.16,10h-1.68 l3.86,6H3.66z M16.71,16l-2.57-4H16l-2.78-3.97L15,5.49L18.16,10h-1.68l3.86,6H16.71z"/>
        </SvgIcon>
    }
}
