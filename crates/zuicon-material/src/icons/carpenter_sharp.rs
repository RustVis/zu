// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CarpenterSharp)]
pub fn carpenter_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CarpenterSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M7,1.5L3.11,5.39l8.13,11.67l-1.41,1.41l4.24,4.24l7.07-7.07L7,1.5z M12.66,18.47l4.24-4.24l1.41,1.41l-4.24,4.24 L12.66,18.47z"/>
        </SvgIcon>
    }
}
