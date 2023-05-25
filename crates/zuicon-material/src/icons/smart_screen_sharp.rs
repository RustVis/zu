// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SmartScreenSharp)]
pub fn smart_screen_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SmartScreenSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M1,5v14h22V5H1z M18,17H6V7h12V17z"/>
        </SvgIcon>
    }
}
