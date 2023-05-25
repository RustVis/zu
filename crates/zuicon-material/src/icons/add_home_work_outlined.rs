// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AddHomeWorkOutlined)]
pub fn add_home_work_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AddHomeWorkOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M11,14H5v5H3v-6.97l5-3.57l5,3.57v1.08c0.57-0.59,1.25-1.07,2-1.42V11L8,6l-7,5v10h6v-5h2v5h2.68 C11.25,20.09,11,19.08,11,18V14z"/><path d="M23,13.11V3H10v1.97l2,1.43V5h9v6.68C21.75,12.04,22.43,12.52,23,13.11z"/><path d="M23,18c0-2.76-2.24-5-5-5s-5,2.24-5,5s2.24,5,5,5S23,20.76,23,18z M17.5,21v-2.5H15v-1h2.5V15h1v2.5H21v1h-2.5V21H17.5z"/>
        </SvgIcon>
    }
}
