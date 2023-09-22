// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SmartButtonSharp)]
pub fn smart_button_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SmartButtonSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,17h-3l0-2h1V9H4v6h6v2H2V7h20V17z M14.5,19l1.09-2.41L18,15.5l-2.41-1.09L14.5,12l-1.09,2.41L11,15.5l2.41,1.09 L14.5,19z M17,14l0.62-1.38L19,12l-1.38-0.62L17,10l-0.62,1.38L15,12l1.38,0.62L17,14z"/>
        </SvgIcon>
    }
}
