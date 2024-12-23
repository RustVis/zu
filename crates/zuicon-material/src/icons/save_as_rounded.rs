// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SaveAsRounded)]
pub fn save_as_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SaveAsRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.41,6.41l-2.83-2.83C17.21,3.21,16.7,3,16.17,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h7.4l8.6-8.6V7.83 C21,7.3,20.79,6.79,20.41,6.41z M12,18c-1.66,0-3-1.34-3-3s1.34-3,3-3s3,1.34,3,3S13.66,18,12,18z M15,9c0,0.55-0.45,1-1,1H7 c-0.55,0-1-0.45-1-1V7c0-0.55,0.45-1,1-1h7c0.55,0,1,0.45,1,1V9z M19.99,16.25l1.77,1.77l-4.84,4.84C16.82,22.95,16.69,23,16.56,23 H15.5c-0.28,0-0.5-0.22-0.5-0.5v-1.06c0-0.13,0.05-0.26,0.15-0.35L19.99,16.25z M23.25,16.51l-0.85,0.85l-1.77-1.77l0.85-0.85 c0.2-0.2,0.51-0.2,0.71,0l1.06,1.06C23.45,16,23.45,16.32,23.25,16.51z"/>
        </SvgIcon>
    }
}
