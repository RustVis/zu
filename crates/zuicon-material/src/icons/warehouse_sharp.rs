// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WarehouseSharp)]
pub fn warehouse_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WarehouseSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,21V7L12,3L2,7v14h5v-9h10v9H22z M11,19H9v2h2V19z M13,16h-2v2h2V16z M15,19h-2v2h2V19z"/>
        </SvgIcon>
    }
}
