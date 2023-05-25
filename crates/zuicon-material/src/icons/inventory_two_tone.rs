// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(InventoryTwoTone)]
pub fn inventory_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("InventoryTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,11.5l1.5,1.5l-6.99,7L11,15.5l1.5-1.5l3.01,3L21,11.5z"/><path d="M5,19V5h2v3h10V5h2v5.67l2-2l0,0V5c0-1.1-0.9-2-2-2h-4.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H5C3.9,3,3,3.9,3,5v14 c0,1.1,0.9,2,2,2h8.68l-2-2H5z M12,3c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1S11.45,3,12,3z"/>
        </SvgIcon>
    }
}
