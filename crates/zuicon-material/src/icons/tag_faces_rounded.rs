// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TagFacesRounded)]
pub fn tag_faces_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TagFacesRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M11.99,2C6.47,2,2,6.48,2,12s4.47,10,9.99,10C17.52,22,22,17.52,22,12S17.52,2,11.99,2z M8.5,8C9.33,8,10,8.67,10,9.5 S9.33,11,8.5,11S7,10.33,7,9.5S7.67,8,8.5,8z M16.75,14.75C15.8,16.39,14.03,17.5,12,17.5s-3.8-1.11-4.75-2.75 C7.06,14.42,7.31,14,7.69,14h8.62C16.7,14,16.94,14.42,16.75,14.75z M15.5,11c-0.83,0-1.5-0.67-1.5-1.5S14.67,8,15.5,8 S17,8.67,17,9.5S16.33,11,15.5,11z"/>
        </SvgIcon>
    }
}
