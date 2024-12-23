// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(JoinRightRounded)]
pub fn join_right_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("JoinRightRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M11.32,17.2c0.39,0.35,0.98,0.35,1.37,0C14.65,15.44,15,13.16,15,12c0-1.15-0.35-3.44-2.32-5.2 c-0.39-0.35-0.98-0.35-1.37,0C9.35,8.56,9,10.84,9,12C9,13.15,9.35,15.44,11.32,17.2z"/><path d="M16.5,12c0,0.97-0.23,4.16-3.03,6.5C14.25,18.81,15.1,19,16,19c3.86,0,7-3.14,7-7s-3.14-7-7-7c-0.9,0-1.75,0.19-2.53,0.5 C16.27,7.84,16.5,11.03,16.5,12z"/><path d="M8,19c0.9,0,1.75-0.19,2.53-0.5c-0.61-0.51-1.1-1.07-1.49-1.63C8.71,16.95,8.36,17,8,17c-2.76,0-5-2.24-5-5s2.24-5,5-5 c0.36,0,0.71,0.05,1.04,0.13c0.39-0.56,0.88-1.12,1.49-1.63C9.75,5.19,8.9,5,8,5c-3.86,0-7,3.14-7,7S4.14,19,8,19z"/>
        </SvgIcon>
    }
}
