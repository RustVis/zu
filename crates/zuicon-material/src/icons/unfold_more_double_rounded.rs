// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(UnfoldMoreDoubleRounded)]
pub fn unfold_more_double_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("UnfoldMoreDoubleRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9.53,5.29L12,2.83l2.46,2.46c0.39,0.39,1.02,0.39,1.41,0s0.39-1.02,0-1.41L12.7,0.7c-0.39-0.39-1.02-0.39-1.41,0 L8.12,3.88c-0.39,0.39-0.39,1.02,0,1.41S9.14,5.68,9.53,5.29z"/><path d="M9.53,10.29L12,7.83l2.46,2.46c0.39,0.39,1.02,0.39,1.41,0s0.39-1.02,0-1.41L12.7,5.7c-0.39-0.39-1.02-0.39-1.41,0 L8.12,8.88c-0.39,0.39-0.39,1.02,0,1.41S9.14,10.68,9.53,10.29z"/><path d="M14.47,13.71L12,16.17l-2.46-2.46c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41l3.17,3.18 c0.39,0.39,1.02,0.39,1.41,0l3.17-3.18c0.39-0.39,0.39-1.02,0-1.41C15.49,13.32,14.86,13.32,14.47,13.71z"/><path d="M14.47,18.72L12,21.17l-2.46-2.46c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41l3.17,3.18 c0.39,0.39,1.02,0.39,1.41,0l3.17-3.17c0.39-0.39,0.39-1.02,0-1.41S14.86,18.33,14.47,18.72z"/>
        </SvgIcon>
    }
}
