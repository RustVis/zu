// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PianoSharp)]
pub fn piano_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PianoSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,3H3v18h18V3z M13,14.5h1.25V19h-4.5v-4.5H11V5h2V14.5z M5,5h2v9.5h1.25V19H5V5z M19,19h-3.25v-4.5H17V5h2V19z"/>
        </SvgIcon>
    }
}
