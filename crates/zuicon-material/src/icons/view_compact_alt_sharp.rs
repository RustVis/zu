// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewCompactAltSharp)]
pub fn view_compact_alt_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewCompactAltSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,4H2v16h20V4z M11.5,16.5h-4v-4h4V16.5z M11.5,11.5h-4v-4h4V11.5z M16.5,16.5h-4v-4h4V16.5z M16.5,11.5h-4v-4h4V11.5z"/>
        </SvgIcon>
    }
}
