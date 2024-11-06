// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OilBarrelSharp)]
pub fn oil_barrel_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("OilBarrelSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,13v-2h-2V5h2V3H3v2h2v6H3v2h2v6H3v2h18v-2h-2v-6H21z M12,16c-1.66,0-3-1.32-3-2.95c0-1.3,0.52-1.67,3-4.55 c2.47,2.86,3,3.24,3,4.55C15,14.68,13.66,16,12,16z"/>
        </SvgIcon>
    }
}
