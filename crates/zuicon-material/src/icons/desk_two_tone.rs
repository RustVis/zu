// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DeskTwoTone)]
pub fn desk_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DeskTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,6v12h2V8h10v10h2v-2h4v2h2V6H2z M20,14h-4v-2h4V14z M20,10h-4V8h4V10z"/>
        </SvgIcon>
    }
}
