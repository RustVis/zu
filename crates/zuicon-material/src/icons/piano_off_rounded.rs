// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PianoOffRounded)]
pub fn piano_off_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PianoOffRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.49,21.9c0.39-0.39,0.39-1.02,0-1.41L3.51,3.51c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41L3,5.83V19 c0,1.1,0.9,2,2,2h13.17l0.9,0.9C19.46,22.29,20.09,22.29,20.49,21.9z M8.25,19H5V7.83l2,2v3.67c0,0.55,0.45,1,1,1h0.25V19z M9.75,19 v-4.5H10c0.46,0,0.82-0.31,0.94-0.73l3.31,3.31V19H9.75z M11,8.17L5.83,3H19c1.1,0,2,0.9,2,2v13.17l-2-2V5h-2v8.5 c0,0.19-0.07,0.36-0.16,0.51L13,10.17V5h-2V8.17z"/>
        </SvgIcon>
    }
}
