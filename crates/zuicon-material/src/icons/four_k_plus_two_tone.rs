// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FourKPlusTwoTone)]
pub fn four_k_plus_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FourKPlusTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,19h14v-6.5h-1.5V14h-1v-1.5H15v-1h1.5V10h1v1.5H19V5H5V19z M11.5,9H13v2.25 L14.75,9h1.75l-2.25,3l2.25,3h-1.75L13,12.75V15h-1.5V9z M5.5,9H7v3h1.5V9H10v3h1v1.5h-1V15H8.5v-1.5h-3V9z" enable-background="new" opacity=".3"/><path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M19,11.5h-1.5V10h-1v1.5H15v1h1.5 V14h1v-1.5H19V19H5V5h14V11.5z"/>
        </SvgIcon>
    }
}
