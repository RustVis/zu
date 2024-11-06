// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewCompactAlt)]
pub fn view_compact_alt(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewCompactAlt"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6C22,4.9,21.1,4,20,4z M11.5,16.5h-4v-4h4V16.5z M11.5,11.5h-4v-4h4V11.5z M16.5,16.5h-4v-4h4V16.5z M16.5,11.5h-4v-4h4V11.5z"/>
        </SvgIcon>
    }
}
