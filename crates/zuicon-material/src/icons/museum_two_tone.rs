// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MuseumTwoTone)]
pub fn museum_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MuseumTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6,20h12V9H6V20z M8,11h2l2,3l2-3h2v7h-2v-4l-2,3l-2-3v4H8V11z" opacity=".3"/><path d="M22,11V9L12,2L2,9v2h2v9H2v2h20v-2h-2v-9H22z M18,20H6V9h12V20z"/>
        </SvgIcon>
    }
}
