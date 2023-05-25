// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TypeSpecimenTwoTone)]
pub fn type_specimen_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TypeSpecimenTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8,16h12V4H8V16z M13.2,5.5h1.61l3.38,9h-1.56l-0.8-2.3h-3.63l-0.82,2.3H9.81L13.2,5.5z" opacity=".3"/><path d="M4,6H2v14c0,1.1,0.9,2,2,2h14v-2H4V6z"/><path d="M20,2H8C6.9,2,6,2.9,6,4v12c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M20,16H8V4h12V16z"/><path d="M12.19,12.2h3.63l0.8,2.3h1.56l-3.38-9H13.2l-3.38,9h1.56L12.19,12.2z M13.96,7.17h0.08l1.31,3.72h-2.69L13.96,7.17z"/>
        </SvgIcon>
    }
}
