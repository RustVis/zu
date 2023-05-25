// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EditNoteSharp)]
pub fn edit_note_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EditNoteSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,10h11v2H3V10z M3,8h11V6H3V8z M3,16h7v-2H3V16z M18.01,12.87l1.41-1.41l2.12,2.12l-1.41,1.41L18.01,12.87z M17.3,13.58 l-5.3,5.3V21h2.12l5.3-5.3L17.3,13.58z"/>
        </SvgIcon>
    }
}
