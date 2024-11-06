// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FileOpenSharp)]
pub fn file_open_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FileOpenSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14,2H4v20h11v-8h5V8L14,2z M13,9V3.5L18.5,9H13z M17,21.66V16h5.66v2h-2.24l2.95,2.95l-1.41,1.41L19,19.41l0,2.24H17z"/>
        </SvgIcon>
    }
}
