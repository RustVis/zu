// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VillaSharp)]
pub fn villa_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VillaSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M7,21H3V8l13-5v7H7V21z M19,10c-1.1,0-2,0.9-2,2H9v9h5v-5h2v5h5v-9C21,10.9,20.1,10,19,10z"/>
        </SvgIcon>
    }
}
