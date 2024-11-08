// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CancelScheduleSendRounded)]
pub fn cancel_schedule_send_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CancelScheduleSendRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16.5,9c-0.42,0-0.83,0.04-1.24,0.11L2.4,3.6C1.74,3.31,1.01,3.8,1.01,4.51L1,9.2c0,0.47,0.33,0.88,0.78,0.98L10,12 l-8.22,1.83C1.33,13.93,1,14.33,1,14.8l0.01,4.68c0,0.72,0.73,1.2,1.39,0.92l6.68-2.86C9.59,21.19,12.71,24,16.5,24 c4.14,0,7.5-3.36,7.5-7.5S20.64,9,16.5,9z M16.5,22c-3.03,0-5.5-2.47-5.5-5.5s2.47-5.5,5.5-5.5s5.5,2.47,5.5,5.5S19.53,22,16.5,22 z"/><path d="M18.62,14.38c-0.2-0.2-0.51-0.2-0.71,0l-1.41,1.41l-1.41-1.41c-0.2-0.2-0.51-0.2-0.71,0s-0.2,0.51,0,0.71l1.41,1.41 l-1.41,1.41c-0.2,0.2-0.2,0.51,0,0.71c0.2,0.2,0.51,0.2,0.71,0l1.41-1.41l1.41,1.41c0.2,0.2,0.51,0.2,0.71,0 c0.2-0.2,0.2-0.51,0-0.71l-1.41-1.41l1.41-1.41C18.82,14.89,18.82,14.57,18.62,14.38z"/>
        </SvgIcon>
    }
}
