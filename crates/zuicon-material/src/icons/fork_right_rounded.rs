// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ForkRightRounded)]
pub fn fork_right_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ForkRightRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9,20c0,0.55,0.45,1,1,1c0.55,0,1-0.45,1-1v-3c0.73-2.58,3.07-3.47,5.17-3l-0.88,0.88c-0.39,0.39-0.39,1.02,0,1.41 c0.39,0.39,1.02,0.39,1.41,0l2.59-2.59c0.39-0.39,0.39-1.02,0-1.41l-2.59-2.59c-0.39-0.39-1.02-0.39-1.41,0 c-0.39,0.39-0.39,1.02,0,1.41L16.17,12c-1.51-0.33-3.73,0.08-5.17,1.36l0-6.53l0.88,0.88c0.39,0.39,1.02,0.39,1.41,0 c0.39-0.39,0.39-1.02,0-1.41l-2.59-2.59c-0.39-0.39-1.02-0.39-1.41,0L6.71,6.29c-0.39,0.39-0.39,1.02,0,1.41 c0.39,0.39,1.02,0.39,1.41,0L9,6.83V20z"/>
        </SvgIcon>
    }
}
