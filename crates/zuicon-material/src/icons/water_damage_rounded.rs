// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WaterDamageRounded)]
pub fn water_damage_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WaterDamageRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M11.33,3.6l-8.36,7.53C2.63,11.43,2.84,12,3.3,12H5v7c0,0.55,0.45,1,1,1h12c0.55,0,1-0.45,1-1v-7h1.7 c0.46,0,0.68-0.57,0.33-0.87L12.67,3.6C12.29,3.26,11.71,3.26,11.33,3.6z M12,16c-1.1,0-2-0.9-2-2c0-0.78,0.99-2.44,1.58-3.36 c0.2-0.31,0.64-0.31,0.84,0C13.01,11.56,14,13.22,14,14C14,15.1,13.1,16,12,16z"/>
        </SvgIcon>
    }
}
