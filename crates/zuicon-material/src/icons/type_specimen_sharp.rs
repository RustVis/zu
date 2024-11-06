// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TypeSpecimenSharp)]
pub fn type_specimen_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TypeSpecimenSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,2H6v16h16V2z M16.63,14.5l-0.8-2.3h-3.63l-0.82,2.3H9.81l3.38-9h1.61l3.38,9H16.63z"/>
        </SvgIcon>
    }
}
