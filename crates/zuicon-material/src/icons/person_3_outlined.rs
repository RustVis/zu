// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Person3Outlined)]
pub fn person_3_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Person3Outlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18.39,14.56C16.71,13.7,14.53,13,12,13c-2.53,0-4.71,0.7-6.39,1.56C4.61,15.07,4,16.1,4,17.22V20h16v-2.78 C20,16.1,19.39,15.07,18.39,14.56z M18,18H6v-0.78c0-0.38,0.2-0.72,0.52-0.88C7.71,15.73,9.63,15,12,15 c2.37,0,4.29,0.73,5.48,1.34C17.8,16.5,18,16.84,18,17.22V18z"/><path d="M10,12c0.17,0,3.83,0,4,0c1.66,0,3-1.34,3-3c0-0.73-0.27-1.4-0.71-1.92C16.42,6.75,16.5,6.38,16.5,6 c0-1.25-0.77-2.32-1.86-2.77C14,2.48,13.06,2,12,2s-2,0.48-2.64,1.23C8.27,3.68,7.5,4.75,7.5,6c0,0.38,0.08,0.75,0.21,1.08 C7.27,7.6,7,8.27,7,9C7,10.66,8.34,12,10,12z M9.24,8.37C10.11,7.33,9.5,6.37,9.5,6c0-0.41,0.24-0.77,0.62-0.92 c0.29-0.12,0.55-0.31,0.75-0.54C11.04,4.33,11.42,4,12,4s0.96,0.33,1.13,0.53c0.2,0.24,0.46,0.42,0.75,0.54 C14.26,5.23,14.5,5.59,14.5,6c0,0.37-0.61,1.33,0.26,2.37C15.34,9.06,14.8,10,14,10h-4C9.2,10,8.66,9.06,9.24,8.37z"/>
        </SvgIcon>
    }
}
