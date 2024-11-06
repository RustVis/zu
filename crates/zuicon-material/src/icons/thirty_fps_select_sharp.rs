// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ThirtyFpsSelectSharp)]
pub fn thirty_fps_select_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ThirtyFpsSelectSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,4v2h5v2H5v2h4v2H4v2h7V4H4z M13,4h7v10h-7V4z M18,6h-3v6h3V6z M5,22H3v-5h2V22z M9,22H7v-5h2V22z M13,22h-2v-5h2V22z M21,22h-6v-5h6V22z"/>
        </SvgIcon>
    }
}
