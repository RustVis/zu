// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EnergySavingsLeafSharp)]
pub fn energy_savings_leaf_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EnergySavingsLeafSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,3c-4.8,0-9,3.86-9,9c0,2.12,0.74,4.07,1.97,5.61L3,19.59L4.41,21l1.97-1.97C7.93,20.26,9.88,21,12,21 c2.3,0,4.61-0.88,6.36-2.64C20.12,16.61,21,14.3,21,12V3H12z M10.5,17L10,16.5l2.5-3.5l-5-0.5l6-5.5L14,7.5L11.5,11l5,0.5L10.5,17 z"/>
        </SvgIcon>
    }
}
