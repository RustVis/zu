// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MedicationRounded)]
pub fn medication_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MedicationRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M17,3H7C6.45,3,6,3.45,6,4v0c0,0.55,0.45,1,1,1h10c0.55,0,1-0.45,1-1v0C18,3.45,17.55,3,17,3z"/><path d="M17,6H7C5.9,6,5,6.9,5,8v11c0,1.1,0.9,2,2,2h10c1.1,0,2-0.9,2-2V8C19,6.9,18.1,6,17,6z M14.5,15h-1v1 c0,0.83-0.67,1.5-1.5,1.5h0c-0.83,0-1.5-0.67-1.5-1.5v-1h-1C8.67,15,8,14.33,8,13.5v0C8,12.67,8.67,12,9.5,12h1v-1 c0-0.83,0.67-1.5,1.5-1.5h0c0.83,0,1.5,0.67,1.5,1.5v1h1c0.83,0,1.5,0.67,1.5,1.5v0C16,14.33,15.33,15,14.5,15z"/>
        </SvgIcon>
    }
}
