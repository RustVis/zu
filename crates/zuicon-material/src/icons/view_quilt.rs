// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewQuilt)]
pub fn view_quilt(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewQuilt"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,5v6.5H9.33V5H21z M14.67,19v-6.5H9.33V19H14.67z M15.67,12.5V19H21v-6.5H15.67z M8.33,19V5H3v14H8.33z"/>
        </SvgIcon>
    }
}
