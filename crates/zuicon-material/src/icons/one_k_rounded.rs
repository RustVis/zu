// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OneKRounded)]
pub fn one_k_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("OneKRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M9.25,15L9.25,15 c-0.41,0-0.75-0.34-0.75-0.75V10.5H7.75C7.34,10.5,7,10.16,7,9.75v0C7,9.34,7.34,9,7.75,9H9c0.55,0,1,0.45,1,1v4.25 C10,14.66,9.66,15,9.25,15z M15.59,15L15.59,15c-0.22,0-0.42-0.1-0.55-0.27l-1.54-1.98v1.5c0,0.41-0.34,0.75-0.75,0.75h0 C12.34,15,12,14.66,12,14.25v-4.5C12,9.34,12.34,9,12.75,9h0c0.41,0,0.75,0.34,0.75,0.75v1.5l1.54-1.98C15.17,9.1,15.38,9,15.59,9 h0c0.58,0,0.91,0.66,0.56,1.12L14.75,12l1.41,1.88C16.5,14.34,16.17,15,15.59,15z"/>
        </SvgIcon>
    }
}
