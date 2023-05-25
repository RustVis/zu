// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CleaningServicesRounded)]
pub fn cleaning_services_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CleaningServicesRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16,11h-1V4c0-1.66-1.34-3-3-3h0c-1.66,0-3,1.34-3,3v7H8c-2.76,0-5,2.24-5,5v5c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2v-5 C21,13.24,18.76,11,16,11z M19,21h-2v-3c0-0.55-0.45-1-1-1s-1,0.45-1,1v3h-2v-3c0-0.55-0.45-1-1-1s-1,0.45-1,1v3H9v-3 c0-0.55-0.45-1-1-1s-1,0.45-1,1v3H5v-5c0-1.65,1.35-3,3-3h8c1.65,0,3,1.35,3,3V21z"/>
        </SvgIcon>
    }
}
