// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TableRestaurantSharp)]
pub fn table_restaurant_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TableRestaurantSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22.33,11l-2-7H3.67l-2,7H5.2L4,20h2l0.67-5h10.67L18,20h2l-1.2-9H22.33z M6.93,13l0.27-2h9.6l0.27,2H6.93z"/>
        </SvgIcon>
    }
}
