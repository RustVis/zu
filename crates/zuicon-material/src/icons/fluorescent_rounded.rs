// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FluorescentRounded)]
pub fn fluorescent_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FluorescentRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M7,15h10c1.1,0,2-0.9,2-2v-2c0-1.1-0.9-2-2-2H7c-1.1,0-2,0.9-2,2v2C5,14.1,5.9,15,7,15z"/><path d="M12,2L12,2c-0.56,0-1,0.45-1,1V4c0,0.55,0.45,1,1,1H12c0.55,0,1-0.45,1-1V3C13,2.45,12.55,2,12,2z"/><path d="M19.79,5.3L19.79,5.3c-0.39-0.39-1.02-0.39-1.41,0l-0.38,0.38c-0.39,0.39-0.39,1.02,0,1.41l0,0 c0.39,0.39,1.02,0.39,1.41,0l0.38-0.38C20.18,6.33,20.18,5.69,19.79,5.3z"/><path d="M12,22L12,22c0.56,0,1-0.45,1-1V20c0-0.55-0.45-1-1-1H12c-0.55,0-1,0.45-1,1V21C11,21.55,11.45,22,12,22z"/><path d="M17.99,18.41l0.38,0.39c0.39,0.39,1.02,0.39,1.41,0l0.01-0.01c0.39-0.39,0.39-1.02,0-1.41L19.4,17 c-0.39-0.39-1.02-0.39-1.41,0l0,0C17.6,17.4,17.6,18.03,17.99,18.41z"/><path d="M6,5.69L5.61,5.31c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41L4.59,7.1c0.39,0.39,1.02,0.39,1.41,0l0,0 C6.38,6.71,6.38,6.07,6,5.69z"/><path d="M4.2,18.79L4.2,18.79c0.39,0.4,1.03,0.4,1.42,0L6,18.4c0.39-0.39,0.39-1.02,0-1.41l0,0c-0.39-0.39-1.02-0.39-1.41,0 L4.2,17.38C3.81,17.77,3.81,18.4,4.2,18.79z"/>
        </SvgIcon>
    }
}
