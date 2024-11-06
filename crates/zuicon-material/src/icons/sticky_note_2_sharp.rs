// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(StickyNote2Sharp)]
pub fn sticky_note_2_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("StickyNote2Sharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2.99,3L3,21h12l6-6V3H2.99z M7,8h10v2H7V8z M12,14H7v-2h5V14z M14,19.5V14h5.5L14,19.5z"/>
        </SvgIcon>
    }
}
