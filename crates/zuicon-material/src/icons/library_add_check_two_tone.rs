// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LibraryAddCheckTwoTone)]
pub fn library_add_check_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LibraryAddCheckTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8,16h12V4H8V16z M10.4,9.09l2.07,2.08L17.6,6L19,7.41L12.47,14L9,10.5L10.4,9.09z" opacity=".3"/><path d="M20,2H8C6.9,2,6,2.9,6,4v12c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M20,16H8V4h12V16z M12.47,14 L9,10.5l1.4-1.41l2.07,2.08L17.6,6L19,7.41L12.47,14z M4,20h14v2H4c-1.1,0-2-0.9-2-2V6h2V20z"/><path d="M0,0h24v24H0V0z" fill="none"/>
        </SvgIcon>
    }
}
