// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WrongLocationRounded)]
pub fn wrong_location_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WrongLocationRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.42,4.5l1.38-1.38c0.39-0.39,0.39-1.02,0-1.41l0,0c-0.39-0.39-1.02-0.39-1.41,0L19,3.08 l-1.38-1.38c-0.39-0.39-1.02-0.39-1.41,0s-0.39,1.02,0,1.41l1.38,1.38l-1.38,1.38c-0.39,0.39-0.39,1.02,0,1.41l0,0 c0.39,0.39,1.02,0.39,1.41,0L19,5.92l1.38,1.38c0.39,0.39,1.02,0.39,1.41,0l0,0c0.39-0.39,0.39-1.02,0-1.41L20.42,4.5z" enable-background="new"/><path d="M19.67,8L19,7.33l-0.59,0.59c-0.7,0.7-1.84,0.88-2.65,0.3c-1.03-0.74-1.12-2.19-0.26-3.05 l0.67-0.67L15.5,3.83c-0.36-0.36-0.54-0.81-0.57-1.28C14.01,2.19,13.02,2,12,2c-4.2,0-8,3.22-8,8.2c0,3.18,2.45,6.92,7.34,11.23 c0.38,0.33,0.95,0.33,1.33,0C17.55,17.12,20,13.38,20,10.2c0-0.76-0.1-1.47-0.26-2.14C19.72,8.04,19.69,8.02,19.67,8z M12,12 c-1.1,0-2-0.9-2-2s0.9-2,2-2c1.1,0,2,0.9,2,2S13.1,12,12,12z" enable-background="new"/>
        </SvgIcon>
    }
}
