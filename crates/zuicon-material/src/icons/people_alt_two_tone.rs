// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PeopleAltTwoTone)]
pub fn people_alt_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PeopleAltTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9,15c-2.7,0-5.8,1.29-6,2.01L3,18h12v-1C14.8,16.29,11.7,15,9,15z" opacity=".3"/><path d="M16.67,13.13C18.04,14.06,19,15.32,19,17v3h4v-3C23,14.82,19.43,13.53,16.67,13.13z"/><path d="M15,12c2.21,0,4-1.79,4-4c0-2.21-1.79-4-4-4c-0.47,0-0.91,0.1-1.33,0.24C14.5,5.27,15,6.58,15,8s-0.5,2.73-1.33,3.76 C14.09,11.9,14.53,12,15,12z"/><path d="M9,12c2.21,0,4-1.79,4-4c0-2.21-1.79-4-4-4S5,5.79,5,8C5,10.21,6.79,12,9,12z M9,6c1.1,0,2,0.9,2,2c0,1.1-0.9,2-2,2 S7,9.1,7,8C7,6.9,7.9,6,9,6z"/><path d="M9,13c-2.67,0-8,1.34-8,4v3h16v-3C17,14.34,11.67,13,9,13z M15,18H3l0-0.99C3.2,16.29,6.3,15,9,15s5.8,1.29,6,2V18z"/>
        </SvgIcon>
    }
}
