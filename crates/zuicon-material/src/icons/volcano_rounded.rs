// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VolcanoRounded)]
pub fn volcano_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VolcanoRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16.49,8h-4.14c-0.82,0-1.55,0.5-1.86,1.26L9,13H7.3c-0.79,0-1.51,0.47-1.83,1.19l-2.22,5C2.66,20.51,3.63,22,5.08,22 h14.27c1.33,0,2.29-1.27,1.92-2.55l-2.86-10C18.17,8.59,17.38,8,16.49,8z"/><path d="M14,1L14,1c-0.55,0-1,0.45-1,1v2c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V2C15,1.45,14.55,1,14,1z"/><path d="M19.66,3.34L19.66,3.34c-0.39-0.39-1.02-0.39-1.41,0l-1.41,1.41c-0.39,0.39-0.39,1.02,0,1.41v0 c0.39,0.39,1.02,0.39,1.41,0l1.41-1.41C20.05,4.37,20.05,3.73,19.66,3.34z"/><path d="M11.17,4.76L9.76,3.34c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41l1.41,1.41 c0.39,0.39,1.02,0.39,1.41,0l0,0C11.56,5.78,11.56,5.15,11.17,4.76z"/>
        </SvgIcon>
    }
}
