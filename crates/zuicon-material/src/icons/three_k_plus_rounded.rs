// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ThreeKPlusRounded)]
pub fn three_k_plus_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ThreeKPlusRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M10,14c0,0.55-0.45,1-1,1H6.25 c-0.41,0-0.75-0.34-0.75-0.75v0c0-0.41,0.34-0.75,0.75-0.75H8.5v-1H7c-0.28,0-0.5-0.22-0.5-0.5v0c0-0.28,0.22-0.5,0.5-0.5h1.5v-1 H6.25c-0.41,0-0.75-0.34-0.75-0.75v0C5.5,9.34,5.84,9,6.25,9H9c0.55,0,1,0.45,1,1V14z M14.59,15L14.59,15 c-0.22,0-0.42-0.1-0.55-0.27l-1.54-1.98v1.5c0,0.41-0.34,0.75-0.75,0.75h0C11.34,15,11,14.66,11,14.25l-0.03-4.49 C10.96,9.34,11.3,9,11.72,9h0c0.41,0,0.75,0.33,0.75,0.74l0.03,1.51l1.54-1.98C14.17,9.1,14.38,9,14.59,9h0 c0.58,0,0.91,0.66,0.56,1.12L13.75,12l1.41,1.88C15.5,14.34,15.17,15,14.59,15z M18.5,12.5h-1v1c0,0.28-0.22,0.5-0.5,0.5l0,0 c-0.28,0-0.5-0.22-0.5-0.5v-1h-1c-0.28,0-0.5-0.22-0.5-0.5v0c0-0.28,0.22-0.5,0.5-0.5h1v-1c0-0.28,0.22-0.5,0.5-0.5l0,0 c0.28,0,0.5,0.22,0.5,0.5v1h1c0.28,0,0.5,0.22,0.5,0.5v0C19,12.28,18.78,12.5,18.5,12.5z"/>
        </SvgIcon>
    }
}
