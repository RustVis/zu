// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TaskAltRounded)]
pub fn task_alt_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TaskAltRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21.29,5.89l-10,10c-0.39,0.39-1.02,0.39-1.41,0l-2.83-2.83c-0.39-0.39-0.39-1.02,0-1.41l0,0c0.39-0.39,1.02-0.39,1.41,0 l2.12,2.12l9.29-9.29c0.39-0.39,1.02-0.39,1.41,0l0,0C21.68,4.87,21.68,5.5,21.29,5.89z M15.77,2.74c-1.69-0.69-3.61-0.93-5.61-0.57 C6.09,2.9,2.84,6.18,2.15,10.25C1.01,17,6.63,22.78,13.34,21.91c3.96-0.51,7.28-3.46,8.32-7.31c0.4-1.47,0.44-2.89,0.21-4.22 c-0.13-0.8-1.12-1.11-1.7-0.54v0c-0.23,0.23-0.33,0.57-0.27,0.89c0.22,1.33,0.12,2.75-0.52,4.26c-1.16,2.71-3.68,4.7-6.61,4.97 c-5.1,0.47-9.33-3.85-8.7-8.98c0.43-3.54,3.28-6.42,6.81-6.91c1.73-0.24,3.37,0.09,4.77,0.81c0.39,0.2,0.86,0.13,1.17-0.18l0,0 c0.48-0.48,0.36-1.29-0.24-1.6C16.31,2.98,16.04,2.85,15.77,2.74z"/>
        </SvgIcon>
    }
}
