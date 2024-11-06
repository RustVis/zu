// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FaceRetouchingNaturalTwoTone)]
pub fn face_retouching_natural_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FaceRetouchingNaturalTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10.66,4.12c2.55,4.23,7.03,3.87,7.18,3.86l-0.57-1.25L12.4,4.5l0.85-0.39C12.84,4.04,12.43,4,12,4 C11.54,4,11.09,4.05,10.66,4.12z" opacity=".3"/><path d="M8.08,5.03C6.37,6,5.05,7.58,4.42,9.47C6.13,8.5,7.45,6.92,8.08,5.03z" opacity=".3"/><path d="M19.89,10.75C19.96,11.16,20,11.57,20,12c0,4.41-3.59,8-8,8s-8-3.59-8-8c0-0.05,0.01-0.1,0-0.14 c2.6-0.98,4.69-2.99,5.74-5.55c3.38,4.14,7.97,3.73,8.99,3.61l-0.89-1.93c-0.13,0.01-4.62,0.38-7.18-3.86 c1.01-0.16,1.71-0.15,2.59-0.01l2.12-0.97l0.64-0.29C14.78,2.3,13.43,2,12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10 c0-1.43-0.3-2.78-0.84-4.01L19.89,10.75z M8.08,5.03C7.45,6.92,6.13,8.5,4.42,9.47C5.05,7.58,6.37,6,8.08,5.03z"/>
        </SvgIcon>
    }
}
