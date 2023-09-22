// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MuseumSharp)]
pub fn museum_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MuseumSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,11V9L12,2L2,9v2h2v9H2v2h20v-2h-2v-9H22z M16,18h-2v-4l-2,3l-2-3v4H8v-7h2l2,3l2-3h2V18z"/>
        </SvgIcon>
    }
}
