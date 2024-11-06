// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DynamicFormSharp)]
pub fn dynamic_form_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DynamicFormSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,20v-9h-2V4h7l-2,5h2L17,20z M15,13v7H2v-7H15z M6.25,15.75h-1.5v1.5h1.5V15.75z M13,4v7H2V4H13z M6.25,6.75h-1.5v1.5 h1.5V6.75z"/>
        </SvgIcon>
    }
}
