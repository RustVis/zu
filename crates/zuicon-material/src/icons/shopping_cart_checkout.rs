// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ShoppingCartCheckout)]
pub fn shopping_cart_checkout(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ShoppingCartCheckout"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M7,18c-1.1,0-1.99,0.9-1.99,2S5.9,22,7,22s2-0.9,2-2S8.1,18,7,18z M17,18c-1.1,0-1.99,0.9-1.99,2s0.89,2,1.99,2s2-0.9,2-2 S18.1,18,17,18z M8.1,13h7.45c0.75,0,1.41-0.41,1.75-1.03L21,4.96L19.25,4l-3.7,7H8.53L4.27,2H1v2h2l3.6,7.59l-1.35,2.44 C4.52,15.37,5.48,17,7,17h12v-2H7L8.1,13z M12,2l4,4l-4,4l-1.41-1.41L12.17,7L8,7l0-2l4.17,0l-1.59-1.59L12,2z"/>
        </SvgIcon>
    }
}
