// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(StickyNote2TwoTone)]
pub fn sticky_note_2_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("StickyNote2TwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,5v14h9v-5h5V5H5z M12,14H7v-2h5V14z M17,10H7V8h10V10z" opacity=".3"/><path d="M19,5v9l-5,0l0,5H5V5H19 M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h10l6-6V5C21,3.9,20.1,3,19,3z M12,14H7v-2h5V14z M17,10H7V8h10V10z"/>
        </SvgIcon>
    }
}
