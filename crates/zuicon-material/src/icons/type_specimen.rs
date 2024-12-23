// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TypeSpecimen)]
pub fn type_specimen(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TypeSpecimen"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,6H2v14c0,1.1,0.9,2,2,2h14v-2H4V6z"/><path d="M20,2H8C6.9,2,6,2.9,6,4v12c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M16.63,14.5l-0.8-2.3h-3.63 l-0.82,2.3H9.81l3.38-9h1.61l3.38,9H16.63z"/>
        </SvgIcon>
    }
}
