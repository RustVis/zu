// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TurnRight)]
pub fn turn_right(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TurnRight"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17.17,11l-1.59,1.59L17,14l4-4l-4-4l-1.41,1.41L17.17,9L9,9c-1.1,0-2,0.9-2,2v9h2v-9L17.17,11z"/>
        </SvgIcon>
    }
}
