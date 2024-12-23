// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OpenInNewOffRounded)]
pub fn open_in_new_off_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("OpenInNewOffRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16.79,5.8l-1.94-1.94C14.54,3.54,14.76,3,15.21,3h5.29C20.78,3,21,3.22,21,3.5v5.29c0,0.45-0.54,0.67-0.85,0.35L18.21,7.2 l-4.09,4.09l-1.41-1.41L16.79,5.8z M19,13v3.17l2,2V13c0-0.55-0.45-1-1-1H20C19.45,12,19,12.45,19,13z M19.07,21.9l-0.9-0.9H5 c-1.11,0-2-0.9-2-2V5.83l-0.9-0.9c-0.39-0.39-0.39-1.02,0-1.41l0,0c0.39-0.39,1.02-0.39,1.41,0l16.97,16.97 c0.39,0.39,0.39,1.02,0,1.41l0,0C20.09,22.29,19.46,22.29,19.07,21.9z M16.17,19l-4.88-4.88L10.41,15c-0.39,0.39-1.02,0.39-1.41,0 l0,0c-0.39-0.39-0.39-1.02,0-1.41l0.88-0.88L5,7.83V19H16.17z M7.83,5H11c0.55,0,1-0.45,1-1V4c0-0.55-0.45-1-1-1H5.83L7.83,5z"/>
        </SvgIcon>
    }
}
