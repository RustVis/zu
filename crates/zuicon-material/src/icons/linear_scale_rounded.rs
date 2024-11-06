// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LinearScaleRounded)]
pub fn linear_scale_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LinearScaleRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,7c-2.41,0-4.43,1.72-4.9,4H6.79C6.4,10.12,5.52,9.5,4.5,9.5C3.12,9.5,2,10.62,2,12s1.12,2.5,2.5,2.5 c1.02,0,1.9-0.62,2.29-1.5h5.31c0.46,2.28,2.48,4,4.9,4c2.76,0,5-2.24,5-5S19.76,7,17,7z M17,15c-1.65,0-3-1.35-3-3s1.35-3,3-3 s3,1.35,3,3S18.65,15,17,15z"/>
        </SvgIcon>
    }
}
