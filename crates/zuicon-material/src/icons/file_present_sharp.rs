// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FilePresentSharp)]
pub fn file_present_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FilePresentSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M14,2H4v20h16V8L14,2z M16,15c0,2.21-1.79,4-4,4s-4-1.79-4-4V9.5C8,8.12,9.12,7,10.5,7S13,8.12,13,9.5V15h-2V9.5 C11,9.22,10.78,9,10.5,9S10,9.22,10,9.5V15c0,1.1,0.9,2,2,2s2-0.9,2-2v-4h2V15z M14,8V4l4,4H14z"/>
        </SvgIcon>
    }
}
