// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AutoDeleteSharp)]
pub fn auto_delete_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AutoDeleteSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16,9c-0.7,0-1.37,0.1-2,0.29V5H2v14h7.68c1.12,2.36,3.53,4,6.32,4c3.87,0,7-3.13,7-7C23,12.13,19.87,9,16,9z M16,21 c-2.76,0-5-2.24-5-5s2.24-5,5-5s5,2.24,5,5S18.76,21,16,21z"/>
        </SvgIcon>
    }
}
